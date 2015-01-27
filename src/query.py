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

"""
This module contains the queries which can be asked to the model and data.

"""
import os
import tempfile
from pyasp.asp import *

root = __file__.rsplit('/', 1)[0]
contradictory_obs_prg =	root +'/encodings/contradictory_obs.lp'
guess_inputs_prg =     	root + '/encodings/guess_inputs.lp'

sign_cons_prg   =      	root + '/encodings/sign-cons-3.lp'
constr_luca_prg =      	root + '/encodings/luca_constraints.lp'
constr_zero_prg =      	root + '/encodings/constrained_zero.lp'
founded_prg     =      	root + '/encodings/founded_constraints.lp'

keep_inputs_prg =      	root + '/encodings/keep_inputs_constraint.lp'
keep_obs_prg    =      	root + '/encodings/keep_observations_constraint.lp'

error_measure_prg      =  root + '/encodings/error_measure.lp'
min_weighted_error_prg =  root + '/encodings/minimize_weighted_error.lp'

add_influence_prg       = root + '/encodings/add_influence.lp'
min_added_influence_prg = root + '/encodings/minimize_added_influnces.lp'

add_edges_prg      =      root + '/encodings/add_edges.lp'
min_added_edges_prg =     root + '/encodings/minimize_added_edges.lp'

remove_edges_prg      =   root + '/encodings/remove_edges.lp'
max_removed_edges_prg =   root + '/encodings/maximize_removed_edges.lp'
min_removed_edges_prg =   root + '/encodings/minimize_removed_edges.lp'

min_repairs_prg =         root + '/encodings/minimize_repairs.lp'


mics_prg =		  root + '/encodings/mics.lp'
mics_constr_luca_prg =    root + '/encodings/mics_luca_constraints.lp'
mics_constr_zero_prg =    root + '/encodings/mics_constrained_zero.lp'
mics_founded_prg     =    root + '/encodings/mics_founded_constraints.lp'


heu_prg =	          root + '/encodings/heuristic.lp'

show_pred_prg =        root + '/encodings/show_predictions.lp'
show_labels_prg =      root + '/encodings/show_vlabels.lp'
show_err_prg =         root + '/encodings/show_errors.lp'
show_rep_prg =         root + '/encodings/show_repairs.lp'

scenfit = [error_measure_prg, min_weighted_error_prg, keep_inputs_prg]
mcos    = [add_influence_prg, min_added_influence_prg, keep_obs_prg]


def get_scenfit(instance, LucaConstraint=False, ConstrainedZero=True, FoundedConstraint=True):
    '''returns the scenfit of data and model described by the ``TermSet`` object [instance].
    '''
    sem=[sign_cons_prg]
    if LucaConstraint : sem.append(constr_luca_prg)
    if ConstrainedZero : sem.append(constr_zero_prg)
    if FoundedConstraint : sem.append(founded_prg)

    inst = instance.to_file()
    prg = sem + scenfit + [inst]
    coptions = '--opt-strategy=5'
    solver = GringoClasp(clasp_options=coptions)
    solution = solver.run(prg,collapseTerms=True,collapseAtoms=False)
    opt = solution[0].score[0]

    os.unlink(inst)
    return opt
    
def get_scenfit_labelings(instance,nm=1, LucaConstraint=False, ConstrainedZero=True, FoundedConstraint=True):
    '''
    returns a list of atmost [nm] ``TermSet`` representing scenfit labelings to the system described by the ``TermSet`` object [instance].
    '''
    sem=[sign_cons_prg]
    if LucaConstraint : sem.append(constr_luca_prg)
    if ConstrainedZero : sem.append(constr_zero_prg)
    if FoundedConstraint : sem.append(founded_prg)
    
    inst = instance.to_file()
    prg = sem + scenfit + [inst]
    coptions = '--opt-strategy=5'
    solver = GringoClasp(clasp_options=coptions)
    solution = solver.run(prg,collapseTerms=True,collapseAtoms=False)

    opt = solution[0].score[0]

    prg = prg + [show_labels_prg, show_err_prg]
    coptions = str(nm)+' --opt-strategy=5 --opt-mode=optN --opt-bound='+str(opt)
    solver2 = GringoClasp(clasp_options=coptions)
    models = solver2.run(prg,collapseTerms=True,collapseAtoms=False)
    
    os.unlink(inst)
    return models
     
