use scopewright_kernel::{DomainError, FactoryRunRequest};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum TaskKind {
    Observe,
    Isolate,
    Failover,
    Verify,
    Restore,
    Communicate,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecoveryTask {
    pub id: String,
    pub kind: TaskKind,
    pub duration_minutes: u32,
    pub dependencies: Vec<String>,
    pub required_roles: Vec<String>,
    pub exclusive_resource: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ScheduledTask {
    pub id: String,
    pub kind: TaskKind,
    pub start_minute: u32,
    pub finish_minute: u32,
    pub lane: usize,
    pub required_roles: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecoveryPlan {
    pub tasks: Vec<ScheduledTask>,
    pub critical_path_minutes: u32,
    pub total_work_minutes: u32,
    pub maximum_parallelism: usize,
    pub objective_minutes: u32,
    pub slack_minutes: i64,
}

#[derive(Debug, Clone, Default)]
struct Lane {
    available_at: u32,
    resources: BTreeSet<String>,
}

pub fn validate_tasks(tasks: &[RecoveryTask]) -> Result<(), DomainError> {
    if tasks.is_empty() {
        return Err(DomainError::Missing("recovery.tasks"));
    }
    let ids: BTreeSet<_> = tasks.iter().map(|task| task.id.as_str()).collect();
    if ids.len() != tasks.len() {
        return Err(DomainError::Integrity(
            "recovery task identifiers must be unique".into(),
        ));
    }
    for task in tasks {
        if task.id.trim().is_empty() {
            return Err(DomainError::Missing("recovery.task.id"));
        }
        if task.duration_minutes == 0 {
            return Err(DomainError::Range("recovery.task.duration"));
        }
        if task.required_roles.is_empty() {
            return Err(DomainError::Missing("recovery.task.roles"));
        }
        for dependency in &task.dependencies {
            if dependency == &task.id {
                return Err(DomainError::Integrity(format!(
                    "recovery task {} depends on itself",
                    task.id
                )));
            }
            if !ids.contains(dependency.as_str()) {
                return Err(DomainError::Integrity(format!(
                    "recovery task {} references unknown dependency {}",
                    task.id, dependency
                )));
            }
        }
    }
    Ok(())
}

fn topological_order(tasks: &[RecoveryTask]) -> Result<Vec<String>, DomainError> {
    validate_tasks(tasks)?;
    let mut incoming: BTreeMap<String, usize> = tasks
        .iter()
        .map(|task| (task.id.clone(), task.dependencies.len()))
        .collect();
    let mut downstream: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for task in tasks {
        for dependency in &task.dependencies {
            downstream
                .entry(dependency.clone())
                .or_default()
                .push(task.id.clone());
        }
    }
    for dependents in downstream.values_mut() {
        dependents.sort();
    }
    let mut ready: VecDeque<String> = incoming
        .iter()
        .filter(|(_, count)| **count == 0)
        .map(|(id, _)| id.clone())
        .collect();
    let mut order = Vec::new();
    while let Some(id) = ready.pop_front() {
        order.push(id.clone());
        if let Some(dependents) = downstream.get(&id) {
            for dependent in dependents {
                let count = incoming
                    .get_mut(dependent)
                    .ok_or_else(|| DomainError::Integrity("dependency graph changed".into()))?;
                *count = count.saturating_sub(1);
                if *count == 0 {
                    ready.push_back(dependent.clone());
                }
            }
        }
    }
    if order.len() != tasks.len() {
        return Err(DomainError::Integrity(
            "recovery task dependency cycle".into(),
        ));
    }
    Ok(order)
}

fn choose_lane(lanes: &[Lane], earliest: u32, exclusive_resource: Option<&str>) -> (usize, u32) {
    lanes
        .iter()
        .enumerate()
        .map(|(index, lane)| {
            let resource_delay = exclusive_resource
                .filter(|resource| lane.resources.contains(*resource))
                .map(|_| lane.available_at)
                .unwrap_or_default();
            (index, earliest.max(lane.available_at).max(resource_delay))
        })
        .min_by_key(|(index, start)| (*start, *index))
        .unwrap_or((0, earliest))
}

pub fn schedule_tasks(
    tasks: &[RecoveryTask],
    lane_count: usize,
    objective_minutes: u32,
) -> Result<RecoveryPlan, DomainError> {
    validate_tasks(tasks)?;
    if lane_count == 0 {
        return Err(DomainError::Range("recovery.lanes"));
    }
    if objective_minutes == 0 {
        return Err(DomainError::Range("recovery.objective"));
    }
    let order = topological_order(tasks)?;
    let by_id: BTreeMap<_, _> = tasks.iter().map(|task| (task.id.as_str(), task)).collect();
    let mut lanes = vec![Lane::default(); lane_count];
    let mut finished = BTreeMap::new();
    let mut scheduled = Vec::with_capacity(tasks.len());

    for id in order {
        let task = by_id
            .get(id.as_str())
            .copied()
            .ok_or_else(|| DomainError::Integrity("ordered task disappeared".into()))?;
        let dependency_finish = task
            .dependencies
            .iter()
            .filter_map(|dependency| finished.get(dependency).copied())
            .max()
            .unwrap_or_default();
        let (lane_index, start_minute) = choose_lane(
            &lanes,
            dependency_finish,
            task.exclusive_resource.as_deref(),
        );
        let finish_minute = start_minute
            .checked_add(task.duration_minutes)
            .ok_or(DomainError::Range("recovery.finish"))?;
        let lane = &mut lanes[lane_index];
        lane.available_at = finish_minute;
        if let Some(resource) = &task.exclusive_resource {
            lane.resources.insert(resource.clone());
        }
        finished.insert(task.id.clone(), finish_minute);
        scheduled.push(ScheduledTask {
            id: task.id.clone(),
            kind: task.kind,
            start_minute,
            finish_minute,
            lane: lane_index,
            required_roles: task.required_roles.clone(),
        });
    }

    scheduled.sort_by_key(|task| (task.start_minute, task.lane, task.id.clone()));
    let critical_path_minutes = scheduled
        .iter()
        .map(|task| task.finish_minute)
        .max()
        .unwrap_or_default();
    let total_work_minutes = tasks.iter().map(|task| task.duration_minutes).sum();
    let maximum_parallelism = (0..=critical_path_minutes)
        .map(|minute| {
            scheduled
                .iter()
                .filter(|task| task.start_minute <= minute && minute < task.finish_minute)
                .count()
        })
        .max()
        .unwrap_or_default();
    Ok(RecoveryPlan {
        tasks: scheduled,
        critical_path_minutes,
        total_work_minutes,
        maximum_parallelism,
        objective_minutes,
        slack_minutes: objective_minutes as i64 - critical_path_minutes as i64,
    })
}

pub fn tasks_for_request(request: &FactoryRunRequest) -> Result<Vec<RecoveryTask>, DomainError> {
    request.validate_identity()?;
    let trace_summary = request.trace_summary.to_ascii_lowercase();
    let isolation = (request.scope_pressure as u32 / 12).max(3);
    let verification = (request.token_budget / 8).max(3);
    let restore = (request.token_budget / 3).max(5);
    let data_task = if trace_summary.contains("database") || trace_summary.contains("data") {
        RecoveryTask {
            id: "verify-durable-evidence".into(),
            kind: TaskKind::Verify,
            duration_minutes: verification + 4,
            dependencies: vec!["isolate-fault".into()],
            required_roles: vec!["data-owner".into(), "auditor".into()],
            exclusive_resource: Some("evidence-store".into()),
        }
    } else {
        RecoveryTask {
            id: "verify-service-state".into(),
            kind: TaskKind::Verify,
            duration_minutes: verification,
            dependencies: vec!["isolate-fault".into()],
            required_roles: vec!["service-owner".into()],
            exclusive_resource: Some("service-control".into()),
        }
    };
    let verify_id = data_task.id.clone();
    Ok(vec![
        RecoveryTask {
            id: "open-observation".into(),
            kind: TaskKind::Observe,
            duration_minutes: 3,
            dependencies: vec![],
            required_roles: vec!["observer".into()],
            exclusive_resource: None,
        },
        RecoveryTask {
            id: "notify-operators".into(),
            kind: TaskKind::Communicate,
            duration_minutes: 2,
            dependencies: vec!["open-observation".into()],
            required_roles: vec!["incident-commander".into()],
            exclusive_resource: Some("operator-channel".into()),
        },
        RecoveryTask {
            id: "isolate-fault".into(),
            kind: TaskKind::Isolate,
            duration_minutes: isolation,
            dependencies: vec!["open-observation".into()],
            required_roles: vec!["service-owner".into(), "policy-owner".into()],
            exclusive_resource: Some("service-control".into()),
        },
        RecoveryTask {
            id: "failover-path".into(),
            kind: TaskKind::Failover,
            duration_minutes: (request.token_budget / 5).max(4),
            dependencies: vec!["isolate-fault".into()],
            required_roles: vec!["platform-owner".into()],
            exclusive_resource: Some("service-control".into()),
        },
        data_task,
        RecoveryTask {
            id: "restore-primary".into(),
            kind: TaskKind::Restore,
            duration_minutes: restore,
            dependencies: vec!["failover-path".into(), verify_id],
            required_roles: vec!["service-owner".into(), "observer".into()],
            exclusive_resource: Some("service-control".into()),
        },
        RecoveryTask {
            id: "close-audit".into(),
            kind: TaskKind::Verify,
            duration_minutes: verification,
            dependencies: vec!["restore-primary".into(), "notify-operators".into()],
            required_roles: vec!["auditor".into(), "incident-commander".into()],
            exclusive_resource: Some("evidence-store".into()),
        },
    ])
}

pub fn plan_request(request: &FactoryRunRequest) -> Result<RecoveryPlan, DomainError> {
    schedule_tasks(&tasks_for_request(request)?, 3, request.token_budget)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(trace_summary: &str, token_budget: u32) -> FactoryRunRequest {
        FactoryRunRequest {
            service: "billing-edge".into(),
            trace_summary: trace_summary.into(),
            scope_pressure: 58,
            token_budget,
            operator: "incident-commander".into(),
        }
    }

    #[test]
    fn plan_honors_dependencies_and_objective() {
        let plan = plan_request(&request("database failover", 60)).unwrap();
        let finish: BTreeMap<_, _> = plan
            .tasks
            .iter()
            .map(|task| (task.id.as_str(), task.finish_minute))
            .collect();
        let restore = plan
            .tasks
            .iter()
            .find(|task| task.id == "restore-primary")
            .unwrap();
        assert!(restore.start_minute >= finish["failover-path"]);
        assert!(restore.start_minute >= finish["verify-durable-evidence"]);
        assert!(plan.slack_minutes >= 0);
    }

    #[test]
    fn scheduler_uses_parallel_lanes() {
        let plan = plan_request(&request("queue partition", 50)).unwrap();
        assert!(plan.maximum_parallelism >= 2);
        assert!(plan.critical_path_minutes < plan.total_work_minutes);
    }

    #[test]
    fn data_trace_summary_requires_auditor_and_data_owner() {
        let tasks = tasks_for_request(&request("database corruption", 60)).unwrap();
        let verify = tasks
            .iter()
            .find(|task| task.id == "verify-durable-evidence")
            .unwrap();
        assert!(verify.required_roles.contains(&"auditor".to_string()));
        assert!(verify.required_roles.contains(&"data-owner".to_string()));
    }

    #[test]
    fn rejects_unknown_dependency() {
        let mut tasks = tasks_for_request(&request("cache loss", 60)).unwrap();
        tasks[0].dependencies.push("missing".into());
        assert!(
            matches!(validate_tasks(&tasks), Err(DomainError::Integrity(message)) if message.contains("unknown"))
        );
    }

    #[test]
    fn rejects_dependency_cycle() {
        let mut tasks = tasks_for_request(&request("cache loss", 60)).unwrap();
        tasks[0].dependencies.push("close-audit".into());
        assert!(
            matches!(schedule_tasks(&tasks, 2, 60), Err(DomainError::Integrity(message)) if message.contains("cycle"))
        );
    }

    #[test]
    fn shorter_objective_reports_negative_slack() {
        let plan = plan_request(&request("regional queue loss", 12)).unwrap();
        assert!(plan.slack_minutes < 0);
    }
}
