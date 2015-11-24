% input facts
% obs_vlabel(Experiment, Vertex , Sign)
% edge(X,Y)
% obs_elabel(X,Y,Sign).
% vertex(Name)

sign(1;-1;0).
obs(1;-1;0;notPlus;notMinus).
exp(E) :- obs_vlabel(E,V,S).
exp(E) :- input(E,V).
vertex(V) :- obs_vlabel(E,V,S).
edge(X,Y) :- obs_elabel(X,Y,S).
vertex(X) :- edge(X,Y).
vertex(Y) :- edge(X,Y).



% for each vertex the measurements are either changing (1,-1) or not (0)
1{vlabel(E,V,1), vlabel(E,V,-1), vlabel(E,V,0)} :- vertex(V), exp(E).
1 {elabel(U,V,1), elabel(U,V,-1)} 1 :- edge(U,V), not rep(remedge(U,V,1)), not rep(remedge(U,V,-1)).

% keep observed labeling of the edges
error_edge(U,V) :- obs_elabel(U,V,1), obs_elabel(U,V,-1).
elabel(U,V,S) :- edge(U,V), obs_elabel(U,V,S), not error_edge(U,V), not rep(remedge(U,V,S)).

% how to hande error_edges
elabel(U,V,1)  :- edge(U,V), obs_elabel(U,V,1), obs_elabel(U,V,-1), not rep(remedge(U,V,1)), rep(remedge(U,V,-1)).
elabel(U,V,-1) :- edge(U,V), obs_elabel(U,V,1), obs_elabel(U,V,-1), rep(remedge(U,V,1)), not rep(remedge(U,V,-1)).

 
% levelbound constraints
% if the initial level is at the minimum it cannot decrease anymore
:- ismin(E,X), vlabel(E,X,-1).

% if the initial level is at the maximum it cannot increase anymore
:- ismax(E,X), vlabel(E,X,1).
 
#hide.
