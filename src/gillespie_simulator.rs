use crate::system::System;

// This implementation uses the notations of the following publication:
// doi: 10.1146/annurev.physchem.58.032806.104637

pub struct GillespieSimulator<'a, F: Fn() -> f64> {
    system: &'a System,
    random: F,
}

impl<'a, F: Fn() -> f64> GillespieSimulator<'a, F> {
    pub fn step(&mut self) {
        let a: Vec<f64> = self
            .system
            .reactions
            .iter()
            .map(|x| x.propensity(&self.system.state))
            .collect();
        let a_0: f64 = a.iter().sum();

        let r_1 = (self.random)();
        let r_2 = (self.random)();

        let tau = (1.0 / r_1).ln() / a_0;

        let mut j: usize = self.system.reactions.len() - 1;
        let target_value = r_2 * a_0;

        let mut running_sum = 0.0;
        for (index, propensity) in a.iter().enumerate() {
            running_sum += propensity;
            if running_sum > target_value {
                j = index;
                break;
            }
        }
        let j = j; // don't allow mutation after this point

        self.system
            .trigger_reaction(self.system.time_of_last_reaction + tau, j);
    }
}

#[cfg(test)]
fn create_default_system() -> System {
    use crate::system::{Product, Reactant, Reaction};
    return System {
        state: vec![2u64, 2u64, 2u64],
        idx_to_name: vec!["o2".to_string(), "h2".to_string(), "h2o".to_string()],
        name_to_idx: [
            ("o2".to_string(), 0),
            ("h2".to_string(), 1),
            ("h2o".to_string(), 2),
        ]
        .iter()
        .cloned()
        .collect(),
        reactions: vec![
            Reaction {
                reaction_parameter: 0.1,
                reactants: vec![
                    Reactant {
                        index: 0,
                        quantity: 1,
                    },
                    Reactant {
                        index: 1,
                        quantity: 2,
                    },
                ],
                products: vec![Product {
                    index: 2,
                    quantity: 2,
                }],
            },
            Reaction {
                reaction_parameter: 0.01,
                reactants: vec![Reactant {
                    index: 2,
                    quantity: 2,
                }],
                products: vec![
                    Product {
                        index: 0,
                        quantity: 1,
                    },
                    Product {
                        index: 1,
                        quantity: 2,
                    },
                ],
            },
        ],
        time_of_last_reaction: 0.0,
        last_reaction: 1000,
    };
}

#[test]
pub fn test_step_r_1_and_r_2_are_zero() {
    let mut sim = GillespieSimulator {
        system: &create_default_system(),
        random: || 0.0,
    };

    sim.step();

    assert_eq!(sim.system.time_of_last_reaction, std::f64::INFINITY);
    assert_eq!(sim.system.last_reaction, 0);
    assert_eq!(sim.system.state, vec![1, 0, 4]);
}

#[test]
pub fn test_step_r_1_and_r_2_are_one() {
    let mut sim = GillespieSimulator {
        system: &create_default_system(),
        random: || 1.0,
    };

    sim.step();

    assert_eq!(sim.system.time_of_last_reaction, 0.0);
    assert_eq!(sim.system.last_reaction, 1);
    assert_eq!(sim.system.state, vec![3, 4, 0]);
}

#[test]
pub fn test_decision_boundary() {
    let system = create_default_system();
    let condensation = system.reactions.get(0).unwrap();
    let hydrolysis = system.reactions.get(1).unwrap();
    let decision_boundary: f64 = condensation.propensity(&system.state)
        / (condensation.propensity(&system.state) + hydrolysis.propensity(&system.state));

    let mut sim = GillespieSimulator {
        system: &create_default_system(),
        random: || -> f64 { decision_boundary - std::f64::EPSILON },
    };
    sim.step();
    assert_eq!(sim.system.last_reaction, 0);

    let mut sim = GillespieSimulator {
        system: &create_default_system(),
        random: || -> f64 { decision_boundary + std::f64::EPSILON },
    };
    sim.step();
    assert_eq!(sim.system.last_reaction, 1);
}
