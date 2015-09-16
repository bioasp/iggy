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
import os

def print_predictions(predictions) :
  predictions = sorted(predictions, key=lambda p: str(p.arg(0)))
  exp            = ''
  pred_plus      = set()
  pred_minus     = set()
  pred_zero      = set()
  pred_not_plus  = set()
  pred_not_minus = set()
  pred_change    = set()
  for p in predictions:
    if p.pred() == "new_pred" :
      if p.arg(2) == "1"        : pred_plus.add(p.arg(1))
      if p.arg(2) == "-1"       : pred_minus.add(p.arg(1))
      if p.arg(2) == "0"        : pred_zero.add(p.arg(1))
      if p.arg(2) == "notPlus"  : pred_not_plus.add(p.arg(1))
      if p.arg(2) == "notMinus" : pred_not_minus.add(p.arg(1))
      if p.arg(2) == "change"   : pred_change.add(p.arg(1))

  pred_not_plus.difference_update(pred_minus)
  pred_not_plus.difference_update(pred_zero)
  pred_not_minus.difference_update(pred_plus)
  pred_not_minus.difference_update(pred_zero)
  pred_change.difference_update(pred_minus)
  pred_change.difference_update(pred_plus)
  for p in pred_plus      : print('  ',p,'= +')
  for p in pred_minus     : print('  ',p,'= -')
  for p in pred_zero      : print('  ',p,'= 0')
  for p in pred_not_plus  : print('  ',p,'= NOT +')
  for p in pred_not_minus : print('  ',p,'= NOT -')
  for p in pred_change    : print('  ',p,'= CHANGE')

  print(' \n   predicted +:', len(pred_plus),
             ' predicted -:', len(pred_minus),
             ' predicted 0:', len(pred_zero),
         ' predicted NOT +:', len(pred_not_plus),
         ' predicted NOT -:', len(pred_not_minus),
        ' predicted CHANGE:', len(pred_change))


def print_labeling(labelings) :
  labelings   = sorted(labelings, key=lambda p: str(p.arg(0)))
  label_plus  = set()
  label_minus = set()
  label_zero  = set()
  repairs     = set()
  for l in labelings:
    if l.pred() == "vlabel" :
      if l.arg(2) == "1"  : label_plus.add(l.arg(1))
      if l.arg(2) == "-1" : label_minus.add(l.arg(1))
      if l.arg(2) == "0"  : label_zero.add(l.arg(1))
    if l.pred() == "err" : repairs.add(l)
    if l.pred() == "rep" : repairs.add(l)

  for l in label_plus  : print('  ',l,'= +')
  for l in label_minus : print('  ',l,'= -')
  for l in label_zero  : print('  ',l,'= 0')
  
  print(' \n   labeled +:', len(label_plus),
             ' labeled -:', len(label_minus),
             ' labeled 0:', len(label_zero),end='')

  for r in repairs     : print('   ',r.arg(0))


def print_repairs(repairs) :
  repair = set()
  for r in repairs:
    if r.pred() == "rep" : repair.add(r)
  for r in repair : print('   ',r.arg(0))


def print_mic(mic, net, obs) :

  nodes = set()
  edges = []
  for node in mic: nodes.add(node.arg(0))

  predecessors = set()
  for e in net:
    if e.pred() == "obs_elabel" :
      if e.arg(1) in nodes :
        predecessors.add(e.arg(0))
        if e.arg(2) == "1"  : edges.append(str(e.arg(0))+" -> "+str(e.arg(1))+" +")
        if e.arg(2) == "-1" : edges.append(str(e.arg(0))+" -> "+str(e.arg(1))+" -")
  for edge in edges: print('   '+edge)
  for o in obs:
    if o.pred() == "obs_vlabel" :
      if o.arg(1) in nodes :
        if o.arg(2) == "1"  : print('  ', o.arg(1), "= +")
        if o.arg(2) == "-1" : print('  ', o.arg(1), "= -")
      if o.arg(1) in predecessors :
        if o.arg(2) == "1"  : print('  ', o.arg(1), "= +")
        if o.arg(2) == "-1" : print('  ', o.arg(1), "= -")


def clean_up() :
  if os.path.isfile("parser.out")              : os.remove("parser.out")
  if os.path.isfile("asp_py_lextab.py")        : os.remove("asp_py_lextab.py")
  if os.path.isfile("asp_py_lextab.pyc")       : os.remove("asp_py_lextab.pyc")
  if os.path.isfile("asp_py_parsetab.py")      : os.remove("asp_py_parsetab.py")
  if os.path.isfile("asp_py_parsetab.pyc")     : os.remove("asp_py_parsetab.pyc")
  if os.path.isfile("graph_parser_lextab.py")  : os.remove("graph_parser_lextab.py")
  if os.path.isfile("graph_parser_parsetab.py"): os.remove("graph_parser_parsetab.py")
  if os.path.isfile("parsetab.py")             : os.remove("parsetab.py")
  if os.path.isfile("parsetab.pyc")            : os.remove("parsetab.pyc")
  if os.path.isfile("sif_parser_lextab.py")    : os.remove("sif_parser_lextab.py")
  if os.path.isfile("sif_parser_lextab.pyc")   : os.remove("sif_parser_lextab.pyc")


