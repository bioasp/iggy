
% measured variations must be explained by  predecessor

forbidden(E,V, 1) :- exp(E), vertex(V), not infl(E,V, 1), not input(E,V).
forbidden(E,V,-1) :- exp(E), vertex(V), not infl(E,V,-1), not input(E,V).

 


