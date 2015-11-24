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
# MERCHANTABILITY or FITNEOS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with iggy.  If not, see <http://www.gnu.org/licenses/>.

# -*- coding: utf-8 -*-

"""
This module contains the queries which can be asked to the model and data.
"""

import os
import tempfile
from pyasp.asp import *

root = __file__.rsplit('/', 1)[0]
contradictory_obs_prg   = root + '/encodings/contradictory_obs.lp'
guess_inputs_prg        = root + '/encodings/guess_inputs.lp'

sign_cons_prg           = root + '/encodings/sign-cons-3.lp'

one_state_prg           = root + '/encodings/one_state.lp'
bwd_prop_prg            = root + '/encodings/backward_propagation.lp'
fwd_prop_prg            = root + '/encodings/forward_propagation.lp'
founded_prg             = root + '/encodings/founded_constraints.lp'

elem_path_prg           = root + '/encodings/elementary_path_constraint.lp'
some_path_prg           = root + '/encodings/path_constraint.lp'

keep_inputs_prg         = root + '/encodings/keep_inputs_constraint.lp'
keep_obs_prg            = root + '/encodings/keep_observations_constraint.lp'

error_measure_prg       = root + '/encodings/error_measure.lp'
min_weighted_error_prg  = root + '/encodings/minimize_weighted_error.lp'

add_influence_prg       = root + '/encodings/add_influence.lp'
min_added_influence_prg = root + '/encodings/minimize_added_influences.lp'

add_edges_prg           = root + '/encodings/add_edges.lp'
min_added_edges_prg     = root + '/encodings/minimize_added_edges.lp'

remove_edges_prg        = root + '/encodings/remove_edges.lp'
max_removed_edges_prg   = root + '/encodings/maximize_removed_edges.lp'
min_removed_edges_prg   = root + '/encodings/minimize_removed_edges.lp'

flip_edges_prg          = root + '/encodings/flip_edges.lp'

#min_repairs_prg         = root + '/encodings/minimize_repairs.lp'
min_repairs_prg         = root + '/encodings/minimize_weighted_repairs.lp'
max_add_edges_prg       = root + '/encodings/max_add_edges.lp'


mics_prg                = root + '/encodings/mics.lp'
mics_constr_luca_prg    = root + '/encodings/mics_luca_constraints.lp'
mics_fwd_prop_prg    = root + '/encodings/mics_constrained_zero.lp'
mics_founded_prg        = root + '/encodings/mics_founded_constraints.lp'


heu_prg                 = root + '/encodings/heuristic.lp'

show_pred_prg           = root + '/encodings/show_predictions.lp'
show_labels_prg         = root + '/encodings/show_vlabels.lp'
show_err_prg            = root + '/encodings/show_errors.lp'
show_rep_prg            = root + '/encodings/show_repairs.lp'

scenfit = [error_measure_prg, min_weighted_error_prg, keep_inputs_prg]
mcos    = [add_influence_prg, min_added_influence_prg, keep_obs_prg]


def get_scenfit(instance, OS, FP, FC, EP, SP):
  '''returns the scenfit of data and model described by the 
  ``TermSet`` object [instance].
  '''
  sem = [sign_cons_prg, bwd_prop_prg]
  if OS : sem.append(one_state_prg)
  if FP : sem.append(fwd_prop_prg)
  if FC : sem.append(founded_prg)
  if EP : sem.append(elem_path_prg)
  if SP : sem.append(some_path_prg)

  inst     = instance.to_file()
  prg      = sem + scenfit + [inst]
  coptions = '--opt-strategy=5'
  solver   = GringoClasp(clasp_options=coptions)
  solution = solver.run(prg,collapseTerms=True,collapseAtoms=False)
  opt      = solution[0].score[0]

  os.unlink(inst)
  return opt
    
