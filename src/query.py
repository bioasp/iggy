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
from pyasp.term import String2TermSet;

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
min_repairs_prg          = root + '/encodings/minimize_weighted_repairs.lp'
best_one_edge_prg        = root + '/encodings/best_one_edge.lp'
best_edge_start_prg      = root + '/encodings/best_one_edge_start.lp'

mics_prg                = root + '/encodings/mics.lp'
mics_constr_luca_prg    = root + '/encodings/mics_luca_constraints.lp'
mics_fwd_prop_prg    = root + '/encodings/mics_constrained_zero.lp'
mics_founded_prg        = root + '/encodings/mics_founded_constraints.lp'


heu_prg                 = root + '/encodings/heuristic.lp'

show_pred_prg           = root + '/encodings/show_predictions.lp'
show_pred_dm_prg        = root + '/encodings/show_predictions_dm.lp'
show_labels_prg         = root + '/encodings/show_vlabels.lp'
show_err_prg            = root + '/encodings/show_errors.lp'
show_rep_prg            = root + '/encodings/show_repairs.lp'

scenfit = [error_measure_prg, min_weighted_error_prg, keep_inputs_prg]
mcos    = [add_influence_prg, min_added_influence_prg, keep_obs_prg]


def get_scenfit(instance, OS, FP, FC, EP):
  '''returns the scenfit of data and model described by the 
  ``TermSet`` object [instance].
  '''
  sem = [sign_cons_prg, bwd_prop_prg]
  if OS : sem.append(one_state_prg)
  if FP : sem.append(fwd_prop_prg)
  if FC : sem.append(founded_prg)
  if EP : sem.append(elem_path_prg)


  inst     = instance.to_file()
  prg      = sem + scenfit + [inst]
  coptions = '--opt-strategy=5'
  solver   = GringoClasp(clasp_options=coptions)
  solution = solver.run(prg,collapseTerms=True,collapseAtoms=False)
  opt      = solution[0].score[0]

  os.unlink(inst)
  return opt


def get_scenfit_labelings(instance,nm, OS, FP, FC, EP):
  '''
  returns a list of atmost [nm] ``TermSet`` representing scenfit labelings
  to the system described by the ``TermSet`` object [instance].
  '''
  sem = [sign_cons_prg, bwd_prop_prg]
  if OS : sem.append(one_state_prg)
  if FP : sem.append(fwd_prop_prg)
  if FC : sem.append(founded_prg)
  if EP : sem.append(elem_path_prg)
 
  inst     = instance.to_file()
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


def get_predictions_under_scenfit(instance, OS, FP, FC, EP):
  '''
  '''
  sem = [sign_cons_prg,bwd_prop_prg]
  if OS : sem.append(one_state_prg)
  if FP : sem.append(fwd_prop_prg)
  if FC : sem.append(founded_prg)
  if EP : sem.append(elem_path_prg)

  inst     = instance.to_file()
  prg      = sem + scenfit + [inst]
  coptions = '--opt-strategy=5'
  solver   = GringoClasp(clasp_options=coptions)
  solution = solver.run(prg,collapseTerms=True,collapseAtoms=False)
  opt      = solution[0].score[0]

  if OS :
    prg    = prg + [show_pred_prg]
  else :
    prg    = prg + [show_pred_dm_prg]
  coptions = '--opt-strategy=5 --opt-mode=optN --enum-mode=cautious --opt-bound='+str(opt)
  solver2  = GringoClasp(clasp_options=coptions)
  models   = solver2.run(prg,collapseTerms=True,collapseAtoms=False)

  os.unlink(inst)
  return models[0]


def get_mcos(instance, OS, FP, FC, EP):
  '''
  '''
  sem = [sign_cons_prg, bwd_prop_prg]
  if OS : sem.append(one_state_prg)
  if FP : sem.append(fwd_prop_prg)
  if FC : sem.append(founded_prg)
  if EP : sem.append(elem_path_prg)

  inst     = instance.to_file()
  #exit()
  prg      = sem + mcos + [inst]
  coptions = '--opt-strategy=5'
  solver   = GringoClasp(clasp_options=coptions)
  solution = solver.run(prg,collapseTerms=True,collapseAtoms=False)
  opt      = solution[0].score[0]

  os.unlink(inst) 
  return opt


def get_mcos_labelings(instance,nm, OS, FP, FC, EP):
  '''
  '''
  sem = [sign_cons_prg, bwd_prop_prg]
  if OS : sem.append(one_state_prg)
  if FP : sem.append(fwd_prop_prg)
  if FC : sem.append(founded_prg)
  if EP : sem.append(elem_path_prg)

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


