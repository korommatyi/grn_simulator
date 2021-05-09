use yaml_rust::{Yaml,YamlLoader};
use std::path::PathBuf;
use std::fs;
use crate::simulation::{Specimen, Reaction, Reactions, State, System, RealReaction, Reactant, Product};

impl From<Yaml> for Specimen {
    fn from(item: Yaml) -> Self {
        let hash = item.as_hash().unwrap();
        let name = hash[&Yaml::String("name".to_string())]
            .as_str()
            .unwrap()
            .to_string();
        let quantity = hash[&Yaml::String("quantity".to_string())]
            .as_i64()
            .unwrap() as u64;
        Specimen { name, quantity }
    }
}

impl From<Yaml> for Reaction {
    fn from(item: Yaml) -> Self {
        let mut hash = item.into_hash().unwrap();
        let reaction_parameter = hash[&Yaml::String("reaction_parameter".to_string())]
            .as_f64()
            .unwrap();
        let inputs = hash
            .remove(&Yaml::String("inputs".to_string()))
            .unwrap()
            .into_vec()
            .unwrap()
            .into_iter()
            .map(|x| x.into())
            .collect();
        let outputs = hash
            .remove(&Yaml::String("outputs".to_string()))
            .unwrap()
            .into_vec()
            .unwrap()
            .into_iter()
            .map(|x| x.into())
            .collect();
        return Reaction {
            reaction_parameter,
            inputs,
            outputs,
        };
    }
}

pub fn load_reactions(filename: &PathBuf) -> Reactions {
    YamlLoader::load_from_str(&fs::read_to_string(filename).unwrap())
        .unwrap()
        .into_iter()
        .nth(0)
        .unwrap()
        .into_vec()
        .unwrap()
        .into_iter()
        .map(|x| x.into())
        .collect()
}



pub fn load_system(reactions_filename: &PathBuf, ini_state_filename: &PathBuf) -> System {
    let mut system = System::new();

    let name_quantity_tuples = YamlLoader::load_from_str(&fs::read_to_string(ini_state_filename).unwrap())
        .unwrap()
        .into_iter()
        .nth(0)
        .unwrap()
        .into_hash()
        .unwrap()
        .into_iter()
        .map(|(x, y)| (x.into_string().unwrap(), y.into_i64().unwrap() as u64));

    for (name, quantity) in name_quantity_tuples {
        system.name_to_idx.insert(name.clone(), system.idx_to_name.len());
        system.idx_to_name.push(name);
        system.state.push(quantity);
    }

    let reactions = YamlLoader::load_from_str(&fs::read_to_string(reactions_filename).unwrap())
        .unwrap()
        .into_iter()
        .nth(0)
        .unwrap()
        .into_vec()
        .unwrap();
    for reaction in reactions {
        let mut reactants: Vec<Reactant> = Vec::new();
        let mut products: Vec<Product> = Vec::new();

        let mut hash = reaction.into_hash().unwrap();
        let reaction_parameter = hash[&Yaml::String("reaction_parameter".to_string())]
            .as_f64()
            .unwrap();
        let inputs = hash
            .remove(&Yaml::String("inputs".to_string()))
            .unwrap()
            .into_vec()
            .unwrap();
        for input in inputs {
            let input_hash = input.as_hash().unwrap();
            let name = input_hash[&Yaml::String("name".to_string())]
                .as_str()
                .unwrap()
                .to_string();
            let quantity = input_hash[&Yaml::String("quantity".to_string())]
                .as_i64()
                .unwrap() as u64;
            reactants.push(
                Reactant{index: system.name_to_idx[&name], quantity});
        }
        let outputs = hash
            .remove(&Yaml::String("outputs".to_string()))
            .unwrap()
            .into_vec()
            .unwrap();
        for output in outputs {
            let output_hash = output.as_hash().unwrap();
            let name = output_hash[&Yaml::String("name".to_string())]
                .as_str()
                .unwrap()
                .to_string();
            let quantity = output_hash[&Yaml::String("quantity".to_string())]
                .as_i64()
                .unwrap() as u64;
            products.push(
                Product{index: system.name_to_idx[&name], quantity});
        }

        system.reactions.push(RealReaction{reaction_parameter, reactants, products});
    }

    return system;
}

fn print_state(system: &System) -> String {
    let mut line_str = String::new();
    for quantity in system.state.iter() {
        line_str.push_str(format!("{};", quantity).as_str());
    }

    return line_str;
}

#[test]
fn test_load_system_from_yaml() {
    let initial_state_file: PathBuf = "resources/test/initial_state.yaml".into();
    let reactions_file: PathBuf = "resources/test/reactions.yaml".into();

    let system = load_system(&reactions_file, &initial_state_file);

    let idx_of_o2 = *system.name_to_idx.get("O2").expect("no O2 in system?");
    let idx_of_h2 = *system.name_to_idx.get("H2").expect("no H2 in system?");
    let idx_of_h2o = *system.name_to_idx.get("H2O").expect("no H2O in system?");

    // idx - name mapping is consistent
    assert_eq!(system.idx_to_name[idx_of_o2], "O2");
    assert_eq!(system.idx_to_name[idx_of_h2], "H2");
    assert_eq!(system.idx_to_name[idx_of_h2o], "H2O");

    // reactions
    assert_eq!(system.reactions.len(), 1);
    let first_reaction = &system.reactions[0];
    assert_eq!(first_reaction.reaction_parameter, 0.2);
    assert_eq!(first_reaction.reactants.len(), 2);
    assert_eq!(first_reaction.products.len(), 1);
    assert_eq!(first_reaction.reactants[0].index, idx_of_o2);
    assert_eq!(first_reaction.reactants[0].quantity, 1);
    assert_eq!(first_reaction.reactants[1].index, idx_of_h2);
    assert_eq!(first_reaction.reactants[1].quantity, 2);
    assert_eq!(first_reaction.products[0].index, idx_of_h2o);
    assert_eq!(first_reaction.products[0].quantity, 2);

    // initial state
    assert_eq!(system.state.len(), 3);
    assert_eq!(system.state[idx_of_o2], 2);
    assert_eq!(system.state[idx_of_h2], 4);
    assert_eq!(system.state[idx_of_h2o], 0);
}

#[test]
fn test_print_state() {
    let mut system = System::new();
    system.state = vec![0u64, 1u64, 1000u64];
    let printed_line = print_state(&system);
    assert_eq!(printed_line.as_str(), "0;1;1000;");
}