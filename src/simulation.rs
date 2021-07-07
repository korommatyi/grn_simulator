#[derive(Debug)]
pub struct Reactant {
    pub index: usize,
    pub quantity: u64,
}

#[derive(Debug)]
pub struct Product {
    pub index: usize,
    pub quantity: u64,
}

#[derive(Debug)]
pub struct Reaction {
    pub reaction_parameter: f64,
    pub reactants: Vec<Reactant>,
    pub products: Vec<Product>,
}

impl Reaction {
    pub fn propensity(&self, state: &Vec<u64>) -> f64 {
        let mut propensity = self.reaction_parameter;

        for reactant in &self.reactants {
            propensity *= binomial(state[reactant.index], reactant.quantity) as f64;
        }

        return propensity;
    }
}

fn binomial(n: u64, k: u64) -> u64 {
    let mut coeff = n;

    // assumption: n is small enough
    for i in 1..(k - 1) {
        coeff *= n - i;
        coeff /= i + 1;
    }
    return coeff;
}

#[derive(Debug)]
pub struct System {
    pub state: Vec<u64>,
    pub idx_to_name: Vec<String>,
    pub name_to_idx: std::collections::HashMap<String, usize>,
    pub reactions: Vec<Reaction>,
    pub time_of_last_reaction: f64,
    pub last_reaction: usize
}

impl System {
    pub fn new() -> System {
        System {
            state: Vec::new(),
            idx_to_name: Vec::new(),
            name_to_idx: std::collections::HashMap::new(),
            reactions: Vec::new(),
            time_of_last_reaction: 0.0,
            last_reaction: 0
        }
    }

    pub fn serialize_state(&self) -> String {
        self.state
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    pub fn trigger_reaction(&mut self, reaction_time: f64, reaction_idx: usize) {
        debug_assert!(self.time_of_last_reaction <= reaction_time);
        debug_assert!(reaction_idx < self.reactions.len());

        let reaction = self.reactions.get(reaction_idx).unwrap();
        for reactant in &reaction.reactants {
            debug_assert!(self.state[reactant.index] >= reactant.quantity);
            self.state[reactant.index] -= reactant.quantity;
        }
        for product in &reaction.products {
            debug_assert!(self.state[product.index] >= product.quantity);
            self.state[product.index] += product.quantity;
        }

        self.time_of_last_reaction = reaction_time;
        self.last_reaction = reaction_idx;
    }
}

#[test]
fn test_trigger_reaction_valid() {
    let mut system = System{
        state: vec![2u64, 2u64, 2u64],
        idx_to_name: vec!["o2".to_string(), "h2".to_string(), "h2o".to_string()],
        name_to_idx: [("o2".to_string(), 0), ("h2".to_string(), 1), ("h2o".to_string(), 2)].iter().cloned().collect(),
        reactions: vec![Reaction{
            reaction_parameter: 0.1,
            reactants: vec![Reactant{index: 0, quantity:1}, Reactant{index: 1, quantity: 2}],
            products: vec![Product{index: 2, quantity: 2}]}],
        time_of_last_reaction: 0.0,
        last_reaction: 1000
    };

    let reaction_time = 0.1;
    let reaction_index = 0;
    system.trigger_reaction(reaction_time, reaction_index);

    assert_eq!(system.last_reaction, reaction_index);
    assert_eq!(system.time_of_last_reaction, reaction_time);
    assert_eq!(system.state, vec![1,0,4]);
}

#[test]
#[should_panic]
fn test_trigger_reaction_not_enough_reactants() {
    // tries to trigger a reaction, for which we don't have enough input
    let mut system = System{
        state: vec![0u64, 0u64, 0u64],
        idx_to_name: vec!["o2".to_string(), "h2".to_string(), "h2o".to_string()],
        name_to_idx: [("o2".to_string(), 0), ("h2".to_string(), 1), ("h2o".to_string(), 2)].iter().cloned().collect(),
        reactions: vec![Reaction{
            reaction_parameter: 0.1,
            reactants: vec![Reactant{index: 0, quantity:1}, Reactant{index: 1, quantity: 2}],
            products: vec![Product{index: 2, quantity: 2}]}],
        time_of_last_reaction: 0.0,
        last_reaction: 1000
    };

    let reaction_time = 0.1;
    let reaction_index = 0;
    system.trigger_reaction(reaction_time, reaction_index);
}

#[test]
#[should_panic]
fn test_trigger_reaction_invalid_reaction_index() {
    // tries to trigger a reaction with an invalid index
    let mut system = System{
        state: vec![2u64, 2u64, 2u64],
        idx_to_name: vec!["o2".to_string(), "h2".to_string(), "h2o".to_string()],
        name_to_idx: [("o2".to_string(), 0), ("h2".to_string(), 1), ("h2o".to_string(), 2)].iter().cloned().collect(),
        reactions: vec![Reaction{
            reaction_parameter: 0.1,
            reactants: vec![Reactant{index: 0, quantity:1}, Reactant{index: 1, quantity: 2}],
            products: vec![Product{index: 2, quantity: 2}]}],
        time_of_last_reaction: 0.0,
        last_reaction: 1000
    };

    let reaction_time = 0.1;
    let reaction_index = 1;
    system.trigger_reaction(reaction_time, reaction_index);
}

#[test]
#[should_panic]
fn test_trigger_reaction_in_the_past() {
    // tries to trigger a reaction before the last recorded reaction
    let mut system = System{
        state: vec![2u64, 2u64, 2u64],
        idx_to_name: vec!["o2".to_string(), "h2".to_string(), "h2o".to_string()],
        name_to_idx: [("o2".to_string(), 0), ("h2".to_string(), 1), ("h2o".to_string(), 2)].iter().cloned().collect(),
        reactions: vec![Reaction{
            reaction_parameter: 0.1,
            reactants: vec![Reactant{index: 0, quantity:1}, Reactant{index: 1, quantity: 2}],
            products: vec![Product{index: 2, quantity: 2}]}],
        time_of_last_reaction: 50.0,
        last_reaction: 1000
    };

    let reaction_time = 0.1;
    let reaction_index = 0;
    system.trigger_reaction(reaction_time, reaction_index);
}

#[test]
fn test_serialize_state() {
    let mut system = System::new();
    system.state = vec![0u64, 1u64, 1000u64];
    let printed_line = system.serialize_state();
    assert_eq!(printed_line.as_str(), "0,1,1000");
}
