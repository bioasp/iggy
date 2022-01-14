% new inputs through repair
input(E,or(\"unknownup\"))    :- new_influence(E,X,S).
vlabel(E,or(\"unknownup\"),1) :- new_influence(E,X,S).
elabel(or(\"unknownup\"), X, 1)   :- new_influence(E,X,1).
elabel(or(\"unknownup\"), X,-1)   :- new_influence(E,X,-1).

% in a network exists under Condition E a positive path to X

pos_path(E,X,X) :- input(E,X), vlabel(E,X, 1), not is_max(E,X).

neg_path(E,X,X) :- input(E,X), vlabel(E,X,-1), not is_min(E,X).

pos_path(E,X,conc(P,X)) :- pos_path(E,Y,P), not is_max(E,X),
                               elabel(Y,X, 1), not input(E,X), X!=Y,
	                       false==@member(X,P).


neg_path(E,X,conc(P,X)) :- pos_path(E,Y,P), not is_min(E,X),
                               elabel(Y,X,-1), not input(E,X), X!=Y,
	                       false==@member(X,P).

pos_path(E,X,conc(P,X)) :- neg_path(E,Y,P), not is_max(E,X),
                               elabel(Y,X,-1), not input(E,X), X!=Y,
	                       false==@member(X,P).

neg_path(E,X,conc(P,X)) :- neg_path(E,Y,P), not is_min(E,X),
                               elabel(Y,X, 1), not input(E,X), X!=Y,
	                       false==@member(X,P).

pos_path(E,V) :- pos_path(E,V,P).
neg_path(E,V) :- neg_path(E,V,P).


% pure influences
prinfl(E,V, 1) :- elabel(U,V, 1),
                  pos_path(E,V,P),true==@member(U,P),
                  vlabel(E,U, 1), not vlabel(E,U,0), not vlabel(E,U,-1).
prinfl(E,V,-1) :- elabel(U,V,-1),
                  neg_path(E,V,P),true==@member(U,P),
                  vlabel(E,U, 1), not vlabel(E,U,0), not vlabel(E,U,-1).
prinfl(E,V,-1) :- elabel(U,V, 1),
                  neg_path(E,V,P),true==@member(U,P),
                  vlabel(E,U,-1), not vlabel(E,U,0), not vlabel(E,U, 1).
prinfl(E,V, 1) :- elabel(U,V,-1),
                  pos_path(E,V,P),true==@member(U,P),
                  vlabel(E,U,-1), not vlabel(E,U,0), not vlabel(E,U, 1).

forbidden(E,V, 1) :- exp(E), vertex(V), not pos_path(E,V), not input(E,V).
forbidden(E,V,-1) :- exp(E), vertex(V), not neg_path(E,V), not input(E,V).
