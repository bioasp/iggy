Installation
============


You can install iggy by running::

	$ pip install --user iggy

The executable scripts can then be found in ~/.local/bin.


Usage
=====

Typical usage is::

	$ iggy.py network.sif observation.obs --show_colorings 10 --show_predictions

For more options you can ask for help as follows::

	$ iggy.py -h 		
	usage: iggy.py [-h] [--no_zero_constraints]
               [--propagate_unambigious_influences] [--no_founded_constraint]
               [--autoinputs] [--scenfit] [--show_colorings SHOW_COLORINGS]
               [--show_predictions]
               networkfile observationfile

	positional arguments:
	  networkfile           influence graph in SIF format
	  observationfile       observations in bioquali format

	optional arguments:
	  -h, --help            show this help message and exit
	  --no_zero_constraints
				turn constraints on zero variations OFF, default is ON
	  --propagate_unambigious_influences
				turn constraints ON that if all predecessor of a node
				have the same influence this must have an effect,
				default is ON
	  --no_founded_constraint
				turn constraints OFF that every variation must be
				explained by an input, default is ON
	  --autoinputs          compute possible inputs of the network (nodes with
				indegree 0)
	  --scenfit             compute scenfit of the data, default is mcos
	  --show_colorings SHOW_COLORINGS
				number of colorings to print, default is OFF, 0=all
	  --show_predictions    show predictions


The second script contained is opt_graph.py
Typical usage is::

	$ opt_graph.py network.sif observations_dir/ --show_repairs 10

For more options you can ask for help as follows::

	$ opt_graph.py -h 	
	usage: opt_graph.py [-h] [--no_zero_constraints]
		    [--propagate_unambigious_influences]
		    [--no_founded_constraint] [--autoinputs]
		    [--show_repairs SHOW_REPAIRS] [--opt_graph]
		    networkfile observationfiles

	positional arguments:
	  networkfile           influence graph in SIF format
	  observationfiles      directory of observations in bioquali format

	optional arguments:
	  -h, --help            show this help message and exit
	  --no_zero_constraints
				turn constraints on zero variations OFF, default is ON
	  --propagate_unambigious_influences
				turn constraints ON that if all predecessor of a node
				have the same influence this must have an effect,
				default is ON
	  --no_founded_constraint
				turn constraints OFF that every variation must be
				explained by an input, default is ON
	  --autoinputs          compute possible inputs of the network (nodes with
				indegree 0)
	  --show_repairs SHOW_REPAIRS
				number of repairs to show, default is OFF, 0=all
	  --opt_graph           compute opt-graph repairs (allows also adding edges),
				default is only removing edges


Samples
=======

Sample files available here:: iggy_demo_data.tar.gz_

.. _iggy_demo_data.tar.gz: http://www.cs.uni-potsdam.de/~sthiele/bioasp/downloads/samples/iggy_demo_data.tar.gz
