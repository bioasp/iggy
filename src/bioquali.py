# Copyright (c) 2012, Sven Thiele <sthiele78@gmail.com>
#
# This file is part of ingranalyze.
#
# ingranalyze is free software: you can redistribute it and/or modify
# it under the terms of the GNU General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# ingranalyze is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU General Public License for more details.
#
# You should have received a copy of the GNU General Public License
# along with ingranalyze.  If not, see <http://www.gnu.org/licenses/>.
# -*- coding: utf-8 -*-
import re
from pyasp.asp import *
from pyasp.misc import *

import pyasp.ply.lex as lex
import pyasp.ply.yacc as yacc

import graph_parser
import sif_parser
import profile_parser


def parse_val(s):
    if s == '+': return '1'
    elif s == '-': return '-1'
    elif s == '0': return '0'
    elif s == 'nc': return '0'
    elif s == 'notPlus': return 'notPlus'    
    elif s == 'notMinus': return 'notMinus'        
    elif s == 'input': return 'input'
    else: 
        print s
        assert(False)


def readSIFGraph(filename):
    p = sif_parser.Parser()
    """
    input: string, name of a file containing a Bioquali-like graph description
    output: asp.TermSet, with atoms matching the contents of the input file
    
    Parses a Bioquali-like graph description, and returns
    a TermSet object.
    Written using original Bioquali
    """
	
    accu = TermSet()
    file = open(filename,'r')
    s = file.readline()
    while s!="":
        try:
            accu = p.parse(s)
        except EOFError:
            break
        s = file.readline()

    return accu



def readProfile_new(filename):
    p = profile_parser.Parser()
    """
    input: string, name of a file containing a Bioquali-like graph description
    output: asp.TermSet, with atoms matching the contents of the input file
    
    Parses a Bioquali-like graph description, and returns
    a TermSet object.
    Written using original Bioquali
    """
	
    accu = TermSet()
    file = open(filename,'r')
    s = file.readline()
    while s!="":
        if s!="\n":
	    try:
		accu = p.parse(s)
	    except EOFError:
		break
	s = file.readline()

    return accu
    
def readProfile(filename):
    GENE_ID = '[-a-zA-Z0-9_:\(\)/]+'
    VAL = '(-|\+|0|nc|input|notPlus|notMinus)'
    file = open(filename,'r')
    val_re = '(?P<genid>'+GENE_ID+')(\s)*=(\s)*(?P<sign>'+VAL+')'
    val = re.compile(val_re)
    line_number = 1
    line = file.readline()
    name = filename
    name = quote(name)
    accu = TermSet()
    accu.add(Term('exp',[name]))
    while line:
        vm = val.match(line)
        if vm:
	    if parse_val(vm.group('sign'))=='input':
	      vertex = quote(vm.group('genid'))
	      accu.add(Term('input',[name, "gen("+vertex+")"]))
	    else:
	      vertex = quote(vm.group('genid'))
	      accu.add(Term('obs_vlabel',[name, "gen("+vertex+")", parse_val(vm.group('sign'))]))
        else:
            print 'Syntax error line:', line_number,'  '+line 
        line = file.readline()
        line_number+=1
    return accu


                
