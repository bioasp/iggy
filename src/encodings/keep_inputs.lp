% keep observed input variation
forbidden(E,V,T) :- input(E,V), obs_v_label(E,V,S), sign(S), sign(T), S!=T.

% A weak input
forbidden(E,V, 1) :- input(E,V), obs_v_label(E,V,notPlus).
forbidden(E,V,-1) :- input(E,V), obs_v_label(E,V,notMinus).
