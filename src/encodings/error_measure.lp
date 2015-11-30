
err(flip(E,X,1)) :- obs_vlabel(E,X, 1), not vlabel(E,X, 1),     vlabel(E,X,0).
err(flip(E,X,2)) :- obs_vlabel(E,X, 1), not vlabel(E,X, 1), not vlabel(E,X,0), vlabel(E,X,-1).

err(flip(E,X,1)) :- obs_vlabel(E,X,-1), not vlabel(E,X,-1),     vlabel(E,X,0).
err(flip(E,X,2)) :- obs_vlabel(E,X,-1), not vlabel(E,X,-1), not vlabel(E,X,0), vlabel(E,X, 1).

err(flip(E,X,1)) :- obs_vlabel(E,X, 0), not vlabel(E,X, 0).

err(flip(E,X,2)) :- obs_vlabel(E,X, notMinus), not vlabel(E,X, 0), not vlabel(E,X, 1).
err(flip(E,X,2)) :- obs_vlabel(E,X, notPlus),  not vlabel(E,X, 0), not vlabel(E,X,-1).

