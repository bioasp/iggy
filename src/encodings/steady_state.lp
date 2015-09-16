
% measured variations must be explained by  predecessor

infl(E,V,S*T) :- elabel(U,V,S), vlabel(E,U,T).

 :- vlabel(E,V, 1), not infl(E,V,1),  not input(E,V).
 :- vlabel(E,V,-1), not infl(E,V,-1), not input(E,V).
 

% effects of a repair
infl(E,V,S) :- rep(new_influence(E,V,S)).

#hide.
