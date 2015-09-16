# Copyright (c) 2015, Sven Thiele <sthiele78@gmail.com>
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
import re
from pyasp.asp import *
from pyasp.misc import *

import pyasp.ply.lex as lex
import pyasp.ply.yacc as yacc

import __iggy__.sif_parser as sif_parser
import __iggy__.profile_parser as profile_parser


def readSIFGraph(filename):
  """
  input: string, name of a file containing a graph description
  output: asp.TermSet, with atoms matching the contents of the input file
  
  Parses a Simple Interaction Format (SIF) graph description, and returns
  a TermSet object.
  """
  p    = sif_parser.Parser()
  accu = TermSet()
  file = open(filename,'r')
  s    = file.readline()
  while s!="":
    try:
      accu = p.parse(s)
    except EOFError:
      break
    s = file.readline()

  return accu


def readProfile(filename):
  """
  input: string, name of a file containing a profile description
  output: asp.TermSet, with atoms matching the contents of the input file

  Parses a profile description, and returns
  a TermSet object.
  """
  p    = profile_parser.Parser()
  accu = TermSet()
  file = open(filename,'r')
  s    = file.readline()
  while s!="":
    try:
      accu = p.parse(s,filename)
    except EOFError:
      break
    s = file.readline()

  return accu