def get_scenfit_labelings(instance,nm, OS, FP, FC, EP, SP):
  '''
  returns a list of atmost [nm] ``TermSet`` representing scenfit labelings
  to the system described by the ``TermSet`` object [instance].
  '''
  sem = [sign_cons_prg, bwd_prop_prg]
  if OS : sem.append(one_state_prg)
  if FP : sem.append(fwd_prop_prg)
  if FC : sem.append(founded_prg)
  if EP : sem.append(elem_path_prg)
  if SP : sem.append(some_path_prg)
 
  inst     = instance.to_file("instance.lp")
  prg      = sem + scenfit + [inst]
  coptions = '--opt-strategy=5'
  solver   = GringoClasp(clasp_options=coptions)
  solution = solver.run(prg,collapseTerms=True,collapseAtoms=False)

  opt      = solution[0].score[0]

  prg      = prg + [show_labels_prg, show_err_prg]
  coptions = str(nm)+' --project --opt-strategy=5 --opt-mode=optN --opt-bound='+str(opt)
  solver2  = GringoClasp(clasp_options=coptions)
  models   = solver2.run(prg,collapseTerms=True,collapseAtoms=False)
  
  os.unlink(inst)
  return models
     
def get_predictions_under_scenfit(instance, OS, FP, FC, EP, SP):
  '''
  '''
  sem = [sign_cons_prg,bwd_prop_prg]
  if OS : sem.append(one_state_prg)
  if FP : sem.append(fwd_prop_prg)
  if FC : sem.append(founded_prg)
  if EP : sem.append(elem_path_prg)
  if SP : sem.append(some_path_prg)

  inst     = instance.to_file()
  prg      = sem + scenfit + [inst]
  coptions = '--opt-strategy=5'
  solver   = GringoClasp(clasp_options=coptions)
  solution = solver.run(prg,collapseTerms=True,collapseAtoms=False)

  opt      = solution[0].score[0]

  prg      = prg + [show_pred_prg]
  coptions = '--opt-strategy=5 --opt-mode=optN --enum-mode=cautious --opt-bound='+str(opt)
  solver2  = GringoClasp(clasp_options=coptions)
  models   = solver2.run(prg,collapseTerms=True,collapseAtoms=False)

  os.unlink(inst)    
  return models[0]

def get_mcos(instance, OS, FP, FC, EP, SP):
  '''
  '''
  sem = [sign_cons_prg, bwd_prop_prg]
  if OS : sem.append(one_state_prg)
  if FP : sem.append(fwd_prop_prg)
  if FC : sem.append(founded_prg)
  if EP : sem.append(elem_path_prg)
  if SP : sem.append(some_path_prg)

  inst     = instance.to_file()
  prg      = sem + mcos + [inst]
  coptions = '--opt-strategy=5'
  solver   = GringoClasp(clasp_options=coptions)
  solution = solver.run(prg,collapseTerms=True,collapseAtoms=False)
  
  opt      = solution[0].score[0]

  os.unlink(inst) 
  return opt

def get_mcos_labelings(instance,nm, OS, FP, FC, EP, SP):
  '''
  '''
  sem = [sign_cons_prg, bwd_prop_prg]
  if OS : sem.append(one_state_prg)
  if FP : sem.append(fwd_prop_prg)
  if FC : sem.append(founded_prg)
  if EP : sem.append(elem_path_prg)
  if SP : sem.append(some_path_prg)

  inst     = instance.to_file()
  prg      = sem + mcos + [inst]
  coptions = '--opt-strategy=5'
  solver   = GringoClasp(clasp_options=coptions)
  solution = solver.run(prg,collapseTerms=True,collapseAtoms=False)
  
  opt      = solution[0].score[0]

  prg      = prg + [show_labels_prg, show_rep_prg]
  coptions = str(nm)+' --project --opt-strategy=5 --opt-mode=optN --opt-bound='+str(opt)
  solver2  = GringoClasp(clasp_options=coptions)
  models   = solver2.run(prg,collapseTerms=True,collapseAtoms=False)

  os.unlink(inst) 
  return models


def get_predictions_under_mcos(instance, OS, FP, FC, EP, SP):
  '''
  '''
  sem = [sign_cons_prg,bwd_prop_prg]
  if OS : sem.append(one_state_prg)
  if FP : sem.append(fwd_prop_prg)
  if FC : sem.append(founded_prg)
  if EP : sem.append(elem_path_prg)
  if SP : sem.append(some_path_prg)

  inst     = instance.to_file()
  prg      = sem + mcos + [inst]
  coptions = '--opt-strategy=5'
  solver   = GringoClasp(clasp_options=coptions)
  solution = solver.run(prg,collapseTerms=True,collapseAtoms=False)

  opt      = solution[0].score[0]

  prg      = prg + [show_pred_prg]
  coptions = '--opt-strategy=5 --opt-mode=optN --enum-mode=cautious --opt-bound='+str(opt)
  solver2  = GringoClasp(clasp_options=coptions)
  models   = solver2.run(prg,collapseTerms=True,collapseAtoms=False)

  os.unlink(inst)
  return models[0]


