%special case: check elementary path consistency with remove edges and bound add edges

0{rep(remedge(U,V,S))}1 :- not mandatory(U,V), obs_elabel(U,V,S).
0{rep(remedge(U,V,1)), rep(remedge(U,V,-1))}2 :- not mandatory(U,V), edge(U,V), not obs_elabel(U,V,1), not obs_elabel(U,V,-1).

% guess edges to add
0{rep(addedge(gen(U),gen(V),1)),rep(addedge(gen(U),gen(V),-1))}1 :-
     vertex(gen(U)), vertex(gen(V)), U!=V,
     not obs_elabel(gen(U),gen(V),1), not obs_elabel(gen(U),gen(V),-1).

% labels for newly added edgesm
elabel(U,V,1) :- rep(addedge(U,V,1)).
elabel(U,V,-1) :-rep(addedge(U,V,-1)).

% one_add_edges(M). is given

:- C = #count{rep(addedge(U,V,S))}, max_add_edges(M), C>M.