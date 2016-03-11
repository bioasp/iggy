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

  reserved = {
    'notPlus'  : 'NOTPLUS',
    'notMinus' : 'NOTMINUS',
    'input'    : 'INPUT',
    'MIN'      : 'MIN',
    'MAX'      : 'MAX'
  }

  tokens = [
    'IDENT',
    'EQ',
    'NULL',
    'PLUS',
    'MINUS'
    #'NOTPLUS',
    #'NOTMINUS',
    #'INPUT',
    #'MIN',
    #'MAX'
  ]+ list(reserved.values())

  # Token expressions

  def t_IDENT(self, t):
    r'[a-zA-Z][a-zA-Z0-9_:\-\[\]/]*'
    t.type = self.reserved.get(t.value,'IDENT')    # Check for reserved words
    return t

  t_EQ       = r'='
  t_NULL     = r'0'
  t_PLUS     = r'[1\+]+'
  t_MINUS    = r'-[1]*'
  t_MIN      = r'MIN'
  t_MAX      = r'MAX'


  def __init__(self):
    import pyasp.ply.lex as lex
    self.lexer = lex.lex(object = self, optimize=1, lextab='profile_parser_lextab')

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
    self.name             = "noname"
    self.aux_node_counter = 0
    self.accu             = TermSet()
    self.args             = []
    self.lexer            = Lexer()
    import pyasp.ply.yacc as yacc
    #self.parser = yacc.yacc(module=self, tabmodule='calc_parsetab', debugfile="calc_parser.out")
    self.parser           = yacc.yacc(module=self,optimize=1,debug=0, write_tables=0)

  def p_statement_expr(self, t):
    '''statement : null_assignment
    | plus_assignment
    | minus_assignment
    | notplus_assignment
    | notminus_assignment
    | input_assignment
    | min_assignment
    | max_assignment
    '''
    return


  def p_null_assignment(self, t):
    '''null_assignment : IDENT EQ NULL'''
    self.accu.add(Term('obs_vlabel', [self.name,"gen(\""+t[1]+"\")","0"]))
  
  def p_plus_assignment(self, t):
    '''plus_assignment : IDENT EQ PLUS'''
    self.accu.add(Term('obs_vlabel', [self.name,"gen(\""+t[1]+"\")","1"]))

  def p_minus_assignment(self, t):
    '''minus_assignment : IDENT EQ MINUS'''
    self.accu.add(Term('obs_vlabel', [self.name,"gen(\""+t[1]+"\")","-1"]))

  def p_notplus_assignment(self, t):
    '''notplus_assignment : IDENT EQ NOTPLUS'''
    self.accu.add(Term('obs_vlabel', [self.name,"gen(\""+t[1]+"\")","notPlus"]))

  def p_notminus_assignment(self, t):
    '''notminus_assignment : IDENT EQ NOTMINUS'''
    self.accu.add(Term('obs_vlabel', [self.name,"gen(\""+t[1]+"\")","notMinus"]))

  def p_input_assignment(self, t):
    '''input_assignment : IDENT EQ INPUT'''
    self.accu.add(Term('input', [self.name,"gen(\""+t[1]+"\")"]))

  def p_min_assignment(self, t):
    '''min_assignment : IDENT EQ MIN'''
    self.accu.add(Term('ismin', [self.name,"gen(\""+t[1]+"\")"]))

  def p_max_assignment(self, t):
    '''max_assignment : IDENT EQ MAX'''
    self.accu.add(Term('ismax', [self.name,"gen(\""+t[1]+"\")"]))

  def p_error(self, t):
    print("Syntax error at '",str(t),"'")

  def parse(self, line, name):
    self.name='\"'+name+'\"'
    #self.parser.name=name
    self.parser.parse(line, lexer=self.lexer.lexer)
    return self.accu