def get_opt_remove_edges(instance, OS, FP, FC, EP, SP):

  sem = [sign_cons_prg,bwd_prop_prg]
  if OS : sem.append(one_state_prg)
  if FP : sem.append(fwd_prop_prg)
  if FC : sem.append(founded_prg)
  if EP : sem.append(elem_path_prg)
  if SP : sem.append(some_path_prg)

  inst     = instance.to_file()
  prg      = sem + scenfit + [remove_edges_prg, min_repairs_prg, inst ]
  coptions = '--opt-strategy=5'
  solver   = GringoClasp(clasp_options=coptions)
  solution = solver.run(prg,collapseTerms=True,collapseAtoms=False)
  fit      = solution[0].score[0]
  repairs  = solution[0].score[1]

  os.unlink(inst)
  return (fit,repairs)


def get_opt_repairs_remove_edges(instance,nm, OS, FP, FC, EP, SP):

  sem = [sign_cons_prg,bwd_prop_prg]
  if OS : sem.append(one_state_prg)
  if FP : sem.append(fwd_prop_prg)
  if FC : sem.append(founded_prg)
  if EP : sem.append(elem_path_prg)
  if SP : sem.append(some_path_prg)

  inst     = instance.to_file()
  prg      = sem + scenfit + [remove_edges_prg, min_repairs_prg, inst ]

  coptions = '--opt-strategy=5'
  solver   = GringoClasp(clasp_options=coptions)
  solution = solver.run(prg,collapseTerms=True,collapseAtoms=False)
  fit      = solution[0].score[0]
  repairs  = solution[0].score[1]

  prg      = prg + [show_rep_prg]
  coptions = str(nm)+' --project --opt-strategy=5 --opt-mode=optN --opt-bound='+str(fit)+','+str(repairs)
  solver2  = GringoClasp(clasp_options=coptions)
  models   = solver2.run(prg, collapseTerms=True, collapseAtoms=False)

  os.unlink(inst)
  return models


def get_opt_flip_edges(instance, OS, FP, FC, EP, SP):

  sem = [sign_cons_prg,bwd_prop_prg]
  if OS : sem.append(one_state_prg)
  if FP : sem.append(fwd_prop_prg)
  if FC : sem.append(founded_prg)
  if EP : sem.append(elem_path_prg)
  if SP : sem.append(some_path_prg)

  inst     = instance.to_file()
  prg      = sem + scenfit + [flip_edges_prg, min_repairs_prg, inst ]
  coptions = '--opt-strategy=5'
  solver   = GringoClasp(clasp_options=coptions)
  solution = solver.run(prg,collapseTerms=True,collapseAtoms=False)
  fit      = solution[0].score[0]
  repairs  = solution[0].score[1]

  os.unlink(inst)
  return (fit,repairs)


def get_opt_repairs_flip_edges(instance,nm, OS, FP, FC, EP, SP):

  sem = [sign_cons_prg,bwd_prop_prg]
  if OS : sem.append(one_state_prg)
  if FP : sem.append(fwd_prop_prg)
  if FC : sem.append(founded_prg)
  if EP : sem.append(elem_path_prg)
  if SP : sem.append(some_path_prg)

  inst     = instance.to_file()
  prg      = sem + scenfit + [flip_edges_prg, min_repairs_prg, inst ]

  coptions = '--opt-strategy=5'
  solver   = GringoClasp(clasp_options=coptions)
  solution = solver.run(prg,collapseTerms=True,collapseAtoms=False)
  fit      = solution[0].score[0]
  repairs  = solution[0].score[1]

  prg      = prg + [show_rep_prg]
  coptions = str(nm)+' --project --opt-strategy=5 --opt-mode=optN --opt-bound='+str(fit)+','+str(repairs)
  solver2  = GringoClasp(clasp_options=coptions)
  models   = solver2.run(prg, collapseTerms=True, collapseAtoms=False)

  os.unlink(inst)
  return models


