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



% influences
infl(E,V,S*T) :- elabel(U,V,S), vlabel(E,U,T).
% effects of a repair
infl(E,V,S) :- rep(new_influence(E,V,S)).


% pure influences
pinfl(E,V, 1) :- elabel(U,V, 1), vlabel(E,U, 1), not vlabel(E,U,0), not vlabel(E,U,-1).
pinfl(E,V,-1) :- elabel(U,V,-1), vlabel(E,U, 1), not vlabel(E,U,0), not vlabel(E,U,-1).
pinfl(E,V,-1) :- elabel(U,V, 1), vlabel(E,U,-1), not vlabel(E,U,0), not vlabel(E,U, 1).
pinfl(E,V, 1) :- elabel(U,V,-1), vlabel(E,U,-1), not vlabel(E,U,0), not vlabel(E,U, 1).




% if a node has been observed only one sign
forbidden(E,V,T) :- vlabel(E,V,S1), obs_vlabel(E,V,S), sign(S), sign(T), T!=S1.
forbidden(E,V,T) :- vlabel(E,V,S1), err(flip(E,V,N)), obs_vlabel(E,V,S), sign(T), T!=S1.

% constraints
:- forbidden(E,V,S), vlabel(E,V,S).

% levelbound constraints
% if the initial level is at the minimum it cannot decrease anymore
forbidden(E,V,-1) :- ismin(E,V).

% if the initial level is at the maximum it cannot increase anymore
forbidden(E,V, 1) :- ismax(E,V).


#hide.
% #show vlabel/3.
%#show err/1.
%#show pos_path/3.
%#show neg_path/3.
% #show strange/3.
