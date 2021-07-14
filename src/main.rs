use std::path::PathBuf;
use structopt::StructOpt;

mod gillespie_simulator;
mod io;
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
        help = "Where the write the output. The output is a csv file with the format described on \
                https://github.com/korommatyi/grn_simulator/wiki."
    )]
    output: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    let system = io::load_system(&opt.reactions, &opt.initial_state);

    println!("System:");
    println!("{:?}", system);
}
