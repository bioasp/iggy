 
%luca constraint 
bot :- active(V),      not trivial(V), vlabel(V,1),  neg(U,V) : edge(U,V).
bot :- active(V),      not trivial(V), vlabel(V,0),  neg(U,V) : edge(U,V).
bot :- active(V),      not trivial(V), vlabel(V,-1), pos(U,V) : edge(U,V).
bot :- active(V),      not trivial(V), vlabel(V,0),  infl(V,0),  pos(U,V) : edge(U,V).