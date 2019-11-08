[![Build Status](https://travis-ci.org/bioasp/iggy.svg?branch=master)](https://travis-ci.org/bioasp/iggy)

# `iggy` + `opt_graph`

`iggy` and `opt_graph` are tools for consistency based analysis of influence graphs and observed systems behavior (signed changes between two measured states). For many (biological) systems are knowledge bases available that describe the interaction of its components in terms of causal networks, boolean networks and influence graphs where edges indicate either positive or negative effect of one node upon another.

`iggy` implements methods to check the consistency of large-scale data sets and provides explanations for inconsistencies. In practice, this is used to identify unreliable data or to indicate missing reactions. Further, `iggy` addresses the problem of  repairing networks and corresponding yet often discrepant measurements in order to re-establish their mutual consistency and predict unobserved variations even under inconsistency.

`opt_graph` confronts interaction graph models with observed systems behavior from multiple experiments. `opt_graph` computes networks fitting the observation data by removing (or adding) a minimal number of edges in the given network.

## Downloads

- precompiled binaries for 64bit linux and macOS can be found on the [release page](https://github.com/bioasp/iggy/releases/latest)

- [Iggy user guide](https://bioasp.github.io/iggy/guide/guide.html)

- sample data [demo_data.tar.gz](https://bioasp.github.io/iggy/downloads/demo_data.tar.gz)


## Compile yourself

Clone the git repository:

    git clone https://github.com/bioasp/iggy.git
    cargo build --release

The executables can be found under `./target/release/`

## Iggy

Typical usage is:

    > iggy -n network.cif -o observation.obs -l 10 -p

For more options you can ask for help as follows:

    > iggy -h
    iggy 2.1.0
    Sven Thiele <sthiele78@gmail.com>
    Iggy confronts interaction graph models with observations of (signed) changes between two measured states 
    (including uncertain observations). Iggy discovers inconsistencies in networks or data, applies minimal 
    repairs, and predicts the behavior for the unmeasured species. It distinguishes strong predictions (e.g. 
    increase in a node) and weak predictions (e.g., the value of a node increases or remains unchanged).

    USAGE:
        iggy [FLAGS] [OPTIONS] --network <network-file>

    FLAGS:
        -a, --auto-inputs                Declare nodes with indegree 0 as inputs
            --depmat                     Combine multiple states, a change must be explained by an elementary path from an
                                         input
            --elempath                   Every change must be explained by an elementary path from an input
            --founded-constraints-off    Disable foundedness constraints
            --fwd-propagation-off        Disable forward propagation constraints
        -h, --help                       Prints help information
            --mics                       Compute minimal inconsistent cores
            --scenfit                    Compute scenfit of the data, default is mcos
        -p, --show-predictions           Show predictions
        -V, --version                    Prints version information

    OPTIONS:
        -l, --show-labelings <max-labelings>     Show max-labelings labelings, default is OFF, 0=all
        -n, --network <network-file>             Influence graph in CIF format
        -o, --observations <observations-file>   Observations in bioquali format


## Opt_graph

Typical usage is:

    > opt_graph -n network.cif -o observations_dir/ --show_repairs 10

For more options you can ask for help as follows:

    > opt_graph -h
    opt_graph 2.1.0
    Sven Thiele <sthiele78@gmail.com>
    Opt-graph confronts interaction graph models with observations of (signed) changes between two measured 
    states. Opt-graph computes networks fitting the observation data by removing (or adding) a minimal number 
    of edges in the given network.

    USAGE:
        opt_graph [FLAGS] [OPTIONS] --network <network-file> --observations <observations-dir>

    FLAGS:
        -a, --auto-inputs               Declare nodes with indegree 0 as inputs
            --depmat                    Combine multiple states, a change must be explained by an                                elementary path from an input
            --elempath                  Every change must be explained by an elementary path from an                             input
            --founded-constraints-off   Disable foundedness constraints
            --fwd-propagation-off       Disable forward propagation constraints
        -h, --help                      Prints help information
        -V, --version                   Prints version information

    OPTIONS:
        -r, --show-repairs <max-repairs>        Show max-repairs repairs, default is OFF, 0=all
        -n, --network <network-file>            Influence graph in CIF format
        -o, --observations <observations-dir>   Directory of observations in bioquali format
        -m, --repair-mode <repair-mode>         Repair mode: remove = remove edges (default),
                                                           optgraph = add + remove edges,
                                                           flip = flip direction of edges
