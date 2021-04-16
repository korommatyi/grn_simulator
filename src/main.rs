use std::convert::From;
use std::fs;
use std::path::PathBuf;
use structopt::StructOpt;
use yaml_rust::{Yaml, YamlLoader};

#[derive(Debug, StructOpt)]
#[structopt(name = "GRN simulator", about = "Gene Regulatory Network Simulator")]
struct Opt {
    #[structopt(short, long)]
    debug: bool,

    #[structopt(
        short,
        long,
        default_value = "0",
        help = "Seed used for generating random numbers"
    )]
    seed: u64,

    #[structopt(
        short,
        long,
        parse(from_os_str),
        help = "Yaml file describing the reactions. \
                For the valid format see https://github.com/korommatyi/grn_simulator/wiki."
    )]
    reactions: PathBuf,

    #[structopt(
        short,
        long,
        parse(from_os_str),
        help = "Yaml file describing the initial state of the system. \
                For the valid format see https://github.com/korommatyi/grn_simulator/wiki."
    )]
    initial_state: PathBuf,

    #[structopt(
        short,
        long,
        parse(from_os_str),
        help = "Where the write the output. The output is a csv file with the format described on \
                https://github.com/korommatyi/grn_simulator/wiki."
    )]
    output: PathBuf,
}

#[derive(Debug)]
struct Specimen {
    name: String,
    quantity: u64,
}

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

type State = Vec<Specimen>;

#[derive(Debug)]
struct Reaction {
    reaction_parameter: f64,
    inputs: Vec<Specimen>,
    outputs: Vec<Specimen>,
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

type Reactions = Vec<Reaction>;

fn main() {
    let opt = Opt::from_args();

    let reactions_content: Reactions =
        YamlLoader::load_from_str(&fs::read_to_string(opt.reactions).unwrap())
            .unwrap()
            .into_iter()
            .nth(0)
            .unwrap()
            .into_vec()
            .unwrap()
            .into_iter()
            .map(|x| x.into())
            .collect();
    println!("{:?}", reactions_content);
}
