0{flip(U,V,S)}1 :- not mandatory(U,V), obs_e_label(U,V,S).

% labels for fliped edges
remedge(U,V,S) :- flip(U,V,S).
elabel(V,U,S) :- flip(U,V,S).
