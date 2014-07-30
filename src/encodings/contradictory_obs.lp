 
contradiction(E,X) :- obs_vlabel(E,X,1),obs_vlabel(E,X,0).
contradiction(E,X) :- obs_vlabel(E,X,-1),obs_vlabel(E,X,0).
contradiction(E,X) :- obs_vlabel(E,X,-1),obs_vlabel(E,X,1).
contradiction(E,X) :- obs_vlabel(E,X,-1),obs_vlabel(E,X,1).
contradiction(E,X) :- obs_vlabel(E,X,notMinus),obs_vlabel(E,X,-1).
contradiction(E,X) :- obs_vlabel(E,X,notPlus),obs_vlabel(E,X,1).


#hide.
#show contradiction/2.