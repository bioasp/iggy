use std::path::PathBuf;
use structopt::StructOpt;
use std::str::FromStr;
use failure::*;
use iggy::*;

/// Opt-graph confronts interaction graph models with observations of (signed) changes between
/// two measured states.
/// Opt-graph computes networks fitting the observation data by removing (or adding) a minimal
/// number of edges in the given network
 
#[derive(StructOpt, Debug)]
#[structopt(name = "Iggy")]
struct Opt {
    /// Influence graph in NSSIF format
    #[structopt(short = "n", long = "network", parse(from_os_str))]
    networkfile: PathBuf,

    /// Directory of observations in bioquali format
    #[structopt(short = "o", long = "observations", parse(from_os_str))]
    observationfile: PathBuf,

    /// Disable forward propagation constraints
    #[structopt(long = "fwd_propagation_off", conflicts_with = "depmat")]
    fwd_propagation_off: bool,

    /// Disable foundedness constraints
    #[structopt(long = "founded_constraints_off", conflicts_with = "depmat")]
    founded_constraints_off: bool,

    /// Every change must be explained by an elementary path from an input
    #[structopt(long = "elempath")]
    elempath: bool,

    /// Combine multiple states, a change must be explained by an elementary path from an input
    #[structopt(long = "depmat")]
    depmat: bool,

    /// Declare nodes with indegree 0 as inputs
    #[structopt(short = "a", long = "autoinputs")]
    autoinputs: bool,

    /// Show N repairs to print, default is OFF, 0=all
    #[structopt(short = "r", long = "show_repairs")]
    show_repairs: Option<u32>,

    /// Repair mode: add = add edges (default),
    ///              optgraph = add + remove edges,
    ///              flip = flip edges
    #[structopt(short = "m", long = "repair_mode")]
    repair_mode: RepairMode,
}

#[derive(Debug)]
enum RepairMode {
    Add,
    OptGraph,
    Flip,
}
#[derive(Debug, Fail)]
#[fail(display = "ParseRepairModeError: {}", msg)]
pub struct ParseRepairModeError {
    pub msg: &'static str,
}
impl ParseRepairModeError {
    fn new(msg: &'static str) -> ParseRepairModeError {
        ParseRepairModeError { msg }
    }
}
impl FromStr for RepairMode {
    type Err = ParseRepairModeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
           "add" => Ok(RepairMode::Add),
           "optgraph" => Ok(RepairMode::OptGraph),
           "flip" => Ok(RepairMode::Flip),
           _ =>  Err(ParseRepairModeError::new("failed to parse repair mode. Possible values are: add, optgraph and flip.")),
        }
    }
}

fn main() {

    let opt = Opt::from_args();
    let setting = get_setting(&opt);
}

fn get_setting(opt: &Opt) -> SETTING {
    println!("_____________________________________________________________________\n");
    let setting = if opt.depmat {
        println!(" + DepMat combines multiple states.");
        println!(" + An elementary path from an input must exist to explain changes.");
        SETTING {
            os: false,
            ep: true,
            fp: true,
            fc: true,
        }
    } else {
        println!(" + All observed changes must be explained by an predecessor.");
        SETTING {
            os: true,
            ep: if opt.elempath {
                println!(" + An elementary path from an input must exist to explain changes.");
                true
            } else {
                false
            },
            fp: if opt.fwd_propagation_off {
                false
            } else {
                println!(" + 0-change must be explained.");
                true
            },
            fc: if opt.founded_constraints_off {
                false
            } else {
                println!(" + All observed changes must be explained by an input.");
                true
            },
        }
    };
    println!("_____________________________________________________________________\n");
    setting
}