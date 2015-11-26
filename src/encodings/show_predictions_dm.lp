pred(E,X, 1) :- vlabel(E,X, 1), not vlabel(E,X,0), not vlabel(E,X,-1).
pred(E,X,-1) :- vlabel(E,X,-1), not vlabel(E,X,0), not vlabel(E,X, 1).
pred(E,X, 0) :- vlabel(E,X, 0), not vlabel(E,X,1), not vlabel(E,X,-1).

pred(E,V,notPlus)  :- vlabel(E,V, 0), vlabel(E,X,-1), not vlabel(E,X, 1).
pred(E,V,notMinus) :- vlabel(E,V, 0), vlabel(E,X, 1), not vlabel(E,X,-1).
pred(E,V,change)   :- vlabel(E,V,-1), vlabel(E,X, 1), not vlabel(E,X, 0).

#show pred/3.

