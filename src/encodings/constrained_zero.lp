 
%constraints for zero for generic nodes
 :- vlabel(E,gen(V),0), infl(E,gen(V),1), not infl(E,gen(V),-1), not input(E,gen(V)).
 :- vlabel(E,gen(V),0), infl(E,gen(V),-1), not infl(E,gen(V),1), not input(E,gen(V)).

%constraints for zero for and node
 :- vlabel(E,and(V),0), infl(E,and(V),1), not infl(E,and(V),-1), not infl(E,and(V),0), not input(E,and(V)).
 :- vlabel(E,and(V),0), infl(E,and(V),-1), not infl(E,and(V),1), not infl(E,and(V),0), not input(E,and(V)).
 
 
% constraintes for pure depmat(founded through shortest elementary path) for generic nodes
:- vlabel(E,gen(V),0), getspos(E,gen(V)), not getsneg(E,gen(V)), not input(E,gen(V)).
:- vlabel(E,gen(V),0), getsneg(E,gen(V)), not getspos(E,gen(V)), not input(E,gen(V)).

% constraintes for pure depmat(founded through shortest elementary path) for and nodes
:- vlabel(E,and(V),0), getspos(E,and(V)), not mis_pos(E,and(V)), not getsneg(E,and(V)), not input(E,and(V)).
:- vlabel(E,and(V),0), getsneg(E,and(V)), not mis_neg(E,and(V)), not getspos(E,and(V)), not input(E,and(V)).