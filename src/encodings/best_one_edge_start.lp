
% guess one edge start to add
0{rep(addedge(gen(V),X,1)), rep(addedge(gen(V),X,-1))}1 :- vertex(gen(V)), edge_end(X).

% add only one edge !!!
:- rep(addedge(Y1,X,1)), rep(addedge(Y2,X,-1)).
:- rep(addedge(Y1,X,S)), rep(addedge(Y2,X,T)), Y1!=Y2.

% new inputs through repair

elabel(U,V,1) :- rep(addedge(U,V,1)).
elabel(U,V,-1) :-rep(addedge(U,V,-1)).
