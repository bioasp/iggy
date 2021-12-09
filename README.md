# `iggy` + `optgraph` [![Build Status](https://github.com/bioasp/iggy/workflows/CI%20Test/badge.svg)](https://github.com/bioasp/iggy)

`iggy` and `optgraph` are tools for consistency based analysis of influence graphs and observed systems behavior (signed changes between two measured states). For many (biological) systems are knowledge bases available that describe the interaction of its components in terms of causal networks, boolean networks and influence graphs where edges indicate either positive or negative effect of one node upon another.

`iggy` implements methods to check the consistency of large-scale data sets and provides explanations for inconsistencies. In practice, this is used to identify unreliable data or to indicate missing reactions. Further, `iggy` addresses the problem of  repairing networks and corresponding yet often discrepant measurements in order to re-establish their mutual consistency and predict unobserved variations even under inconsistency.

`optgraph` confronts interaction graph models with observed systems behavior from multiple experiments. `opt_graph` computes networks fitting the observation data by removing (or adding) a minimal number of edges in the given network.

## Downloads

- Precompiled binaries for 64bit linux and macOS can be found on the [release page](https://github.com/bioasp/iggy/releases/latest)

- [Iggy user guide](https://bioasp.github.io/iggy/guide/guide.html)

- Sample data [demo_data.tar.gz](https://bioasp.github.io/iggy/downloads/demo_data.tar.gz)

## Compile yourself

Clone the git repository:

```sh
git clone https://github.com/bioasp/iggy.git
cargo build --release
```

The executables can be found under `./target/release/`

## Iggy

Typical usage is:

```sh
iggy -n network.cif -o observation.obs -l 10 -p
```

For more options you can ask for help as follows:

```txt
> iggy -h
iggy 2.2.0

Sven Thiele <sthiele78@gmail.com>

Iggy confronts interaction graph models with observations of (signed) changes between two measured
states (including uncertain observations). Iggy discovers inconsistencies in networks or data,
applies minimal repairs, and predicts the behavior for the unmeasured species. It distinguishes
strong predictions (e.g. increase in a node) and weak predictions (e.g., the value of a node
increases or remains unchanged)

USAGE:
    iggy [OPTIONS] --network <NETWORK_FILE>

OPTIONS:
    -a, --auto-inputs
            Declare nodes with indegree 0 as inputs

        --depmat
            Combine multiple states, a change must be explained by an elementary path from an input

        --elempath
            Every change must be explained by an elementary path from an input

        --founded-constraints-off
            Disable foundedness constraints

        --fwd-propagation-off
            Disable forward propagation constraints

    -h, --help
            Print help information

        --json
            Print JSON output

    -l, --show-labelings <MAX_LABELINGS>
            Show MAX_LABELINGS labelings, default is OFF, 0=all

        --mics
            Compute minimal inconsistent cores

    -n, --network <NETWORK_FILE>
            Influence graph in CIF format

    -o, --observations <OBSERVATIONS_FILE>
            Observations in bioquali format

    -p, --show-predictions
            Show predictions

        --scenfit
            Compute scenfit of the data, default is mcos

    -V, --version
            Print version information
```

## Optgraph

Typical usage is:

```sh
optgraph -n network.cif -o observations_dir/ --show-repairs 10
```

For more options you can ask for help as follows:

```txt
> optgraph -h
optgraph 2.2.0

Sven Thiele <sthiele78@gmail.com>

Optgraph confronts interaction graph models with observations of (signed) changes between two
measured states. Opt-graph computes networks fitting the observation data by removing (or adding) a
minimal number of edges in the given network

USAGE:
    optgraph [OPTIONS] --network <NETWORK_FILE> --observations <OBSERVATIONS_DIR>

OPTIONS:
    -a, --auto-inputs
            Declare nodes with indegree 0 as inputs

        --depmat
            Combine multiple states, a change must be explained by an elementary path from an input

        --elempath
            Every change must be explained by an elementary path from an input

        --founded-constraints-off
            Disable foundedness constraints

        --fwd-propagation-off
            Disable forward propagation constraints

    -h, --help
            Print help information

        --json
            Print JSON output

    -m, --repair-mode <REPAIR_MODE>
            REPAIR_MODE: remove = remove edges (default), optgraph = add + remove edges, flip = flip
            direction of edges

    -n, --network <NETWORK_FILE>
            Influence graph in CIF format

    -o, --observations <OBSERVATIONS_DIR>
            Directory of observations in bioquali format

    -r, --show-repairs <MAX_REPAIRS>
            Show MAX_REPAIRS repairs, default is OFF, 0=all

    -V, --version
            Print version information
```
