% keep observed variations
forbidden(E,V,T) :- obs_v_label(E,V,S), sign(S), sign(T), S!=T.

% A weak vertex variation has been observed
forbidden(E,V, 1) :- vertex(V), obs_v_label(E,V,notPlus).
forbidden(E,V,-1) :- vertex(V), obs_v_label(E,V,notMinus).
