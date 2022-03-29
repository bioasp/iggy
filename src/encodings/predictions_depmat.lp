pred(E,X, 1) :- vlabel(E,X, 1), not vlabel(E,X,0), not vlabel(E,X,-1).
pred(E,X,-1) :- vlabel(E,X,-1), not vlabel(E,X,0), not vlabel(E,X, 1).
pred(E,X, 0) :- vlabel(E,X, 0), not vlabel(E,X,1), not vlabel(E,X,-1).

pred(E,X,notPlus)  :- vlabel(E,X, 0), vlabel(E,X,-1), not vlabel(E,X, 1).
pred(E,X,notMinus) :- vlabel(E,X, 0), vlabel(E,X, 1), not vlabel(E,X,-1).
pred(E,X,change)   :- vlabel(E,X,-1), vlabel(E,X, 1), not vlabel(E,X, 0).

#show pred/3.
