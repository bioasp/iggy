0{rep(remedge(U,V,S))}1 :- not mandatory(U,V), obs_elabel(U,V,S).
0{rep(remedge(U,V,1)), rep(remedge(U,V,-1))}2 :- not mandatory(U,V), edge(U,V), not obs_elabel(U,V,1), not obs_elabel(U,V,-1).