% special case: repairs with remove edges and one added edge

0{rep(remedge(U,V,S))}1 :- not mandatory(U,V), obs_elabel(U,V,S).
0{rep(remedge(U,V,1)), rep(remedge(U,V,-1))}2 :- not mandatory(U,V), edge(U,V), not obs_elabel(U,V,1), not obs_elabel(U,V,-1).

% guess edges to add
0{rep(addedge(gen(U),gen(V),1)),rep(addedge(gen(U),gen(V),-1))}1 :-
     vertex(gen(U)), vertex(gen(V)), U!=V,
     not obs_elabel(gen(U),gen(V),1), not obs_elabel(gen(U),gen(V),-1).

% labels for newly added edgesm
elabel(U,V,1) :- rep(addedge(U,V,1)).
elabel(U,V,-1) :-rep(addedge(U,V,-1)).

% one_add_edge
:- rep(addedge(U1,V1,S1)), rep(addedge(U2,V2,S2)) , U1!=U2.
:- rep(addedge(U1,V1,S1)), rep(addedge(U2,V2,S2)) , V1!=V2.
:- rep(addedge(U1,V1,S1)), rep(addedge(U2,V2,S2)) , S1!=S2.
