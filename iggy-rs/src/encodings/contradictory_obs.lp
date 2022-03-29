% two contradictory observations
contradiction1(E,X) :- obs_v_label(E,X,1), obs_v_label(E,X,0).
contradiction2(E,X) :- obs_v_label(E,X,-1), obs_v_label(E,X,0).
contradiction3(E,X) :- obs_v_label(E,X,-1), obs_v_label(E,X,1).
contradiction4(E,X) :- obs_v_label(E,X,notMinus), obs_v_label(E,X,-1).
contradiction5(E,X) :- obs_v_label(E,X,notPlus), obs_v_label(E,X,1).

% contradictions of observed behavior and initial level
contradiction6(E,X) :- obs_v_label(E,X,-1), is_min(E,X).
contradiction7(E,X) :- obs_v_label(E,X,1), is_max(E,X).

#show contradiction1/2.
#show contradiction2/2.
#show contradiction3/2.
#show contradiction4/2.
#show contradiction5/2.
#show contradiction6/2.
#show contradiction7/2.
