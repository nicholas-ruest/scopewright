use scopewright_kernel::{DomainError, FactoryRunRequest};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ComponentKind {
    EntryPoint,
    Service,
    DataStore,
    Queue,
    ControlPlane,
    ExternalDependency,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Component {
    pub id: String,
    pub kind: ComponentKind,
    pub criticality: u8,
    pub token_budget: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Dependency {
    pub from: String,
    pub to: String,
    pub propagation: f64,
    pub required: bool,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq)]
pub struct ServiceTopology {
    pub components: Vec<Component>,
    pub dependencies: Vec<Dependency>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Impact {
    pub component_id: String,
    pub probability: f64,
    pub depth: usize,
    pub path: Vec<String>,
    pub weighted_criticality: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TopologyAnalysis {
    pub origin: String,
    pub impacts: Vec<Impact>,
    pub maximum_weighted_impact: f64,
    pub affected_components: usize,
    pub longest_path: usize,
    pub recovery_burden_minutes: u32,
}

impl ServiceTopology {
    pub fn validate(&self) -> Result<(), DomainError> {
        if self.components.is_empty() {
            return Err(DomainError::Missing("topology.components"));
        }
        let mut ids = BTreeSet::new();
        for component in &self.components {
            if component.id.trim().is_empty() {
                return Err(DomainError::Missing("component.id"));
            }
            if !ids.insert(component.id.as_str()) {
                return Err(DomainError::Integrity(format!(
                    "duplicate topology component {}",
                    component.id
                )));
            }
            if !(1..=100).contains(&component.criticality) {
                return Err(DomainError::Range("component.criticality"));
            }
            if component.token_budget == 0 {
                return Err(DomainError::Range("component.token_budget"));
            }
        }
        for edge in &self.dependencies {
            if !ids.contains(edge.from.as_str()) || !ids.contains(edge.to.as_str()) {
                return Err(DomainError::Integrity(format!(
                    "dependency references unknown component {} -> {}",
                    edge.from, edge.to
                )));
            }
            if edge.from == edge.to {
                return Err(DomainError::Integrity(format!(
                    "self dependency is not permitted for {}",
                    edge.from
                )));
            }
            if !edge.propagation.is_finite() || !(0.0..=1.0).contains(&edge.propagation) {
                return Err(DomainError::Range("dependency.propagation"));
            }
        }
        Ok(())
    }

    pub fn component(&self, id: &str) -> Option<&Component> {
        self.components.iter().find(|component| component.id == id)
    }

    pub fn downstream(&self, id: &str) -> Vec<&Dependency> {
        let mut edges: Vec<_> = self
            .dependencies
            .iter()
            .filter(|dependency| dependency.from == id)
            .collect();
        edges.sort_by(|left, right| {
            right
                .propagation
                .total_cmp(&left.propagation)
                .then_with(|| left.to.cmp(&right.to))
        });
        edges
    }

    pub fn propagate(
        &self,
        origin: &str,
        maximum_depth: usize,
        probability_floor: f64,
    ) -> Result<TopologyAnalysis, DomainError> {
        self.validate()?;
        if self.component(origin).is_none() {
            return Err(DomainError::Missing("topology.origin"));
        }
        if maximum_depth == 0 {
            return Err(DomainError::Range("topology.maximum_depth"));
        }
        if !probability_floor.is_finite() || !(0.0..=1.0).contains(&probability_floor) {
            return Err(DomainError::Range("topology.probability_floor"));
        }
        let mut best_probability = BTreeMap::new();
        let mut best_path: BTreeMap<String, Vec<String>> = BTreeMap::new();
        let mut queue = VecDeque::new();
        best_probability.insert(origin.to_string(), 1.0_f64);
        best_path.insert(origin.to_string(), vec![origin.to_string()]);
        queue.push_back((origin.to_string(), 1.0_f64, 0_usize));
        while let Some((current, probability, depth)) = queue.pop_front() {
            if depth >= maximum_depth {
                continue;
            }
            for dependency in self.downstream(&current) {
                let required_multiplier = if dependency.required { 1.0 } else { 0.72 };
                let candidate = probability * dependency.propagation * required_multiplier;
                if candidate < probability_floor {
                    continue;
                }
                let previous = best_probability
                    .get(&dependency.to)
                    .copied()
                    .unwrap_or_default();
                if candidate <= previous {
                    continue;
                }
                let mut path = best_path
                    .get(&current)
                    .cloned()
                    .unwrap_or_else(|| vec![current.clone()]);
                if path.iter().any(|component| component == &dependency.to) {
                    continue;
                }
                path.push(dependency.to.clone());
                best_probability.insert(dependency.to.clone(), candidate);
                best_path.insert(dependency.to.clone(), path);
                queue.push_back((dependency.to.clone(), candidate, depth + 1));
            }
        }
        let mut impacts = Vec::new();
        for (component_id, probability) in best_probability {
            let component = self
                .component(&component_id)
                .ok_or_else(|| DomainError::Integrity("impact component disappeared".into()))?;
            let path = best_path.remove(&component_id).unwrap_or_default();
            let depth = path.len().saturating_sub(1);
            impacts.push(Impact {
                component_id,
                probability,
                depth,
                path,
                weighted_criticality: probability * component.criticality as f64,
            });
        }
        impacts.sort_by(|left, right| {
            right
                .weighted_criticality
                .total_cmp(&left.weighted_criticality)
                .then_with(|| left.component_id.cmp(&right.component_id))
        });
        let maximum_weighted_impact = impacts
            .iter()
            .map(|impact| impact.weighted_criticality)
            .fold(0.0_f64, f64::max);
        let longest_path = impacts.iter().map(|impact| impact.depth).max().unwrap_or(0);
        let recovery_burden_minutes = impacts
            .iter()
            .filter_map(|impact| {
                self.component(&impact.component_id).map(|component| {
                    (component.token_budget as f64 * impact.probability).ceil() as u32
                })
            })
            .sum();
        Ok(TopologyAnalysis {
            origin: origin.to_string(),
            affected_components: impacts.len(),
            impacts,
            maximum_weighted_impact,
            longest_path,
            recovery_burden_minutes,
        })
    }

    pub fn cut_candidates(&self, origin: &str) -> Result<Vec<(String, f64)>, DomainError> {
        let baseline = self.propagate(origin, self.components.len().max(1), 0.01)?;
        let baseline_total: f64 = baseline
            .impacts
            .iter()
            .map(|impact| impact.weighted_criticality)
            .sum();
        let mut candidates = Vec::new();
        for component in &self.components {
            if component.id == origin {
                continue;
            }
            let mut reduced = self.clone();
            reduced
                .dependencies
                .retain(|edge| edge.from != component.id && edge.to != component.id);
            let reduced_analysis =
                reduced.propagate(origin, reduced.components.len().max(1), 0.01)?;
            let reduced_total: f64 = reduced_analysis
                .impacts
                .iter()
                .map(|impact| impact.weighted_criticality)
                .sum();
            candidates.push((
                component.id.clone(),
                (baseline_total - reduced_total).max(0.0),
            ));
        }
        candidates.sort_by(|left, right| {
            right
                .1
                .total_cmp(&left.1)
                .then_with(|| left.0.cmp(&right.0))
        });
        Ok(candidates)
    }
}

pub fn topology_for_request(request: &FactoryRunRequest) -> Result<ServiceTopology, DomainError> {
    request.validate_identity()?;
    let trace_summary = request.trace_summary.to_ascii_lowercase();
    let service = request
        .service
        .trim()
        .to_ascii_lowercase()
        .replace(' ', "-");
    let recovery = request.token_budget.max(5);
    let data_weight = if trace_summary.contains("database") || trace_summary.contains("data") {
        0.94
    } else {
        0.76
    };
    let queue_weight = if trace_summary.contains("queue") || trace_summary.contains("stream") {
        0.91
    } else {
        0.68
    };
    let control_weight = if trace_summary.contains("policy") || trace_summary.contains("credential")
    {
        0.89
    } else {
        0.61
    };
    let topology = ServiceTopology {
        components: vec![
            Component {
                id: "entry".into(),
                kind: ComponentKind::EntryPoint,
                criticality: 72,
                token_budget: recovery / 6 + 1,
            },
            Component {
                id: service.clone(),
                kind: ComponentKind::Service,
                criticality: request.scope_pressure.max(25),
                token_budget: recovery / 3 + 1,
            },
            Component {
                id: "durable-evidence".into(),
                kind: ComponentKind::DataStore,
                criticality: 96,
                token_budget: recovery / 2 + 1,
            },
            Component {
                id: "event-stream".into(),
                kind: ComponentKind::Queue,
                criticality: 78,
                token_budget: recovery / 4 + 1,
            },
            Component {
                id: "policy-plane".into(),
                kind: ComponentKind::ControlPlane,
                criticality: 90,
                token_budget: recovery / 5 + 1,
            },
            Component {
                id: "operator-notify".into(),
                kind: ComponentKind::ExternalDependency,
                criticality: 64,
                token_budget: recovery / 6 + 1,
            },
        ],
        dependencies: vec![
            Dependency {
                from: service.clone(),
                to: "entry".into(),
                propagation: 0.82,
                required: true,
            },
            Dependency {
                from: service.clone(),
                to: "durable-evidence".into(),
                propagation: data_weight,
                required: true,
            },
            Dependency {
                from: service.clone(),
                to: "event-stream".into(),
                propagation: queue_weight,
                required: true,
            },
            Dependency {
                from: service.clone(),
                to: "policy-plane".into(),
                propagation: control_weight,
                required: true,
            },
            Dependency {
                from: "event-stream".into(),
                to: "operator-notify".into(),
                propagation: 0.74,
                required: false,
            },
            Dependency {
                from: "policy-plane".into(),
                to: "entry".into(),
                propagation: 0.58,
                required: true,
            },
            Dependency {
                from: "durable-evidence".into(),
                to: "operator-notify".into(),
                propagation: 0.44,
                required: false,
            },
        ],
    };
    topology.validate()?;
    Ok(topology)
}

pub fn analyze_request(request: &FactoryRunRequest) -> Result<TopologyAnalysis, DomainError> {
    let topology = topology_for_request(request)?;
    let origin = request
        .service
        .trim()
        .to_ascii_lowercase()
        .replace(' ', "-");
    topology.propagate(&origin, 6, 0.05)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn request(trace_summary: &str) -> FactoryRunRequest {
        FactoryRunRequest {
            service: "checkout-api".into(),
            trace_summary: trace_summary.into(),
            scope_pressure: 68,
            token_budget: 45,
            operator: "incident-commander".into(),
        }
    }

    #[test]
    fn propagation_prioritizes_durable_evidence_for_database_loss() {
        let analysis = analyze_request(&request("database loss and queue replay")).unwrap();
        assert!(analysis.affected_components >= 5);
        let durable = analysis
            .impacts
            .iter()
            .find(|impact| impact.component_id == "durable-evidence")
            .unwrap();
        assert!(durable.probability > 0.9);
        assert!(analysis.recovery_burden_minutes > 45);
    }

    #[test]
    fn alternate_paths_keep_only_strongest_probability() {
        let topology = topology_for_request(&request("policy outage")).unwrap();
        let analysis = topology.propagate("checkout-api", 8, 0.01).unwrap();
        let entry = analysis
            .impacts
            .iter()
            .find(|impact| impact.component_id == "entry")
            .unwrap();
        assert!(entry.probability >= 0.82);
    }

    #[test]
    fn cut_candidates_are_sorted_by_reduced_exposure() {
        let topology = topology_for_request(&request("event stream partition")).unwrap();
        let candidates = topology.cut_candidates("checkout-api").unwrap();
        assert!(candidates.windows(2).all(|pair| pair[0].1 >= pair[1].1));
    }

    #[test]
    fn rejects_unknown_edge_endpoint() {
        let mut topology = topology_for_request(&request("database loss")).unwrap();
        topology.dependencies.push(Dependency {
            from: "missing".into(),
            to: "entry".into(),
            propagation: 0.5,
            required: true,
        });
        assert!(
            matches!(topology.validate(), Err(DomainError::Integrity(message)) if message.contains("unknown"))
        );
    }

    #[test]
    fn cycles_do_not_loop_forever() {
        let mut topology = topology_for_request(&request("queue failure")).unwrap();
        topology.dependencies.push(Dependency {
            from: "operator-notify".into(),
            to: "checkout-api".into(),
            propagation: 0.9,
            required: true,
        });
        let analysis = topology.propagate("checkout-api", 20, 0.001).unwrap();
        assert!(analysis.longest_path < topology.components.len());
    }

    #[test]
    fn probability_floor_removes_weak_optional_paths() {
        let topology = topology_for_request(&request("minor latency")).unwrap();
        let strict = topology.propagate("checkout-api", 6, 0.7).unwrap();
        let permissive = topology.propagate("checkout-api", 6, 0.01).unwrap();
        assert!(strict.affected_components < permissive.affected_components);
    }
}
