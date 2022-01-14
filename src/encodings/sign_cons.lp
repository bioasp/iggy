% input facts
% obs_v_label(Experiment, Vertex , Sign)
% edge(X,Y)
% obs_e_label(X,Y,Sign).
% vertex(Name)

sign(1;-1;0).
obs(1;-1;0;notPlus;notMinus).
exp(E) :- obs_v_label(E,V,S).
exp(E) :- input(E,V).
vertex(V) :- obs_v_label(E,V,S).
edge(X,Y) :- obs_e_label(X,Y,S).
vertex(X) :- edge(X,Y).
vertex(Y) :- edge(X,Y).
input(E,V) :- input(V), exp(E).

% for each vertex the measurements are either changing (1,-1) or not (0)
1 {vlabel(E,V,1); vlabel(E,V,-1); vlabel(E,V,0)} :- vertex(V), exp(E).
1 {elabel(U,V,1); elabel(U,V,-1)} 1 :- edge(U,V), not remedge(U,V,1), not remedge(U,V,-1).

% keep observed labeling of the edges
error_edge(U,V) :- obs_e_label(U,V,1), obs_e_label(U,V,-1).
elabel(U,V,S) :- edge(U,V), obs_e_label(U,V,S), not error_edge(U,V), not remedge(U,V,S).

% how to hande error_edges
elabel(U,V,1)  :- edge(U,V), obs_e_label(U,V,1), obs_e_label(U,V,-1), not remedge(U,V,1), remedge(U,V,-1).
elabel(U,V,-1) :- edge(U,V), obs_e_label(U,V,1), obs_e_label(U,V,-1), remedge(U,V,1), not remedge(U,V,-1).

% new edges through repair

elabel(U,V,1) :- addedge(U,V,1).
elabel(U,V,-1) :-addedge(U,V,-1).

% influences
infl(E,V,S*T) :- elabel(U,V,S), vlabel(E,U,T).
% effects of a repair
infl(E,V,S) :- new_influence(E,V,S).

% pure influences
pinfl(E,V, 1) :- elabel(U,V, 1), vlabel(E,U, 1), not vlabel(E,U,0), not vlabel(E,U,-1).
pinfl(E,V,-1) :- elabel(U,V,-1), vlabel(E,U, 1), not vlabel(E,U,0), not vlabel(E,U,-1).
pinfl(E,V,-1) :- elabel(U,V, 1), vlabel(E,U,-1), not vlabel(E,U,0), not vlabel(E,U, 1).
pinfl(E,V, 1) :- elabel(U,V,-1), vlabel(E,U,-1), not vlabel(E,U,0), not vlabel(E,U, 1).
% effects of a repair
pinfl(E,V,S) :- new_influence(E,V,S).

% constraints
:- forbidden(E,V,S), vlabel(E,V,S).

% levelbound constraints
% if the initial level is at the minimum it cannot decrease anymore
forbidden(E,V,-1) :- is_min(E,V).
% if the initial level is at the maximum it cannot increase anymore
forbidden(E,V, 1) :- is_max(E,V).
