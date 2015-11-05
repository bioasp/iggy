0{rep(flip(U,V,S))}1 :- not mandatory(U,V), obs_elabel(U,V,S).


% labels for fliped edges
rep(remedge(U,V,S)) :- rep(flip(U,V,S)).
elabel(V,U,S) :- rep(flip(U,V,S)).

