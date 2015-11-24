 
%constraints for zero for generic nodes
 :- vlabel(E,gen(V),0), infl(E,gen(V),1), not infl(E,gen(V),-1), not input(E,gen(V)).
 :- vlabel(E,gen(V),0), infl(E,gen(V),-1), not infl(E,gen(V),1), not input(E,gen(V)).

%constraints for zero for and node
 :- vlabel(E,and(V),0), infl(E,and(V),1), not infl(E,and(V),-1), not infl(E,and(V),0), not input(E,and(V)).
 :- vlabel(E,and(V),0), infl(E,and(V),-1), not infl(E,and(V),1), not infl(E,and(V),0), not input(E,and(V)).
 
 
% constraintes for pure depmat(founded through shortest elementary path) for generic nodes
:- vlabel(E,V,0), depmat, not cannull(E,V).