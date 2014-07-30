 
 % keep observed variations
vlabel(E,V,-1) :- vertex(V), obs_vlabel(E,V,-1).
vlabel(E,V, 1) :- vertex(V), obs_vlabel(E,V, 1).
vlabel(E,V, 0) :- vertex(V), obs_vlabel(E,V, 0).


% A weak vertex variation has been observed
1{vlabel(E,V,-1), vlabel(E,V,0)}1 :- vertex(V), obs_vlabel(E,V,notPlus).
1{vlabel(E,V, 1), vlabel(E,V,0)}1 :- vertex(V), obs_vlabel(E,V,notMinus).