def get_predictions_under_mcos(instance, OS, FP, FC, EP):
  '''
  '''
  sem = [sign_cons_prg,bwd_prop_prg]
  if OS : sem.append(one_state_prg)
  if FP : sem.append(fwd_prop_prg)
  if FC : sem.append(founded_prg)
  if EP : sem.append(elem_path_prg)

  inst     = instance.to_file()
  prg      = sem + mcos + [inst]
  coptions = '--opt-strategy=5'
  solver   = GringoClasp(clasp_options=coptions)
  solution = solver.run(prg,collapseTerms=True,collapseAtoms=False)
  opt      = solution[0].score[0]

  if OS :
    prg    = prg + [show_pred_prg]
  else :
    prg    = prg + [show_pred_dm_prg]

  coptions = '--opt-strategy=5 --opt-mode=optN --enum-mode=cautious --opt-bound='+str(opt)
  solver2  = GringoClasp(clasp_options=coptions)
  models   = solver2.run(prg,collapseTerms=True,collapseAtoms=False)

  os.unlink(inst)
  return models[0]


def get_opt_remove_edges(instance, OS, FP, FC, EP):

  sem = [sign_cons_prg,bwd_prop_prg]
  if OS : sem.append(one_state_prg)
  if FP : sem.append(fwd_prop_prg)
  if FC : sem.append(founded_prg)
  if EP : sem.append(elem_path_prg)

  inst     = instance.to_file()
  prg      = sem + scenfit + [remove_edges_prg, min_repairs_prg, inst ]
  coptions = '--opt-strategy=5'
  solver   = GringoClasp(clasp_options=coptions)
  solution = solver.run(prg,collapseTerms=True,collapseAtoms=False)
  fit      = solution[0].score[0]
  repairs  = solution[0].score[1]

  os.unlink(inst)
  return (fit,repairs)


def get_opt_repairs_remove_edges(instance,nm, OS, FP, FC, EP):

  sem = [sign_cons_prg,bwd_prop_prg]
  if OS : sem.append(one_state_prg)
  if FP : sem.append(fwd_prop_prg)
  if FC : sem.append(founded_prg)
  if EP : sem.append(elem_path_prg)

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


def get_opt_flip_edges(instance, OS, FP, FC, EP):

  sem = [sign_cons_prg,bwd_prop_prg]
  if OS : sem.append(one_state_prg)
  if FP : sem.append(fwd_prop_prg)
  if FC : sem.append(founded_prg)
  if EP : sem.append(elem_path_prg)

  inst     = instance.to_file()
  prg      = sem + scenfit + [flip_edges_prg, min_repairs_prg, inst ]
  coptions = '--opt-strategy=5'
  solver   = GringoClasp(clasp_options=coptions)
  solution = solver.run(prg,collapseTerms=True,collapseAtoms=False)
  fit      = solution[0].score[0]
  repairs  = solution[0].score[1]

  os.unlink(inst)
  return (fit,repairs)


def get_opt_repairs_flip_edges(instance,nm, OS, FP, FC, EP):

  sem = [sign_cons_prg,bwd_prop_prg]
  if OS : sem.append(one_state_prg)
  if FP : sem.append(fwd_prop_prg)
  if FC : sem.append(founded_prg)
  if EP : sem.append(elem_path_prg)

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


def get_opt_add_remove_edges_greedy(instance):
  '''
   only apply with elementary path consistency notion
  '''
  sem        = [sign_cons_prg, elem_path_prg, fwd_prop_prg, bwd_prop_prg]
  inst       = instance.to_file()
  prg        = [ inst, remove_edges_prg,
                 min_repairs_prg, show_rep_prg
               ] + sem + scenfit
  coptions   = '1 --project --opt-strategy=5 --opt-mode=optN --quiet=1'
  solver     = GringoClasp(clasp_options=coptions)
  models     = solver.run(prg, collapseTerms=True, collapseAtoms=False)

  bscenfit   = models[0].score[0]
  brepscore  = models[0].score[1]
  
  #print('model:   ',models[0])
  #print('bscenfit:   ',bscenfit)
  #print('brepscore:  ',brepscore)


  strt_edges = TermSet()
  fedges     = [(strt_edges, bscenfit, brepscore)]
  tedges     = []
  dedges     = []
  coptions   = '0 --project --opt-strategy=5 --opt-mode=optN --quiet=1'
  solver     = GringoClasp(clasp_options=coptions)

  while fedges:
#    sys.stdout.flush()
#    print ("TODO: ",len(fedges))
    (oedges, oscenfit, orepscore) = fedges.pop()

#    print('(oedges,oscenfit, orepscore):',(oedges,oscenfit, orepscore))
#    print('len(oedges):',len(oedges))

    # extend till no better solution can be found
    end       = True # assume this time its the end
    f_oedges  = TermSet(oedges).to_file()
    prg       = [ inst, f_oedges, remove_edges_prg, best_one_edge_prg,
                  min_repairs_prg, show_rep_prg
                ] + sem + scenfit
    models    = solver.run(prg, collapseTerms=True, collapseAtoms=False)
    nscenfit  = models[0].score[0]
    nrepscore = models[0].score[1]+2*(len(oedges))

#    print('nscenfit:   ',nscenfit)
#    print('nrepscore:  ',nrepscore)

    if (nscenfit < oscenfit) or nrepscore < orepscore: # better score or more that 1 scenfit
