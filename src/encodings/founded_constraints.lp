
%founded rules

founded(E,X,-1) :- input(E,X).
founded(E,X, 1) :- input(E,X).
founded(E,X,-1) :- founded(E,Y,-1), elabel(Y,X, 1).
founded(E,X,-1) :- founded(E,Y, 1), elabel(Y,X,-1).
founded(E,X, 1) :- founded(E,Y,-1), elabel(Y,X,-1).
founded(E,X, 1) :- founded(E,Y, 1), elabel(Y,X, 1).

founded(E,X,S) :- vlabel(E,X,S), rep(new_influence(E,X,S)).

forbidden(E,V, 1):- exp(E), vertex(V), not founded(E,V, 1).
forbidden(E,V,-1):- exp(E), vertex(V), not founded(E,V,-1).
