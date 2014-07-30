 
%constraint 0
bot :- active(V),      not trivial(V), vlabel(V,0),  infl(V,1),  noneg(U,V) : edge(U,V).
bot :- active(V),      not trivial(V), vlabel(V,0),  infl(V,-1), nopos(U,V) : edge(U,V).