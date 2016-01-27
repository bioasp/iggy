
% propagate forward
vlabel(E,V, 0) :- exp(E), vertex(V), not forbidden(E,V, 0).
vlabel(E,V, 1) :- infl(E,V, 1), not forbidden(E,V, 1).
vlabel(E,V,-1) :- infl(E,V,-1), not forbidden(E,V,-1).


% constraint zero for generic nodes
forbidden(E,gen(V),0) :- pinfl(E,gen(V), 1), not infl(E,gen(V),-1), not input(E,gen(V)), not ismax(E,gen(V)).
forbidden(E,gen(V),0) :- pinfl(E,gen(V),-1), not infl(E,gen(V), 1), not input(E,gen(V)), not ismin(E,gen(V)) .

% constraint zero for and nodes
forbidden(E,and(V),0) :- infl(E,and(V), 1), not infl(E,and(V),-1), not infl(E,and(V),0), not input(E,and(V)), not ismax(E,and(V)).
forbidden(E,and(V),0) :- infl(E,and(V),-1), not infl(E,and(V), 1), not infl(E,and(V),0), not input(E,and(V)), not ismin(E,and(V)).


% constraint zero for depmat does not work with and nodes
forbidden(E,gen(V), 0) :- prinfl(E,gen(V),-1), not pos_path(E,gen(V)), not input(E,gen(V)).
forbidden(E,gen(V), 0) :- prinfl(E,gen(V), 1), not neg_path(E,gen(V)), not input(E,gen(V)).
