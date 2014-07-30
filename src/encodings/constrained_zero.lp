 
%constraints for zero
 :- vlabel(E,V,0), infl(E,V,1), not infl(E,V,-1), not input(E,V).
 :- vlabel(E,V,0), infl(E,V,-1), not infl(E,V,1), not input(E,V).