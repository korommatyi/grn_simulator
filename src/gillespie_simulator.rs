use crate::simulation::{System};

// This implementation uses the notations of the following publication:
// doi: 10.1146/annurev.physchem.58.032806.104637

struct GillespieSimulator {
    system: System,
    random: fn() -> f64
}

impl GillespieSimulator {
    pub fn step(&mut self) {
        // a_0 is the sum of propensities in the source paper
        let a: Vec<f64> = self.system.reactions.iter().map(|x| x.propensity(&self.system.state)).collect();
        let a_0: f64 = a.iter().sum();

        let r_1 = (self.random)();
        let r_2 = (self.random)();

        let tau = (1.0/r_1).ln()/a_0;

        let mut j: usize = self.system.reactions.len()-1;
        let target_value = r_2*a_0;

        let mut running_sum = 0.0;
        for (index,propensity) in a.iter().enumerate() {
            running_sum += propensity;
            if running_sum > target_value {
                j = index;
                break;
            }
        }
        let j = j;  // don't allow mutation after this point

        self.system.trigger_reaction(self.system.time_of_last_reaction+tau, j);
    }
}

#[test]
pub fn test_step_r_1_and_r_2_are_zero() {
    use crate::simulation::{System, Reactant, Product, Reaction};

    let mut sim = GillespieSimulator{
        system: System{
            state: vec![2u64, 2u64, 2u64],
            idx_to_name: vec!["o2".to_string(), "h2".to_string(), "h2o".to_string()],
            name_to_idx: [("o2".to_string(), 0), ("h2".to_string(), 1), ("h2o".to_string(), 2)].iter().cloned().collect(),
            reactions: vec![Reaction{
                reaction_parameter: 0.1,
                reactants: vec![Reactant{index: 0, quantity:1}, Reactant{index: 1, quantity: 2}],
                products: vec![Product{index: 2, quantity: 2}]}],
            time_of_last_reaction: 0.0,
            last_reaction: 1000
        },
        random: || {0.0}
    };

    sim.step();
}

#[test]
pub fn test_step_r_1_and_r_2_are_one() {
    use crate::simulation::{System, Reactant, Product, Reaction};

    let mut sim = GillespieSimulator{
        system: System{
            state: vec![2u64, 2u64, 2u64],
            idx_to_name: vec!["o2".to_string(), "h2".to_string(), "h2o".to_string()],
            name_to_idx: [("o2".to_string(), 0), ("h2".to_string(), 1), ("h2o".to_string(), 2)].iter().cloned().collect(),
            reactions: vec![Reaction{
                reaction_parameter: 0.1,
                reactants: vec![Reactant{index: 0, quantity:1}, Reactant{index: 1, quantity: 2}],
                products: vec![Product{index: 2, quantity: 2}]}],
            time_of_last_reaction: 0.0,
            last_reaction: 1000
        },
        random: || {1.0}
    };

    sim.step();
}