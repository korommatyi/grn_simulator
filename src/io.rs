use yaml_rust::{Yaml,YamlLoader};
use std::path::PathBuf;
use std::fs;
use std::collections::HashMap;
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