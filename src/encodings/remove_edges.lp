0{remedge(U,V,S)}1 :- not mandatory(U,V), obs_e_label(U,V,S).
0{remedge(U,V,1); remedge(U,V,-1)}2 :- not mandatory(U,V), edge(U,V), not obs_e_label(U,V,1), not obs_e_label(U,V,-1).
