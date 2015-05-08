 
% two contradictory observations
contradiction(E,X) :- obs_vlabel(E,X,1),obs_vlabel(E,X,0).
contradiction(E,X) :- obs_vlabel(E,X,-1),obs_vlabel(E,X,0).
contradiction(E,X) :- obs_vlabel(E,X,-1),obs_vlabel(E,X,1).
contradiction(E,X) :- obs_vlabel(E,X,notMinus),obs_vlabel(E,X,-1).
contradiction(E,X) :- obs_vlabel(E,X,notPlus),obs_vlabel(E,X,1).

% contradictions of observed behavior and initial level
contradiction(E,X) :- obs_vlabel(E,X,-1),ismin(E,X).
contradiction(E,X) :- obs_vlabel(E,X,1),ismax(E,X).


#hide.
#show contradiction/2.