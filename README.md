# Installation



You can install iggy by running:

    > pip install --user iggy

On Linux the executable scripts can then be found in `~/.local/bin`

and on MacOS the scripts are under `/Users/YOURUSERNAME/Library/Python/3.5/bin`.


# Usage


Typical usage is:

    > iggy.py network.sif observation.obs --show_labelings 10 --show_predictions

For more options you can ask for help as follows:

    > iggy.py -h
    usage: iggy.py [-h] [--no_fwd_propagation] [--no_founded_constraints]
                   [--elempath] [--depmat] [--mics] [--autoinputs] [--scenfit]
                   [--show_labelings SHOW_LABELINGS] [--show_predictions]
                   networkfile observationfile

    Iggy confronts biological networks given as interaction graphs with
    experimental observations given as signs that represent the concentration
    changes between two measured states. Iggy supports the incorporation of
    uncertain measurements, discovers inconsistencies in data or network, applies
    minimal repairs, and predicts the behavior of unmeasured species. In
    particular, it distinguishes strong predictions (e.g. increase of a node
    level) and weak predictions (e.g., node level increases or remains unchanged).

    positional arguments:
      networkfile           influence graph in SIF format
      observationfile       observations in bioquali format

    optional arguments:
      -h, --help            show this help message and exit
      --no_fwd_propagation  turn forward propagation OFF, default is ON
      --no_founded_constraints
                            turn constraints OFF that every variation must be
                            founded in an input, default is ON
      --elempath            a change must be explained by an elementary path from
                            an input.
      --depmat              combines multiple states, a change must be explained
                            by an elementary path from an input.
      --mics                compute minimal inconsistent cores
      --autoinputs          compute possible inputs of the network (nodes with
                            indegree 0)
      --scenfit             compute scenfit of the data, default is mcos
      --show_labelings SHOW_LABELINGS
                            number of labelings to print, default is OFF, 0=all
      --show_predictions    show predictions


The second script contained is opt_graph.py
Typical usage is:

    > opt_graph.py network.sif observations_dir/ --show_repairs 10

For more options you can ask for help as follows:

    > opt_graph.py -h
    usage: opt_graph.py [-h] [--no_fwd_propagation] [--no_founded_constraints]
                        [--elempath] [--depmat] [--autoinputs]
                        [--show_repairs SHOW_REPAIRS] [--repair_mode REPAIR_MODE]
                        networkfile observationfiles

    Opt-graph confronts a biological network given as interaction graphs with sets
    of experimental observations given as signs that represent the concentration
    changes between two measured states. Opt-graph computes the networks fitting
    the observation data by removing (or adding) a minimal number of edges in the
    given network

    positional arguments:
      networkfile           influence graph in SIF format
      observationfiles      directory of observations in bioquali format

    optional arguments:
      -h, --help            show this help message and exit
      --no_fwd_propagation  turn forward propagation OFF, default is ON
      --no_founded_constraints
                            turn constraints OFF that every variation must be
                            founded in an input, default is ON
      --elempath            a change must be explained by an elementary path from
                            an input.
      --depmat              combines multiple states, a change must be explained
                            by an elementary path from an input.
      --autoinputs          compute possible inputs of the network (nodes with
                            indegree 0)
      --show_repairs SHOW_REPAIRS
                            number of repairs to show, default is OFF, 0=all
      --repair_mode REPAIR_MODE
                            choose repair mode: 1 = remove edges (default), 2 = add +
                            remove edges (opt-graph), 3 = flip edges


# Samples

Sample files available here: [demo_data.tar.gz](https://bioasp.github.io/iggy/downloads/demo_data.tar.gz)
