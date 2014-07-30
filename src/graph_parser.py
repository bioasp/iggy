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
from pyasp.asp import *
from pyasp.misc import *

class Lexer:
	tokens = (
		'IDENT',
		'ARROW',
		'PLUS',
		'MINUS',
		'AND',
		#'COMPL',
		#'OR',
		#'LP',
		#'RP',
		#'COM',
	)

	# Tokens

	t_IDENT = r'[a-zA-Z][a-zA-Z0-9_:\-\[\]/]*'
	t_ARROW = r'->'
	t_PLUS = r'\+'
	t_MINUS = r'-'
	t_AND = r'\?'
	t_COMPL = r'&'
	t_OR = r'\|'
	t_LP = r'\('
	t_RP = r'\)'
	t_COM = r','

	def __init__(self):
		import pyasp.ply.lex as lex
		self.lexer = lex.lex(object = self, optimize=1, lextab='graph_parser_lextab')

	# Ignored characters
	t_ignore = " \t"

	def t_newline(self, t):
		r'\n+'
		t.lexer.lineno += t.value.count("\n")
		
	def t_error(self, t):
		print "Illegal character '%s'" % t.value[0]
		t.lexer.skip(1)


class Parser:
	tokens = Lexer.tokens

	precedence = ( )
		#('left','PLUS','MINUS'),
		#('left','TIMES','DIVIDE'),
		#('right','UMINUS'),
		#)

	def __init__(self):
		self.aux_node_counter=0
		self.accu = TermSet()
		self.fun_flag = False
		self.args = []
		self.commas = []
		self.lexer = Lexer()
		import pyasp.ply.yacc as yacc
		#self.parser = yacc.yacc(module=self, debug=False, tabmodule='calc_parsetab', debugfile="calc_parser.out")
		self.parser = yacc.yacc(module=self, optimize=1, tabmodule='graph_parser_parsetab',)

	def p_statement_expr(self, t):
		'''statement : node_expression ARROW node_expression value
					| node_expression '''
		if len(t)<3 : 
			self.accu.add(Term('input', [t[1]]))
		else :
			self.accu.add(Term('edge', [t[1],t[3]]))
			if t[4]!="0" : self.accu.add(Term('obs_elabel', [t[1],t[3],t[4]]))
			#self.names[t[0]] = t[2]

	def p_node_expression(self, t):
		'''node_expression : IDENT '''

		if len(t)<3 : 
			
			if self.fun_flag==True :
				t[0]=t[1]
				self.fun_flag=False
			else :
				t[0] = "gen(\""+t[1]+"\")"
				self.accu.add(Term('vertex', ["gen(\""+t[1]+"\")"]))
		elif t[1] == 'opposite_sign' :
			self.aux_node_counter+=1
			aux_node = "aux_node_"+str(self.aux_node_counter)
			t[0] = "opposite_sign"+"("+aux_node+")"
			self.accu.add(Term('edge', [t[3],t[1]+"("+aux_node+")"]))
			self.accu.add(Term('obs_elabel', [t[3],t[1]+"("+aux_node+")","1"]))
		elif t[1] == 'strong_inhibitor' :
			t[0] = "strong_inhibitor"+"("+t[3]+")"
			for i in self.commas : 
				self.accu.add(Term('edge', [i,t[1]+"("+t[3]+")"]))
				self.accu.add(Term('obs_elabel', [i,t[1]+"("+t[3]+")","1"]))
			self.commas= []
		else : t[0] = "unknown"

	def p_value(self, t):
		'''value : PLUS
		       | MINUS
		       | AND '''
		if t[1] == '-' : t[0] = "-1"
		elif t[1] == '+' : t[0] = "1"
		elif t[1] == '?' : t[0] = "0"

			
	def p_error(self, t):
		print "Syntax error at '%s'" % t

	def parse(self, line):
		self.parser.parse(line, lexer=self.lexer.lexer)
		return self.accu
