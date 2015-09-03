
% compute getspos getsneg

% in network n exist a pos_path from X to Y
pos_path(E,X,Y) :-
       	input(E,X), elabel(X,Y, 1), not input(E,Y),
       	vertex(Y).
pos_path(E,X,Y)    :-
       	pos_path(E,X,Z), elabel(Z,Y, 1), not input(E,Y),
       	vertex(Y).                               
pos_path(E,X,Y)    :-
       	neg_path(E,X,Z), elabel(Z,Y,-1), not input(E,Y),
       	vertex(Y).
                       
% in network n exist a neg_path from X to Y
neg_path(E,X,Y) :-
       	input(E,X), elabel(X,Y,-1), not input(E,Y),
       	vertex(Y).
neg_path(E,X,Y)    :-
       	neg_path(E,X,Z), elabel(Z,Y, 1), not input(E,Y),
       	vertex(Y).
                          
neg_path(E,X,Y)    :-
       	pos_path(E,X,Z), elabel(Z,Y,-1), not input(E,Y),
       	vertex(Y).

% new inputs through repair
input(E,gen("unknownup")) :- rep(new_influence(E,X,S)).
obs_vlabel(E,gen("unknownup"),1):- rep(new_influence(E,X,S)).  
elabel(gen("unknownup"), X,1)  :- rep(new_influence(E,X,1)).     
elabel(gen("unknownup"), X,-1) :- rep(new_influence(E,X,-1)).     

% Y gets a positive influence
getspos(E,Y) :- input(E,X), vlabel(E,X, 1), vertex(Y), pos_path(E,X,Y).
getspos(E,Y) :- input(E,X), vlabel(E,X,-1), vertex(Y), neg_path(E,X,Y).

% Y gets a negative influence
getsneg(E,Y) :- input(E,X), vlabel(E,X, 1), vertex(Y), neg_path(E,X,Y).
getsneg(E,Y) :- input(E,X), vlabel(E,X,-1), vertex(Y), pos_path(E,X,Y).


% labels must be consistent with depmat

 :- vlabel(E,V, 1), not getspos(E,V), not input(E,V).
 :- vlabel(E,V,-1), not getsneg(E,V), not input(E,V).

 :- vlabel(E,V,0), getspos(E,V), not getsneg(E,V).
 :- vlabel(E,V,0), getsneg(E,V), not getspos(E,V).

#hide.
