
% guess one edge start to add
0{rep(addedge(gen(V),X,1)), rep(addedge(gen(V),X,-1))}1 :- vertex(gen(V)), edge_end(X).

% new inputs through repair

elabel(U,V,1) :- rep(addedge(U,V,1)).
elabel(U,V,-1) :-rep(addedge(U,V,-1)).