def get_opt_add_remove_edges_inc(instance):
  '''
   only apply with elementary path consistency notion
  '''
  sem      = [sign_cons_prg, elem_path_prg]

  inst     = instance.to_file()
 
  num_adds = 1
  maxfact  = String2TermSet('max_add_edges('+str(num_adds)+')')
  fmaxfact = maxfact.to_file()
  prg      = [ inst, fmaxfact,  
	       min_repairs_prg, max_add_edges_prg
	     ] + sem + scenfit

  coptions = '--opt-strategy=5'
  solver   = GringoClasp(clasp_options=coptions)
  solution = solver.run(prg,collapseTerms=True,collapseAtoms=False)
  fit      = solution[0].score[0]
  repairs  = solution[0].score[1]
  os.unlink(fmaxfact)

  print('fit,repairs:',fit,repairs)


  best = False
  # loop till no better solution can be found
  while not best :
    num_adds+= 1
    maxfact  = String2TermSet('max_add_edges('+str(num_adds)+')')
    fmaxfact = maxfact.to_file()
    prg      = [ inst, fmaxfact,
  	         min_repairs_prg, max_add_edges_prg
  	       ] + sem + scenfit
  
    coptions = '--opt-strategy=5'
    solver   = GringoClasp(clasp_options=coptions)
    solution = solver.run(prg,collapseTerms=True,collapseAtoms=False)
    nfit     = solution[0].score[0]
    nrepairs = solution[0].score[1]
    os.unlink(fmaxfact)

    print('fit,repairs:',nfit,nrepairs)

    if (nfit==fit) and (nrepairs==repairs) : best = True
    else:
      fit     = nfit
      repairs = nrepairs

  os.unlink(inst)
  return (fit,repairs)

def get_opt_add_remove_edges_greedy(instance):
  '''
   only apply with elementary path consistency notion
  '''
  sem      = [sign_cons_prg, elem_path_prg]
  inst     = instance.to_file()
  maxfact  = String2TermSet('max_add_edges(1)')
  fmaxfact = maxfact.to_file()
  prg      = [ inst, fmaxfact,
               min_repairs_prg, show_rep_prg,
             ] + sem + scenfit
      
  coptions = '1 --project --opt-strategy=5 --opt-mode=optN --quiet=1'
  solver   = GringoClasp(clasp_options=coptions)                        
  models   = solver.run(prg, collapseTerms=True, collapseAtoms=False)
  bfit     = models[0].score[0]

  strt_edges = TermSet()
  fedges     = [(strt_edges,bfit)]
  fedges2    = []
  tedges     = []

  coptions = '0 --project --opt-strategy=5 --opt-mode=optN --quiet=1'
  solver   = GringoClasp(clasp_options=coptions)                        


  while fedges:
    print ("TODO: ",len(fedges)+len(fedges2))
    (edges,ofit) = fedges.pop()
  # loop till no better solution can be found
    print('(edges,ofit):',(edges,ofit))
    end         = True # this time its the end
    f_edges     = TermSet(edges).to_file()
    prg         = [ inst, fmaxfact, f_edges,
                    min_repairs_prg, max_add_edges_prg, show_rep_prg,
                  ] + sem + scenfit

    models   = solver.run(prg, collapseTerms=True, collapseAtoms=False)
    fit      = models[0].score[0]
    repairs  = models[0].score[1]
    os.unlink(f_edges)

    if fit+1 < ofit : # more than one better
      print('new bfit:',str(bfit))
      if fit<bfit : bfit = fit     # update bfit
      end  = False

    if end : 
      if (edges,bfit,repairs) not in tedges : 
        print('LAST tedges append',edges)
        tedges.append((edges,fit,repairs))
    else : 
      for m in models:
        for a in m :
          if a.pred() == 'rep' :
            if a.arg(0)[0:7]=='addedge' :
              print ('new edge',a.arg(0)[7:])
              nedges  = edges.union(String2TermSet('obs_elabel'+(a.arg(0)[7:])))
              end     = False
        if nedges not in fedges2 :
           fedges2.append((nedges,fit))
           
    if not fedges :
      fedges=fedges2
      fedges2=[] # flip fedges

  # take only the results with the best fit
  redges=[]
  for (edges,fit,repairs) in tedges:
    print('red:  ',edges,str(fit),str(repairs))
    if fit == bfit: redges.append((edges,repairs))

  os.unlink(fmaxfact)
  os.unlink(inst)
  return (bfit,redges)


