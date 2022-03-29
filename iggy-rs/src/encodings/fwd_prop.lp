% propagate forward
vlabel(E,V, 0) :- exp(E), vertex(V), not forbidden(E,V, 0).
vlabel(E,V, 1) :- infl(E,V, 1), not forbidden(E,V, 1).
vlabel(E,V,-1) :- infl(E,V,-1), not forbidden(E,V,-1).

% constraint zero for or nodes
forbidden(E,or(V),0) :- pinfl(E,or(V), 1), not infl(E,or(V),-1), not input(E,or(V)), not is_max(E,or(V)).
forbidden(E,or(V),0) :- pinfl(E,or(V),-1), not infl(E,or(V), 1), not input(E,or(V)), not is_min(E,or(V)) .

% constraint zero for and nodes
forbidden(E,and(V),0) :- infl(E,and(V), 1), not infl(E,and(V),-1), not infl(E,and(V),0), not input(E,and(V)), not is_max(E,and(V)).
forbidden(E,and(V),0) :- infl(E,and(V),-1), not infl(E,and(V), 1), not infl(E,and(V),0), not input(E,and(V)), not is_min(E,and(V)).

% constraint zero for depmat does not work with and nodes
forbidden(E,or(V), 0) :- prinfl(E,or(V), 1), not neg_path(E,or(V)), not input(E,or(V)), not is_max(E,or(V)).
forbidden(E,or(V), 0) :- prinfl(E,or(V),-1), not pos_path(E,or(V)), not input(E,or(V)), not is_min(E,or(V)).
