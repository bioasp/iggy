# Installation


Clone the git repository:

	git clone https://github.com/bioasp/iggy.git


Compile:

	cargo build --release

The binaries can be found under `./target/release/`


# Usage

Typical usage is:

    $ iggy -n network.cif -o observation.obs -l 10 -p

For more options you can ask for help as follows:

    $ iggy -h
    iggy 2.0.0
    Sven Thiele <sthiele78@gmail.com>
    Iggy confronts interaction graph models with observations of (signed) changes between two measured states (including uncertain observations). Iggy discovers inconsistencies in networks or data, applies minimal repairs, and predicts the behavior for the unmeasured species. It distinguishes strong predictions (e.g. increase in a node) and weak predictions (e.g., the value of a node increases or remains unchanged).

    USAGE:
        iggy [FLAGS] [OPTIONS] --network <networkfile> --observations <observationfile>

    FLAGS:
        -a, --autoinputs                Declare nodes with indegree 0 as inputs
            --depmat                    Combine multiple states, a change must be explained by an elementary path from an
                                        input
            --elempath                  Every change must be explained by an elementary path from an input
            --founded_constraints_off    Disable foundedness constraints
            --fwd_propagation_off        Disable forward propagation constraints
        -h, --help                      Prints help information
            --mics                      Compute minimal inconsistent cores
            --scenfit                    Compute scenfit of the data, default is mcos
        -p, --show_predictions          Show predictions
        -V, --version                   Prints version information

    OPTIONS:
        -n, --network <networkfile>              Influence graph in CIF format
        -o, --observations <observationfile>     Observations in bioquali format
        -l, --show_labelings <show_labelings>   Show N labelings to print, default is OFF, 0=all


The second program contained is opt_graph
Typical usage is:

    $ opt_graph -n network.cif -o observations_dir/ --show_repairs 10

For more options you can ask for help as follows:

    $ opt_graph -h
    opt_graph 2.0.0
    Sven Thiele <sthiele78@gmail.com>
    Opt-graph confronts interaction graph models with observations of (signed) changes between two measured states. Opt-graph computes networks fitting the observation data by removing (or adding) a minimal number of edges in the given network.

    USAGE:
        opt_graph [FLAGS] [OPTIONS] --network <networkfile> --observations <observationdir>

    FLAGS:
        -a, --autoinputs                Declare nodes with indegree 0 as inputs
            --depmat                    Combine multiple states, a change must be explained by an elementary path from an
                                        input
            --elempath                  Every change must be explained by an elementary path from an input
            --founded_constraints_off    Disable foundedness constraints
            --fwd_propagation_off        Disable forward propagation constraints
        -h, --help                      Prints help information
        -V, --version                   Prints version information

    OPTIONS:
        -r, --show_repairs <max_repairs>     Show max_repairs repairs, default is OFF, 0=all
        -n, --network <networkfile>            Influence graph in CIF format
        -o, --observations <observationdir>   Directory of observations in bioquali format
        -m, --repair_mode <repair_mode>       Repair mode: remove = remove edges (default),
                                                           optgraph = add + remove edges,
                                                           flip = flip direction of edges


# Samples

Sample files available here: [demo_data.tar.gz](https://bioasp.github.io/iggy/downloads/demo_data.tar.gz)
