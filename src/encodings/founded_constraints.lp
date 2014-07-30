
%founded rules

founded(E,X) :- input(E,X).
founded(E,X) :- founded(E,Y), vlabel(E,X, 1), vlabel(E,Y, 1), elabel(Y,X,1).
founded(E,X) :- founded(E,Y), vlabel(E,X, 1), vlabel(E,Y,-1), elabel(Y,X,-1).
founded(E,X) :- founded(E,Y), vlabel(E,X,-1), vlabel(E,Y, 1), elabel(Y,X,-1).
founded(E,X) :- founded(E,Y), vlabel(E,X,-1), vlabel(E,Y,-1), elabel(Y,X,1).

:- vlabel(E,V, 1), not founded(E,V).
:- vlabel(E,V,-1), not founded(E,V).