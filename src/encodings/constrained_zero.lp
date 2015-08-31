 
%constraints for zero
 :- vlabel(E,V,0), infl(E,V,1), not infl(E,V,-1), not input(E,V).
 :- vlabel(E,V,0), infl(E,V,-1), not infl(E,V,1), not input(E,V).

% constraintes for pure depmat(founded through shortest elementary path)
:- vlabel(E,V,0), getspos(E,V), not getsneg(E,V), not input(E,V).
:- vlabel(E,V,0), getsneg(E,V), not getspos(E,V), not input(E,V).
