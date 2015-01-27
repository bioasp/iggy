#!python
# Copyright (c) 2014, Sven Thiele <sthiele78@gmail.com>
#
# This file is part of iggy.
#
# iggy is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# iggy is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with iggy.  If not, see <http://www.gnu.org/licenses/>.
# -*- coding: utf-8 -*-
import sys
import argparse
from pyasp.asp import *
from __iggy__ import query, utils, bioquali

if __name__ == '__main__':
       
       
    desc = ('Iggy confronts biological networks given as interaction graphs with experimental observations '
            'given as signs that represent the concentration changes between two measured states. '
            'Iggy supports the incorporation of uncertain measurements, discovers inconsistencies in data or network, '
            'applies minimal repairs, and predicts the behavior of unmeasured species. '
            'In particular, it distinguishes strong predictions (e.g. increase of a node level) and weak predictions '
            '(e.g., node level increases or remains unchanged).')
    parser = argparse.ArgumentParser(description=desc)
    parser.add_argument("networkfile",
                        help="influence graph in SIF format")
    parser.add_argument("observationfile",
                        help="observations in bioquali format")
                        
    parser.add_argument('--no_zero_constraints',
			help="turn constraints on zero variations OFF, default is ON",
			action="store_true")
			
    parser.add_argument('--propagate_unambigious_influences',
			help="turn constraints ON that if all predecessor of a node have the same influence this must have an effect, default is ON",
			action="store_true")
			
    parser.add_argument('--no_founded_constraint',
			help="turn constraints OFF that every variation must be explained by an input, default is ON",
			action="store_true")

    parser.add_argument('--mics',
			help="compute minimal inconsistent cores",
			action="store_true")
			
    parser.add_argument('--autoinputs',
			help="compute possible inputs of the network (nodes with indegree 0)",
			action="store_true")

    parser.add_argument('--scenfit',
			help="compute scenfit of the data, default is mcos",
			action="store_true")
    
    parser.add_argument('--show_labelings',type=int, default=-1,
			help="number of labelings to print, default is OFF, 0=all")

    parser.add_argument('--show_predictions',
			help="show predictions",
			action="store_true")


    args = parser.parse_args()
    
    net_string = args.networkfile
    obs_string = args.observationfile

    LC =args.propagate_unambigious_influences
    CZ= not (args.no_zero_constraints)
    FC= not (args.no_founded_constraint)

    print ' all observed changes must be explained by an predecessor'
    if LC : print ' unambigious influences propagate'
    if CZ : print ' no-change observations must be explained'
    if FC : print ' all observed changes must be explained by an input'
    
    print '\nReading network',net_string, '...',
    net = bioquali.readSIFGraph(net_string)
    print 'done.'
    #edge_counter=0
    activations = set()
    inhibitions = set()
    nodes=set()
    for a in net:
      #if a.pred() == 'edge' : edge_counter+=1
      if a.pred() == 'obs_elabel' :
	if a.arg(2) == '1' : activations.add((a.arg(0),a.arg(1)))
        if a.arg(2) == '-1' : inhibitions.add((a.arg(0),a.arg(1)))
      if a.pred() == 'vertex' : nodes.add(a.arg(0))
    unspecified = activations & inhibitions
    print "   Nodes:",len(nodes)," Activations:",len(activations)," Inhibitions:",len(inhibitions)," Dual:",len(unspecified)

    print '\nReading observations',obs_string, '...',
    mu = bioquali.readProfile(obs_string)
    print 'done.'

    print '\nChecking observations',obs_string, '...',
    contradictions = query.get_contradictory_obs(mu)
    print 'done.'
    if len(contradictions) == 0 : print '   Observations are consistent.'
    else:
      print '   Contradictory observations found. Please correct manualy.'
      for c in contradictions : print '  ',c
      utils.clean_up()
      exit()
      
    plus=set()
    zero=set()
    minus=set()
    notminus=set()
    notplus=set()
    inputs=set()
    for a in mu:
      if a.pred() == 'obs_vlabel' :
        if a.arg(2) == '1'        : plus.add(a.arg(1))
	if a.arg(2) == '0'        : zero.add(a.arg(1))
	if a.arg(2) == '-1'       : minus.add(a.arg(1))
	if a.arg(2) == 'notMinus' : notminus.add(a.arg(1))
	if a.arg(2) == 'notPlus'  : notplus.add(a.arg(1))
      if a.pred() == 'input'      : inputs.add(a.arg(1))
      
    unobserved = nodes -(plus|minus|notplus| notminus)
    not_in_model= (plus|minus|notplus|notminus)-nodes
    print "   inputs:",len(inputs&nodes)," observed +:",len(plus&nodes), " observed -:",len(minus&nodes)," observed 0:",len(zero&nodes)," observed notPlus:",len(notplus&nodes)," observed notMinus:",len(notminus&nodes)," unobserved:",len(unobserved)," not in model:",len(not_in_model)

    if args.autoinputs :
      print '\nComputing input nodes ...',
      inputs = query.guess_inputs(net)
      net = net.union(inputs)
      print 'done.'
      print "   number of inputs:", len(inputs)
	
    net_with_data = net.union(mu)

    if args.scenfit :
      print '\nComputing scenfit of network and data ...',
      scenfit = query.get_scenfit(net_with_data, LucaConstraint=LC, ConstrainedZero=CZ, FoundedConstraint=FC)
      print 'done.'
      if scenfit == 0 : print "   The network and data are consistent: scenfit = 0."
      else:
	print "   The network and data are inconsistent: scenfit = "+str(scenfit)+'.'
	
	if args.mics:
	  print '\nComputing minimal inconsistent cores (mic\'s) ...',
	  sys.stdout.flush()
	  mics = query.get_minimal_inconsistent_cores(net_with_data, LucaConstraint=LC, ConstrainedZero=CZ, FoundedConstraint=FC)
	  print 'done.'
	  count = 1
	  oldmic = 0
	  for mic in mics:
	    if oldmic != mic:
	      print 'mic '+str(count)+':'
	      utils.print_mic(mic.to_list(),net.to_list(),mu.to_list())
	      count+=1
	      oldmic= mic


      if args.show_labelings >= 0 :
	print '\nCompute scenfit labelings...',
	labelings = query.get_scenfit_labelings(net_with_data, args.show_labelings, LucaConstraint=LC, ConstrainedZero=CZ, FoundedConstraint=FC)
	print 'done.'
	count=0
	for l in labelings :
	  count+=1
	  print 'Labeling',str(count)+':'
	  utils.print_labeling(l)

      if args.show_predictions :
	print '\nCompute predictions under scenfit ...',
	predictions = query.get_predictions_under_scenfit(net_with_data, LucaConstraint=LC, ConstrainedZero=CZ, FoundedConstraint=FC)
	print 'done.'
	utils.print_predictions(predictions)



    if not args.scenfit :
      print '\nComputing mcos of network and data ...',
      mcos = query.get_mcos(net_with_data, LucaConstraint=LC, ConstrainedZero=CZ, FoundedConstraint=FC)
      print 'done.'
      if mcos == 0 : print "   The network and data are consistent: mcos = 0."
      else:
	print "   The network and data are inconsistent: mcos = "+str(mcos)+'.'

      	if args.mics:
	  print '\nComputing minimal inconsistent cores (mic\'s) ...',
	  sys.stdout.flush()
	  mics = query.get_minimal_inconsistent_cores(net_with_data, LucaConstraint=LC, ConstrainedZero=CZ, FoundedConstraint=FC)
	  print 'done.'
	  count = 1
	  oldmic = 0
	  for mic in mics:
	    if oldmic != mic:
	      print 'mic '+str(count)+':'
	      utils.print_mic(mic.to_list(),net.to_list(),mu.to_list())
	      count+=1
	      oldmic= mic

      if args.show_labelings >= 0 :
	print '\nCompute mcos labelings...',
	labelings = query.get_mcos_labelings(net_with_data, args.show_labelings, LucaConstraint=LC, ConstrainedZero=CZ, FoundedConstraint=FC)
	print 'done.'
	count=0
	for l in labelings :
	  count+=1
	  print 'Labeling',str(count)+':'
	  utils.print_labeling(l)

      if args.show_predictions :
	print '\nCompute predictions under mcos ...',
	predictions = query.get_predictions_under_mcos(net_with_data, LucaConstraint=LC, ConstrainedZero=CZ, FoundedConstraint=FC)
	print 'done.'
	utils.print_predictions(predictions)




    utils.clean_up()



