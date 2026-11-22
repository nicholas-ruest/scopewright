use scopewright_kernel::{DomainError, FactoryRunRequest};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskBand {
    Guarded,
    Elevated,
    High,
    Critical,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum FactorKind {
    BlastRadius,
    RecoveryTime,
    DataIntegrity,
    Coordination,
    PolicyExposure,
    DependencyDepth,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RiskFactor {
    pub kind: FactorKind,
    pub likelihood: f64,
    pub consequence: f64,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Control {
    pub name: String,
    pub applies_to: Vec<FactorKind>,
    pub effectiveness: f64,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FactorContribution {
    pub kind: FactorKind,
    pub inherent: f64,
    pub residual: f64,
    pub reduction: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RiskAssessment {
    pub inherent_score: f64,
    pub residual_score: f64,
    pub uncertainty: f64,
    pub band: RiskBand,
    pub contributions: Vec<FactorContribution>,
    pub dominant_factor: FactorKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SensitivityPoint {
    pub control_name: String,
    pub without_control: f64,
    pub delta: f64,
}

fn validate_unit(value: f64, field: &'static str) -> Result<(), DomainError> {
    if !value.is_finite() || !(0.0..=1.0).contains(&value) {
        return Err(DomainError::Range(field));
    }
    Ok(())
}

impl RiskFactor {
    pub fn validate(&self) -> Result<(), DomainError> {
        validate_unit(self.likelihood, "risk.likelihood")?;
        validate_unit(self.consequence, "risk.consequence")?;
        validate_unit(self.confidence, "risk.confidence")
    }

    fn inherent(&self) -> f64 {
        self.likelihood * self.consequence * 100.0
    }
}

impl Control {
    pub fn validate(&self) -> Result<(), DomainError> {
        if self.name.trim().is_empty() {
            return Err(DomainError::Missing("control.name"));
        }
        if self.applies_to.is_empty() {
            return Err(DomainError::Missing("control.applies_to"));
        }
        validate_unit(self.effectiveness, "control.effectiveness")?;
        validate_unit(self.confidence, "control.confidence")
    }

    fn reduction_for(&self, kind: FactorKind) -> f64 {
        if self.applies_to.contains(&kind) {
            self.effectiveness * self.confidence
        } else {
            0.0
        }
    }
}

pub fn assess(factors: &[RiskFactor], controls: &[Control]) -> Result<RiskAssessment, DomainError> {
    if factors.is_empty() {
        return Err(DomainError::Missing("risk.factors"));
    }
    for factor in factors {
        factor.validate()?;
    }
    for control in controls {
        control.validate()?;
    }

    let mut contributions = Vec::with_capacity(factors.len());
    let mut inherent_weight = 0.0_f64;
    let mut residual_weight = 0.0_f64;
    let mut confidence_weight = 0.0_f64;

    for factor in factors {
        let inherent = factor.inherent();
        let retained = controls.iter().fold(1.0_f64, |remaining, control| {
            remaining * (1.0 - control.reduction_for(factor.kind)).clamp(0.0, 1.0)
        });
        let residual = inherent * retained;
        let calibrated_inherent = inherent * factor.confidence;
        let calibrated_residual = residual * factor.confidence;
        inherent_weight += calibrated_inherent;
        residual_weight += calibrated_residual;
        confidence_weight += factor.confidence;
        contributions.push(FactorContribution {
            kind: factor.kind,
            inherent,
            residual,
            reduction: inherent - residual,
        });
    }

    let denominator = confidence_weight.max(f64::EPSILON);
    let inherent_score = (inherent_weight / denominator).clamp(0.0, 100.0);
    let residual_score = (residual_weight / denominator).clamp(0.0, 100.0);
    let uncertainty = factors
        .iter()
        .map(|factor| (1.0 - factor.confidence) * factor.inherent())
        .sum::<f64>()
        / factors.len() as f64;
    contributions.sort_by(|left, right| {
        right
            .residual
            .total_cmp(&left.residual)
            .then_with(|| format!("{:?}", left.kind).cmp(&format!("{:?}", right.kind)))
    });
    let dominant_factor = contributions
        .first()
        .map(|contribution| contribution.kind)
        .ok_or(DomainError::Missing("risk.contributions"))?;
    let band = match residual_score {
        score if score < 20.0 => RiskBand::Guarded,
        score if score < 45.0 => RiskBand::Elevated,
        score if score < 70.0 => RiskBand::High,
        _ => RiskBand::Critical,
    };

    Ok(RiskAssessment {
        inherent_score,
        residual_score,
        uncertainty,
        band,
        contributions,
        dominant_factor,
    })
}

pub fn sensitivity(
    factors: &[RiskFactor],
    controls: &[Control],
) -> Result<Vec<SensitivityPoint>, DomainError> {
    let baseline = assess(factors, controls)?.residual_score;
    let mut points = Vec::new();
    for (index, control) in controls.iter().enumerate() {
        let reduced: Vec<_> = controls
            .iter()
            .enumerate()
            .filter(|(candidate, _)| *candidate != index)
            .map(|(_, value)| value.clone())
            .collect();
        let without = assess(factors, &reduced)?.residual_score;
        points.push(SensitivityPoint {
            control_name: control.name.clone(),
            without_control: without,
            delta: without - baseline,
        });
    }
    points.sort_by(|left, right| {
        right
            .delta
            .total_cmp(&left.delta)
            .then_with(|| left.control_name.cmp(&right.control_name))
    });
    Ok(points)
}

pub fn stress_grid(
    factors: &[RiskFactor],
    controls: &[Control],
    likelihood_multipliers: &[f64],
    consequence_multipliers: &[f64],
) -> Result<Vec<RiskAssessment>, DomainError> {
    if likelihood_multipliers.is_empty() || consequence_multipliers.is_empty() {
        return Err(DomainError::Missing("risk.stress_grid"));
    }
    let mut assessments = Vec::new();
    for likelihood in likelihood_multipliers {
        if !likelihood.is_finite() || *likelihood <= 0.0 {
            return Err(DomainError::Range("risk.likelihood_multiplier"));
        }
        for consequence in consequence_multipliers {
            if !consequence.is_finite() || *consequence <= 0.0 {
                return Err(DomainError::Range("risk.consequence_multiplier"));
            }
            let stressed: Vec<_> = factors
                .iter()
                .map(|factor| RiskFactor {
                    kind: factor.kind,
                    likelihood: (factor.likelihood * likelihood).clamp(0.0, 1.0),
                    consequence: (factor.consequence * consequence).clamp(0.0, 1.0),
                    confidence: factor.confidence,
                })
                .collect();
            assessments.push(assess(&stressed, controls)?);
        }
    }
    Ok(assessments)
}

pub fn factors_for_request(request: &FactoryRunRequest) -> Result<Vec<RiskFactor>, DomainError> {
    request.validate_identity()?;
    let trace_summary = request.trace_summary.to_ascii_lowercase();
    let radius = request.scope_pressure as f64 / 100.0;
    let recovery = (request.token_budget as f64 / 180.0).clamp(0.05, 1.0);
    let data = if trace_summary.contains("data") || trace_summary.contains("database") {
        0.92
    } else {
        0.48
    };
    let coordination = if trace_summary.contains("region") || trace_summary.contains("multi") {
        0.78
    } else {
        0.42
    };
    let policy = if trace_summary.contains("policy") || trace_summary.contains("credential") {
        0.86
    } else {
        0.37
    };
    let depth = (request.trace_summary.split_whitespace().count() as f64 / 12.0).clamp(0.2, 1.0);
    Ok(vec![
        RiskFactor {
            kind: FactorKind::BlastRadius,
            likelihood: radius,
            consequence: (0.5 + radius / 2.0).clamp(0.0, 1.0),
            confidence: 0.96,
        },
        RiskFactor {
            kind: FactorKind::RecoveryTime,
            likelihood: recovery,
            consequence: 0.82,
            confidence: 0.91,
        },
        RiskFactor {
            kind: FactorKind::DataIntegrity,
            likelihood: data,
            consequence: 0.97,
            confidence: 0.88,
        },
        RiskFactor {
            kind: FactorKind::Coordination,
            likelihood: coordination,
            consequence: 0.68,
            confidence: 0.81,
        },
        RiskFactor {
            kind: FactorKind::PolicyExposure,
            likelihood: policy,
            consequence: 0.89,
            confidence: 0.93,
        },
        RiskFactor {
            kind: FactorKind::DependencyDepth,
            likelihood: depth,
            consequence: 0.71,
            confidence: 0.79,
        },
    ])
}

pub fn standard_controls(request: &FactoryRunRequest) -> Vec<Control> {
    let radius_effectiveness = if request.scope_pressure <= 50 {
        0.58
    } else {
        0.32
    };
    vec![
        Control {
            name: "policy-boundary".into(),
            applies_to: vec![FactorKind::BlastRadius, FactorKind::PolicyExposure],
            effectiveness: radius_effectiveness,
            confidence: 0.94,
        },
        Control {
            name: "durable-audit-chain".into(),
            applies_to: vec![FactorKind::DataIntegrity],
            effectiveness: 0.64,
            confidence: 0.97,
        },
        Control {
            name: "correlated-operator-telemetry".into(),
            applies_to: vec![FactorKind::Coordination, FactorKind::DependencyDepth],
            effectiveness: 0.43,
            confidence: 0.86,
        },
        Control {
            name: "recovery-window-budget".into(),
            applies_to: vec![FactorKind::RecoveryTime],
            effectiveness: 0.37,
            confidence: 0.9,
        },
    ]
}

pub fn assess_request(request: &FactoryRunRequest) -> Result<RiskAssessment, DomainError> {
    assess(&factors_for_request(request)?, &standard_controls(request))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn request(trace_summary: &str, radius: u8) -> FactoryRunRequest {
        FactoryRunRequest {
            service: "identity-edge".into(),
            trace_summary: trace_summary.into(),
            scope_pressure: radius,
            token_budget: 55,
            operator: "resilience-lead".into(),
        }
    }

    #[test]
    fn controls_reduce_residual_below_inherent() {
        let request = request("database corruption with policy failover", 64);
        let assessment = assess_request(&request).unwrap();
        assert!(assessment.residual_score < assessment.inherent_score);
        assert!(!assessment.contributions.is_empty());
    }

    #[test]
    fn data_trace_summary_makes_data_integrity_material() {
        let assessment = assess_request(&request("database page corruption", 45)).unwrap();
        let data = assessment
            .contributions
            .iter()
            .find(|item| item.kind == FactorKind::DataIntegrity)
            .unwrap();
        assert!(data.inherent > 80.0);
        assert!(data.reduction > 40.0);
    }

    #[test]
    fn sensitivity_ranks_controls_by_score_delta() {
        let request = request("regional database and policy loss", 70);
        let factors = factors_for_request(&request).unwrap();
        let controls = standard_controls(&request);
        let points = sensitivity(&factors, &controls).unwrap();
        assert_eq!(points.len(), controls.len());
        assert!(points.windows(2).all(|pair| pair[0].delta >= pair[1].delta));
    }

    #[test]
    fn stress_grid_is_cartesian_and_bounded() {
        let request = request("queue partition", 40);
        let grid = stress_grid(
            &factors_for_request(&request).unwrap(),
            &standard_controls(&request),
            &[0.8, 1.0, 1.2],
            &[0.9, 1.1],
        )
        .unwrap();
        assert_eq!(grid.len(), 6);
        assert!(grid
            .iter()
            .all(|item| (0.0..=100.0).contains(&item.residual_score)));
    }

    #[test]
    fn rejects_invalid_control_effectiveness() {
        let factors = factors_for_request(&request("queue partition", 40)).unwrap();
        let control = Control {
            name: "impossible".into(),
            applies_to: vec![FactorKind::BlastRadius],
            effectiveness: 1.4,
            confidence: 0.9,
        };
        assert!(matches!(
            assess(&factors, &[control]),
            Err(DomainError::Range("control.effectiveness"))
        ));
    }

    #[test]
    fn high_radius_is_not_hidden_by_controls() {
        let low = assess_request(&request("cache miss", 20)).unwrap();
        let high = assess_request(&request("cache miss", 79)).unwrap();
        assert!(high.residual_score > low.residual_score);
    }
}
