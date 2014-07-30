edge(X,Y) :- obs_elabel(X,Y,S).
vertex(X) :- edge(X,Y).
vertex(Y) :- edge(X,Y).

% definition of input
input(V) :- vertex(V), not edge(U,V) : edge(U,V) : U != V.

#hide.
#show input/1.