def get_predictions_under_scenfit(instance, LucaConstraint=False, ConstrainedZero=True, FoundedConstraint=True):
    '''
    returns True if there exists a consistent extension
    to the system described by the ``TermSet`` object [instance].
    '''
    sem=[sign_cons_prg]
    if LucaConstraint : sem.append(constr_luca_prg)
    if ConstrainedZero : sem.append(constr_zero_prg)
    if FoundedConstraint : sem.append(founded_prg)

    inst = instance.to_file()
    prg = sem + scenfit + [inst]
    coptions = '--opt-strategy=5'
    solver = GringoClasp(clasp_options=coptions)
    solution = solver.run(prg,collapseTerms=True,collapseAtoms=False)

    opt = solution[0].score[0]

    prg = prg + [show_pred_prg]
    coptions = '--opt-strategy=5 --opt-mode=optN --enum-mode=cautious --opt-bound='+str(opt)
    solver2 = GringoClasp(clasp_options=coptions)
    models = solver2.run(prg,collapseTerms=True,collapseAtoms=False)

    os.unlink(inst)    
    return models[0]

def get_mcos(instance, LucaConstraint=False, ConstrainedZero=True, FoundedConstraint=True):
    '''
    returns True if there exists a consistent extension
    to the system described by the ``TermSet`` object [instance].
    '''
    sem=[sign_cons_prg]
    if LucaConstraint : sem.append(constr_luca_prg)
    if ConstrainedZero : sem.append(constr_zero_prg)
    if FoundedConstraint : sem.append(founded_prg)

    inst =   instance.to_file()
    prg = sem + mcos + [inst]
    coptions = '--opt-strategy=5'
    solver = GringoClasp(clasp_options=coptions)
    solution = solver.run(prg,collapseTerms=True,collapseAtoms=False)
    opt = solution[0].score[0]

    os.unlink(inst) 
    return opt

def get_mcos_labelings(instance,nm=1, LucaConstraint=False, ConstrainedZero=True, FoundedConstraint=True):
    '''
    returns True if there exists a consistent extension
    to the system described by the ``TermSet`` object [instance].
    '''
    sem=[sign_cons_prg]
    if LucaConstraint : sem.append(constr_luca_prg)
    if ConstrainedZero : sem.append(constr_zero_prg)
    if FoundedConstraint : sem.append(founded_prg)

    inst =   instance.to_file()
    prg = sem + mcos + [inst]
    coptions = '--opt-strategy=5'
    solver = GringoClasp(clasp_options=coptions)
    solution = solver.run(prg,collapseTerms=True,collapseAtoms=False)

    opt = solution[0].score[0]

    prg = prg + [show_labels_prg, show_rep_prg]
    coptions = str(nm)+' --opt-strategy=5 --opt-mode=optN --opt-bound='+str(opt)
    solver2 = GringoClasp(clasp_options=coptions)
    models = solver2.run(prg,collapseTerms=True,collapseAtoms=False)

    os.unlink(inst) 
    return models

def get_predictions_under_mcos(instance, LucaConstraint=False, ConstrainedZero=True, FoundedConstraint=True):
    '''
    returns ``TermSet`` if there exists a consistent extension
    to the system described by the ``TermSet`` object [instance].
    '''
    sem=[sign_cons_prg]
    if LucaConstraint : sem.append(constr_luca_prg)
    if ConstrainedZero : sem.append(constr_zero_prg)
    if FoundedConstraint : sem.append(founded_prg)

    inst = instance.to_file()
    prg = sem + mcos + [inst]
    coptions = '--opt-strategy=5'
    solver = GringoClasp(clasp_options=coptions)
    solution = solver.run(prg,collapseTerms=True,collapseAtoms=False)

    opt = solution[0].score[0]

    prg = prg + [show_pred_prg]
    coptions = '--opt-strategy=5 --opt-mode=optN --enum-mode=cautious --opt-bound='+str(opt)
    solver2 = GringoClasp(clasp_options=coptions)
    models = solver2.run(prg,collapseTerms=True,collapseAtoms=False)

    os.unlink(inst)
    return models[0]


def get_opt_remove_edges(instance, LucaConstraint=False, ConstrainedZero=True, FoundedConstraint=True):

    sem=[sign_cons_prg]
    if LucaConstraint : sem.append(constr_luca_prg)
    if ConstrainedZero : sem.append(constr_zero_prg)
    if FoundedConstraint : sem.append(founded_prg)

    inst = instance.to_file()
    prg = sem + scenfit + [remove_edges_prg, min_repairs_prg, inst ]
    coptions = '--opt-strategy=5'
    solver = GringoClasp(clasp_options=coptions)
    solution = solver.run(prg,collapseTerms=True,collapseAtoms=False)
    fit = solution[0].score[0]
    repairs = solution[0].score[1]

    os.unlink(inst)
    return (fit,repairs)


