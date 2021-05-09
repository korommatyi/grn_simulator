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
}

impl System {
    pub fn new() -> System {
        System {
            state: Vec::new(),
            idx_to_name: Vec::new(),
            name_to_idx: std::collections::HashMap::new(),
            reactions: Vec::new(),
        }
    }

    pub fn serialize_state(&self) -> String {
        self.state
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }
}

#[test]
fn test_serialize_state() {
    let mut system = System::new();
    system.state = vec![0u64, 1u64, 1000u64];
    let printed_line = system.serialize_state();
    assert_eq!(printed_line.as_str(), "0,1,1000");
}
