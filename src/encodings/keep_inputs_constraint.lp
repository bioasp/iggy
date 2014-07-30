% keep observed input variation
vlabel(E,V,-1) :- vertex(V), obs_vlabel(E,V,-1), input(E,V).
vlabel(E,V, 1) :- vertex(V), obs_vlabel(E,V, 1), input(E,V).
vlabel(E,V, 0) :- vertex(V), obs_vlabel(E,V, 0), input(E,V).

% A weak input
1{vlabel(E,V,-1), vlabel(E,V,0)}1 :- vertex(V), obs_vlabel(E,V,notPlus), input(E,V).
1{vlabel(E,V, 1), vlabel(E,V,0)}1 :- vertex(V), obs_vlabel(E,V,notMinus), input(E,V).
