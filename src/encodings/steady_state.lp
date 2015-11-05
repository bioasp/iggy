
% measured variations must be explained by  predecessor

infl(E,V,S*T) :- elabel(U,V,S), vlabel(E,U,T).

 :- vlabel(E,gen(V), 1), not infl(E,gen(V),1),  not input(E,gen(V)).
 :- vlabel(E,gen(V),-1), not infl(E,gen(V),-1), not input(E,gen(V)).
 
 

% effects of a repair
infl(E,V,S) :- rep(new_influence(E,V,S)).

#hide.
