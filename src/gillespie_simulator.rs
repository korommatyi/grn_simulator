use crate::system::System;

// This implementation uses the notations of the following publication:
// doi: 10.1146/annurev.physchem.58.032806.104637

pub fn gillespie_step<F: FnMut() -> f64>(system: &mut System, random: &mut F) {
    let a: Vec<f64> = system
        .reactions
        .iter()
        .map(|x| x.propensity(&system.state))
        .collect();
    let a_0: f64 = a.iter().sum();

    let r_1 = (random)();
    let r_2 = (random)();

    let tau = (1.0 / r_1).ln() / a_0;

    let mut j: usize = system.reactions.len() - 1;
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

    system.trigger_reaction(system.time_of_last_reaction + tau, j);
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
    let mut system = create_default_system();

    gillespie_step(&mut system, &mut || 0.0);

    assert_eq!(system.time_of_last_reaction, std::f64::INFINITY);
    assert_eq!(system.last_reaction, 0);
    assert_eq!(system.state, vec![1, 0, 4]);
}

#[test]
pub fn test_step_r_1_and_r_2_are_one() {
    let mut system = create_default_system();

    gillespie_step(&mut system, &mut || 1.0);

    assert_eq!(system.time_of_last_reaction, 0.0);
    assert_eq!(system.last_reaction, 1);
    assert_eq!(system.state, vec![3, 4, 0]);
}

#[test]
pub fn test_decision_boundary() {
    let mut system1 = create_default_system();
    let condensation = system1.reactions.get(0).unwrap();
    let hydrolysis = system1.reactions.get(1).unwrap();
    let decision_boundary: f64 = condensation.propensity(&system1.state)
        / (condensation.propensity(&system1.state) + hydrolysis.propensity(&system1.state));

    gillespie_step(&mut system1, &mut || -> f64 {
        decision_boundary - std::f64::EPSILON
    });
    assert_eq!(system1.last_reaction, 0);

    let mut system2 = create_default_system();
    gillespie_step(&mut system2, &mut || -> f64 {
        decision_boundary + std::f64::EPSILON
    });
    assert_eq!(system2.last_reaction, 1);
}
