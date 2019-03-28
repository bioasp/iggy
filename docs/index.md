---
layout: index
title: iggy
tagline: A tool for consistency based analysis of influence graphs and observed systems behavior
---

### Sign Consistency on Influence Graphs - Diagnosis, Repair, Prediction

For many biological systems knowledge bases are available that describe the interaction of its components usually in terms of causal networks and influence graphs. In particular signed influence graphs where edges indicate either positive or negative effect of one node upon another. Building upon a notion of consistency between biochemical/genetic regulations and high-throughput profiles of cell activity. We present an approach to check the consistency of large-scale data sets, provide explanations for inconsistencies by determining minimal representations of conflicts. In practice, this can be used to identify unreliable data or to indicate missing reactions. Further, we address the problem of repairing networks and corresponding yet often discrepant measurements in order to re-establish their mutual consistency and predict unobserved variations even under inconsistency. 
[![DOI](https://zenodo.org/badge/5393/bioasp/iggy.png)](http://dx.doi.org/10.5281/zenodo.19042)

### Installation

You can install iggy by running:

	$ pip install --user iggy

On Linux the executable scripts can then be found in ``~/.local/bin``

and on Mac OS the scripts are under ``/Users/YOURUSERNAME/Library/Python/3.5/bin``.


### Usage

You can download the [iggy user guide](https://bioasp.github.io/iggy/guide/guide.pdf).
Typical usage is:

	$ iggy.py network.sif observation.obs --show_labelings 10 --show_predictions

For more options you can ask for help as follows:

	$ iggy.py -h 		
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

	$ opt_graph.py network.sif observations_dir/ --show_repairs 10

For more options you can ask for help as follows:

	$ opt_graph.py -h 
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
				choose repair mode: 1 = add edges (default), 2 = add +
				remove edges (opt-graph), 3 = flip edges


### Samples

Sample files available here: [demo\_data.tar.gz](https://bioasp.github.io/iggy/downloads/demo_data.tar.gz)

### Related publications

* [*Extended notions of sign consistency to relate experimental data to signaling and regulatory network topologies*](http://dx.doi.org/10.1186/s12859-015-0733-7), BMC Bioinformatics, 2015.

* [*Repair and Prediction (under Inconsistency) in Large Biological Networks with Answer Set Programming*](http://aaai.org/ocs/index.php/KR/KR2010/paper/view/1334/1660), 12th International Conference on the Principles of Knowledge Representation and Reasoning, 2010.

* [*Directed random walks and constraint programming reveal active pathways in HGF signaling*](http://dx.doi.org/10.1111/febs.13580), FEBS Journal, 2015.

### FAQ

**Q**: I don't have pip. How can I install pip without admin rights?

**A**: You can install pip without admin rights.

1. Download [getpip.py](https://bootstrap.pypa.io/get-pip.py).

		$ wget https://bootstrap.pypa.io/get-pip.py

2. Install pip locally. 

		$ python get-pip.py --user

3. You can install using your local pip.


**Q**: I don't have pip. How can I install iggy without pip?

**A**:  You can install iggy without pip if you take care of the dependencies yourself.

1. Download [pyasp-1.4.3](https://pypi.python.org/pypi/pyasp/1.4.3). 
 
		$ wget https://pypi.python.org/packages/source/p/pyasp/pyasp-1.4.3.tar.gz

2. Extract and install pyasp. 

		$ gzip -d pyasp-1.4.3.tar.gz
		$ tar -xvf pyasp-1.4.3.tar
		$ cd pyasp-1.4.3
		$ python setup.py install --user

3. Download [iggy-1.4.1](https://pypi.python.org/pypi/iggy/1.4.1). 

		$ wget https://pypi.python.org/packages/source/i/iggy/iggy-1.4.1.tar.gz
 
4. Extract and install iggy.

		$ gzip -d iggy-1.4.1.tar.gz
		$ tar -xvf iggy-1.4.1.tar
		$ cd iggy-1.4.1
		$ python setup.py install --user
	

   The executable script can then be found in ``~/.local/bin`` on Linux and in ``/Users/YOURUSERNAME/Library/Python/3.5/bin``on Mac OS.


**Q**: How can I write the output of iggy into a file?

**A**:  You can redirect the output of iggy using ``>`` into a file. For example to write the results into the file ``myfile.txt`` type:

		$ iggy.py network.sif observation.obs --show_labelings 10 --show_predictions > myfile.txt
	