#      print('maybe better solution:')
#      print('#models: ',len(models))

      for m in models:
        #print('MMM   ',models)
        nend = TermSet()
        for a in m :
          if a.pred() == 'rep' :
            if a.arg(0)[0:7]=='addeddy' :
#              print('new addeddy to',a.arg(0)[8:-1])
              nend  = String2TermSet('edge_end('+(a.arg(0)[8:-1])+')')

              # search starts of the addeddy
#              print('search best edge starts')
              f_end  = TermSet(nend).to_file()

              prg    = [ inst, f_oedges, remove_edges_prg, f_end, best_edge_start_prg,
                         min_repairs_prg, show_rep_prg
                       ] + sem + scenfit
              starts = solver.run(prg, collapseTerms=True, collapseAtoms=False)
              os.unlink(f_end)
#              print(starts)
              for s in starts:
                n2scenfit  = s.score[0]
                n2repscore = s.score[1]+2*(len(oedges))
#                print('n2scenfit:   ', n2scenfit)
#                print('n2repscore:  ', n2repscore)

                if (n2scenfit < oscenfit) or n2repscore < orepscore: # better score or more that 1 scenfit
#                  print('better solution:')
                  if (n2scenfit<bscenfit):
                    bscenfit  = n2scenfit # update bscenfit
                    brepscore = n2repscore
                  if (n2scenfit == bscenfit) :
                    if (n2repscore<brepscore) : brepscore = n2repscore

                  nedge = TermSet()
                  for a in s :
                    if a.pred() == 'rep' :
                      if a.arg(0)[0:7]=='addedge' :
#                        print('new edge ',a.arg(0)[8:-1])
                        nedge = String2TermSet('obs_elabel('+(a.arg(0)[8:-1])+')')
                        end   = False

                  nedges = oedges.union(nedge)
                  if (nedges,n2scenfit,n2repscore) not in fedges and nedges not in dedges:
                    fedges.append((nedges,n2scenfit,n2repscore))
                    dedges.append(nedges)

    if end : 
      if (oedges,oscenfit,orepscore) not in tedges and oscenfit == bscenfit and orepscore == brepscore:
#        print('LAST tedges append',oedges)
        tedges.append((oedges,oscenfit,orepscore))

    # end while
    os.unlink(f_oedges)

  # take only the results with the best scenfit
  redges=[]
  for (tedges,tscenfit,trepairs) in tedges:
    if tscenfit == bscenfit: redges.append((tedges,trepairs))

  os.unlink(inst)
  return (bscenfit,redges)


def get_opt_repairs_add_remove_edges_greedy(instance,nm, edges):
  '''
   only apply with elementary path consistency notion
  '''

  sem      = [sign_cons_prg, elem_path_prg, fwd_prop_prg, bwd_prop_prg]
  inst     = instance.to_file()
  f_edges  = TermSet(edges).to_file()
  prg      = [ inst, f_edges, remove_edges_prg,
               min_repairs_prg, show_rep_prg,
             ] + sem + scenfit
  coptions = str(nm)+' --project --opt-strategy=5 --opt-mode=optN --quiet=1'
  solver   = GringoClasp(clasp_options=coptions)
  models   = solver.run(prg, collapseTerms=True, collapseAtoms=False)
  #print(models)
  #nscenfit  = models[0].score[0]
  #nrepscore = models[0].score[1]
  #print('scenfit:   ', nscenfit)
  #print('repscore:  ', nrepscore) 

  os.unlink(f_edges)
  os.unlink(inst)
  return models


def get_opt_add_remove_edges(instance, OS, FP, FC, EP):

  sem = [sign_cons_prg,bwd_prop_prg]
  if OS : sem.append(one_state_prg)
  if FP : sem.append(fwd_prop_prg)
  if FC : sem.append(founded_prg)
  if EP :
    print('error query.get_opt_add_remove_edges should not be called with'
          'elementary path constraint, use instead'
          'get_opt_add_remove_edges_greedy')

  inst     = instance.to_file()
  prg      = sem + scenfit + [remove_edges_prg, add_edges_prg, min_repairs_prg, inst ]
  coptions = '--opt-strategy=5'
  solver   = GringoClasp(clasp_options=coptions)
  solution = solver.run(prg,collapseTerms=True,collapseAtoms=False)
  fit      = solution[0].score[0]
  repairs  = solution[0].score[1]

  os.unlink(inst)
  return (fit,repairs)


def get_opt_repairs_add_remove_edges(instance,nm, OS, FP, FC,EP):

  sem = [sign_cons_prg,bwd_prop_prg]
  if OS : sem.append(one_state_prg)
  if FP : sem.append(fwd_prop_prg)
  if FC : sem.append(founded_prg)
  if EP : sem.append(elem_path_prg)

  inst     = instance.to_file()
  prg      = sem + scenfit + [remove_edges_prg, add_edges_prg, min_repairs_prg, inst ]
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