def get_opt_repairs_remove_edges(instance,nm=1, LucaConstraint=False, ConstrainedZero=True, FoundedConstraint=True):
  
    sem=[sign_cons_prg]
    if LucaConstraint : sem.append(constr_luca_prg)
    if ConstrainedZero : sem.append(constr_zero_prg)
    if FoundedConstraint : sem.append(founded_prg)

    inst = instance.to_file()
    prg = sem + scenfit + [remove_edges_prg, min_repairs_prg, inst ]

    coptions = '--opt-strategy=5'
    solver = GringoClasp(clasp_options=coptions)
    solution = solver.run(prg,collapseTerms=True,collapseAtoms=False)
    fit = solution[0].score[0]
    repairs = solution[0].score[1]

    prg = prg + [show_rep_prg]
    coptions = str(nm)+' --project --opt-strategy=5 --opt-mode=optN --opt-bound='+str(fit)+','+str(repairs)
    solver2 = GringoClasp(clasp_options=coptions)
    models = solver2.run(prg, collapseTerms=True, collapseAtoms=False)

    os.unlink(inst)
    return models
    
def get_opt_add_remove_edges(instance, LucaConstraint=False, ConstrainedZero=True, FoundedConstraint=True):

    sem=[sign_cons_prg]
    if LucaConstraint : sem.append(constr_luca_prg)
    if ConstrainedZero : sem.append(constr_zero_prg)
    if FoundedConstraint : sem.append(founded_prg)

    inst = instance.to_file()    
    prg = sem + scenfit + [remove_edges_prg, add_edges_prg, min_repairs_prg, inst ]
    coptions = '--opt-strategy=5'
    solver = GringoClasp(clasp_options=coptions)
    solution = solver.run(prg,collapseTerms=True,collapseAtoms=False)
    fit = solution[0].score[0]
    repairs = solution[0].score[1]

    os.unlink(inst)
    return (fit,repairs)

def get_opt_repairs_add_remove_edges(instance,nm=1, LucaConstraint=False, ConstrainedZero=True, FoundedConstraint=True):

    sem=[sign_cons_prg]
    if LucaConstraint : sem.append(constr_luca_prg)
    if ConstrainedZero : sem.append(constr_zero_prg)
    if FoundedConstraint : sem.append(founded_prg)

    inst = instance.to_file()    
    prg = sem + scenfit + [remove_edges_prg, add_edges_prg, min_repairs_prg, inst ]

    coptions = '--opt-strategy=5'
    solver = GringoClasp(clasp_options=coptions)
    solution = solver.run(prg,collapseTerms=True,collapseAtoms=False)
    fit = solution[0].score[0]
    repairs = solution[0].score[1]

    prg = prg + [show_rep_prg]
    coptions = str(nm)+' --project --opt-strategy=5 --opt-mode=optN --opt-bound='+str(fit)+','+str(repairs)
    solver2 = GringoClasp(clasp_options=coptions)
    models = solver2.run(prg, collapseTerms=True, collapseAtoms=False)

    os.unlink(inst)
    return models
    
def get_minimal_inconsistent_cores(instance, LucaConstraint=False, ConstrainedZero=True, FoundedConstraint=True):
    '''
    returns a list containing
    [nmodels] ``TermSet`` objects representing subset minimal inconsistent cores of the system
    described by the ``TermSet`` [instance]. The list [exclude] should contain ``TermSet`` objects
    representing (maybe partial) solutions that are to be avoided. If [nmodels] equals [0]
    then the list contain all feasible models.
    '''

    sem=[mics_prg, heu_prg]
    if LucaConstraint : sem.append(mics_constr_luca_prg)
    if ConstrainedZero : sem.append(mics_constr_zero_prg)
    #if FoundedConstraint : sem.append(mics_founded_prg)

    inst = instance.to_file()   
    prg = sem+ [inst]
    coptions='0 --dom-mod=6 --heu=Domain --enum-mode=record'
    solver = GringoClasp(clasp_options=coptions)
    models = solver.run(prg,collapseTerms=True, collapseAtoms=False)

    os.unlink(inst)    
    return models

def guess_inputs(instance):

    inst = instance.to_file()   
    prg = [ guess_inputs_prg, inst ]
    solver = GringoClasp()
    models = solver.run(prg, collapseTerms=True, collapseAtoms=False)
    os.unlink(inst)
    assert(len(models) == 1)

    return models[0]

def get_contradictory_obs(instance):

    inst = instance.to_file()
    prg = [ contradictory_obs_prg, inst ]
    solver = GringoClasp()
    models = solver.run(prg, collapseTerms=True, collapseAtoms=False)
    os.unlink(inst)
    assert(len(models) == 1)

    return models[0]

def get_reductions(instance):
    inst = instance.to_file()
    prg = [ reduction_prg, inst ]
    solver = GringoClasp()
    models = solver.run(prg,0)
    os.unlink(inst)
    assert(len(models) == 1)
    return models[0]


    
