%founded constraint
founded(X) :- input(X).
founded(X) :- founded(Y), vlabel(X, 1), vlabel(Y, 1), elabel(Y,X,1).
founded(X) :- founded(Y), vlabel(X, 1), vlabel(Y,-1), elabel(Y,X,-1).
founded(X) :- founded(Y), vlabel(X,-1), vlabel(Y, 1), elabel(Y,X,-1).
founded(X) :- founded(Y), vlabel(X,-1), vlabel(Y,-1), elabel(Y,X,1).

bot :- active(V), vlabel(V, 1), not founded(E,V).
bot :- active(V), vlabel(V,-1), not founded(E,V).