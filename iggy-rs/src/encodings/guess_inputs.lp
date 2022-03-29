edge(X,Y) :- obs_e_label(X,Y,S).
vertex(X) :- edge(X,Y).
vertex(Y) :- edge(X,Y).
% declare inputs
input(V) :- vertex(V), not edge(U,V) : edge(U,V), U != V.
#show input/1.
