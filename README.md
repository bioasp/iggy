# Installation


Clone the git repository:

	git clone https://github.com/sthiele/iggy.git


Compile:

	cargo build --release

The binaries can be found under './target/release/'


# Usage

Typical usage is:

	$ iggy -n network.cif -o observation.obs -l 10 -p

For more options you can ask for help as follows:

	$ iggy -h
	Iggy 0.1.0
	Sven Thiele <sthiele78@gmail.com>
	Iggy confronts interaction graph models with observations of (signed) changes between two measured states (including uncertain observations). Iggy discovers inconsistencies in networks or data, applies minimal repairs, and predicts the behavior for the unmeasured species. It distinguishes strong predictions (e.g. increase in a node) and weak predictions (e.g., the value of a node increases or remains unchanged).

	USAGE:
		iggy [FLAGS] [OPTIONS] --network <networkfile> --observations <observationfile>

	FLAGS:
		-a, --autoinputs                 Declare nodes with indegree 0 as inputs
			--depmat                     Combine multiple states, a change must be explained by an elementary path from an
										input
			--elempath                   Every change must be explained by an elementary path from an input
			--founded_constraints_off    Disable foundedness constraints
			--fwd_propagation_off        Disable forward propagation constraints
		-h, --help                       Prints help information
			--mics                       Compute minimal inconsistent cores
			--scenfit                    Compute scenfit of the data, default is mcos
		-p, --show_predictions           Show predictions
		-V, --version                    Prints version information

	OPTIONS:
		-n, --network <networkfile>              Influence graph in NSSIF format
		-o, --observations <observationfile>     Observations in bioquali format
		-l, --show_labelings <show_labelings>    Show N labelings to print, default is OFF, 0=all


The second script contained is opt_graph
Typical usage is:

	$ opt_graph -n network.cif -o observations_dir/ --show_repairs 10

For more options you can ask for help as follows:

	$ opt_graph -h
	usage: opt_graph [-h] [--no_fwd_propagation] [--no_founded_constraints]
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
				choose repair mode: 1 = add edges (default), 2 = add +
				remove edges (opt-graph), 3 = flip edges


# Samples

Sample files available here: [demo_data.tar.gz](https://bioasp.github.io/iggy/downloads/demo_data.tar.gz)
