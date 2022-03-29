% guess edges to add
0{addedge(or(U),or(V),1); addedge(or(U),or(V),-1)}1 :- vertex(or(U)), vertex(or(V)), U!=V,
                                               not obs_e_label(or(U),or(V),1), not obs_e_label(or(U),or(V),-1).

% labels for newly added edgesm
elabel(U,V,1) :- addedge(U,V,1).
elabel(U,V,-1) :-addedge(U,V,-1).