def get_opt_repairs_add_remove_edges_greedy(instance,nm, edges):
  '''
   only apply with elementary path consistency notion
  '''
  sem      = [sign_cons_prg, elem_path_prg]
  inst     = instance.to_file()
 
  maxfact  = String2TermSet('max_add_edges(0)')
  fmaxfact = maxfact.to_file()
  f_edges  = TermSet(edges).to_file()
  prg      = [ inst, fmaxfact, f_edges,
	       min_repairs_prg, max_add_edges_prg,
	       show_rep_prg
	     ] + sem + scenfit
  
  coptions = str(nm)+' --project --opt-strategy=5 --opt-mode=optN'
  solver2  = GringoClasp(clasp_options=coptions)
  models   = solver2.run(prg, collapseTerms=True, collapseAtoms=False)

  os.unlink(inst)
  return models

def get_opt_add_remove_edges(instance, OS, FP, FC, EP, SP):

  sem = [sign_cons_prg,bwd_prop_prg]
  if OS : sem.append(one)
  if FP : sem.append(fwd_prop_prg)
  if FC : sem.append(founded_prg)
  if EP : 
    print('error query.get_opt_add_remove_edges should not be called with'
          'elementary path constraint, use instead'
	  'get_opt_add_remove_edges_greedy')
    exit()
  if SP : sem.append(some_path_prg)

  inst     = instance.to_file()
  prg      = sem + scenfit + [remove_edges_prg, add_edges_prg, min_repairs_prg, inst ]
  coptions = '--opt-strategy=5'
  solver   = GringoClasp(clasp_options=coptions)
  solution = solver.run(prg,collapseTerms=True,collapseAtoms=False)
  fit      = solution[0].score[0]
  repairs  = solution[0].score[1]

  os.unlink(inst)
  return (fit,repairs)

def get_opt_repairs_add_remove_edges(instance,nm, OS, FP, FC,EP, SP):

  sem = [sign_cons_prg,bwd_prop_prg]
  if OS : sem.append(one_state_prg)
  if FP : sem.append(fwd_prop_prg)
  if FC : sem.append(founded_prg)
  if EP : sem.append(elem_path_prg)
  if SP : sem.append(some_path_prg)

  inst = instance.to_file()    
  prg  = sem + scenfit + [remove_edges_prg, add_edges_prg, min_repairs_prg, inst ]

  coptions = '--opt-strategy=5'
  solver   = GringoClasp(clasp_options=coptions)
  solution = solver.run(prg,collapseTerms=True,collapseAtoms=False)
  fit      = solution[0].score[0]
  repairs  = solution[0].score[1]

  prg      = prg + [show_rep_prg]
  coptions = str(nm)+' --project --opt-strategy=5 --opt-mode=optN --opt-bound='+str(fit)+','+str(repairs)
  solver2  = GringoClasp(clasp_options=coptions)
  models   = solver2.run(prg, collapseTerms=True, collapseAtoms=False)

  os.unlink(inst)
  return models
    
def get_minimal_inconsistent_cores(instance, OS, FP, FC, FESPC):
  '''
  '''
  sem = [mics_prg, heu_prg]
#  if OS   : sem.append(bwd_prop_prg)
  if FP   : sem.append(mics_fwd_prop_prg)
#  if FC   : sem.append(mics_founded_prg)

  inst     = instance.to_file()   
  prg      = sem+ [inst]
  coptions = '0 --dom-mod=6 --heu=Domain --enum-mode=record'
  solver   = GringoClasp(clasp_options=coptions)
  models   = solver.run(prg,collapseTerms=True, collapseAtoms=False)

  os.unlink(inst)    
  return models

def guess_inputs(instance):

  inst   = instance.to_file()   
  prg    = [ guess_inputs_prg, inst ]
  solver = GringoClasp()
  models = solver.run(prg, collapseTerms=True, collapseAtoms=False)
  os.unlink(inst)
  assert(len(models) == 1)

  return models[0]

def get_contradictory_obs(instance):

  inst   = instance.to_file()
  prg    = [ contradictory_obs_prg, inst ]
  solver = GringoClasp()
  models = solver.run(prg, collapseTerms=True, collapseAtoms=False)
  os.unlink(inst)
  assert(len(models) == 1)

  return models[0]

def get_reductions(instance):
  inst   = instance.to_file()
  prg    = [ reduction_prg, inst ]
  solver = GringoClasp()
  models = solver.run(prg,0)
  os.unlink(inst)
  assert(len(models) == 1)
  return models[0]


