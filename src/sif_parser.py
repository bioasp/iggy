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
    'NOT',
    'AND',
    'LB',
    'RB',
    'PLUS',
    'MINUS',
  )
  
  # Tokens
  
  t_IDENT = r'[a-zA-Z][a-zA-Z0-9_:\-\[\]/]*'
  t_NOT   = r'!'
  t_AND   = r'&'
  t_LB    = r'\('
  t_RB    = r'\)'
  t_PLUS  = r'1'
  t_MINUS = r'-1'


  def __init__(self):
    import pyasp.ply.lex as lex
    self.lexer = lex.lex(object = self, optimize=1, lextab='sif_parser_lextab')

  # Ignored characters
  t_ignore = " \t"

  def t_newline(self, t):
    r'\n+'
    t.lexer.lineno += t.value.count("\n")

  def t_error(self, t):
    print("Illegal character '",str(t.value[0]),"'", sep='')
    t.lexer.skip(1)


class Parser:
  tokens     = Lexer.tokens
  precedence = ( )

  def __init__(self):
    self.accu   = TermSet()
    self.args   = []
    self.lexer  = Lexer()
    import pyasp.ply.yacc as yacc
    self.parser = yacc.yacc(module=self, tabmodule='calc_parsetab', debugfile="calc_parser.out")
    #self.parser = yacc.yacc(module=self,optimize=1,debug=0, write_tables=0)

  def p_statement_expr(self, t):
    '''statement : node_expression PLUS node_expression 
                 | node_expression MINUS node_expression
                 | node_expression PLUS andnode_expression
                 | node_expression MINUS andnode_expression
                 | andnode_expression PLUS node_expression
                 | andnode_expression MINUS node_expression  
                 | andnode_expression PLUS andnode_expression
                 | andnode_expression MINUS andnode_expression                   
                 '''
    if len(t)<3 : 
      self.accu.add(Term('input', [t[1]]))
      print('input', t[1])
    else:
      #print t[1], t[2], t[3]
      self.accu.add(Term('edge', [t[1],t[3]]))
      self.accu.add(Term('obs_elabel', [t[1],t[3],t[2]]))
      #print Term('obs_elabel', ["gen(\""+t[1]+"\")","gen(\""+t[3]+"\")",t[2]])


  def p_andnode_expression(self, t):
    '''andnode_expression : LB  identlist RB '''
    self.accu.add(Term('vertex', ["and(\""+t[2]+"\")"]))
    t[0] = "and(\""+t[2]+"\")"

    
  def p_identlist(self, t):
    '''identlist : IDENT
                  | NOT IDENT
                  | IDENT AND identlist
                  | NOT IDENT AND identlist              
                  '''
    if len(t)==5 : 
      #print(t[1],t[2],t[3],t[4])
      t[0] = t[1]+t[2]+t[3]+t[4]
    elif len(t)==4 : 
      #print(t[1],t[2],t[3])
      t[0] = t[1]+t[2]+t[3]
    elif len(t)==3 : 
      #print(t[1],t[2])
      t[0] = t[1]+t[2]
    elif len(t)==2 :
      #print(t[0],t[1])
      t[0]=t[1]
    else:
      print("Syntax error at '",str(t),"'")
  

  def p_node_expression(self, t):
    '''node_expression : IDENT'''
    self.accu.add(Term('vertex', ["gen(\""+t[1]+"\")"]))
    t[0] = "gen(\""+t[1]+"\")"
  
  		
  def p_error(self, t):
    print("Syntax error at '",str(t),"'")
  
  def parse(self, line):
    self.parser.parse(line, lexer=self.lexer.lexer)
    return self.accu


