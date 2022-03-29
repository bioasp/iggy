flip_node_sign_Plus_to_0(E,X) :- obs_v_label(E,X, 1), not vlabel(E,X, 1),     vlabel(E,X,0).
flip_node_sign_Plus_to_Minus(E,X) :- obs_v_label(E,X, 1), not vlabel(E,X, 1), not vlabel(E,X,0), vlabel(E,X,-1).

flip_node_sign_Minus_to_0(E,X) :- obs_v_label(E,X,-1), not vlabel(E,X,-1),     vlabel(E,X,0).
flip_node_sign_Minus_to_Plus(E,X) :- obs_v_label(E,X,-1), not vlabel(E,X,-1), not vlabel(E,X,0), vlabel(E,X, 1).

flip_node_sign_0_to_Plus(E,X) :- obs_v_label(E,X, 0), not vlabel(E,X, 0), vlabel(E,X, 1).
flip_node_sign_0_to_Minus(E,X) :- obs_v_label(E,X, 0), not vlabel(E,X, 0), vlabel(E,X, -1).

flip_node_sign_notMinus_to_Minus(E,X) :- obs_v_label(E,X, notMinus), not vlabel(E,X, 0), not vlabel(E,X, 1).
flip_node_sign_notPlus_to_Plus(E,X) :- obs_v_label(E,X, notPlus), not vlabel(E,X, 0), not vlabel(E,X,-1).
