% guess edges to add
0{rep(addedge(U,V,1)),rep(addedge(U,V,-1))}1 :- vertex(U), vertex(V), U!=V,
                                               not obs_elabel(U,V,1), not obs_elabel(U,V,-1).

% labels for newly added edgesm
elabel(U,V,1) :- rep(addedge(U,V,1)).
elabel(U,V,-1) :-rep(addedge(U,V,-1)).

