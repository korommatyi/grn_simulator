use rand::{Rng, SeedableRng};
use std::fs::File;
use std::path::PathBuf;
use structopt::StructOpt;

mod gillespie_simulator;
mod io;
mod output_formatter;
mod system;

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
        help = "Where to write the output. The output is a csv file with the format described on \
                https://github.com/korommatyi/grn_simulator/wiki."
    )]
    output: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    let system = io::load_system(&opt.reactions, &opt.initial_state);
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(opt.seed);
    let algorithm = gillespie_simulator::GillespieSimulator {
        system: &system,
        random: || rng.gen::<f64>(),
    };
    let output = output_formatter::CSVFormatter {
        system: &system,
        output: File::create(&opt.output).expect("Cannot create output file."),
    };
    // create simulator from cli args
    // call simulator

    println!("System:");
    println!("{:?}", system);
}
