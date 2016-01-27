
% guess one edgend to add
0{rep(addeddy(gen(V)))}  :-     vertex(gen(V)).
%0{rep(addeddy(V))}  :-     vertex(V).

% new inputs through repair
input(E,"unknown")      :- exp(E).
vertex("unknown").
elabel("unknown", V,1)  :- rep(addeddy(V)).
:-rep(addeddy(U)),rep(addeddy(V)),U!=V.
