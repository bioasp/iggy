% repair model
% define possible repair operations
new_influence(E,or(X),1) :- not not new_influence(E,or(X),1), not new_influence(E,or(X),-1), vertex(or(X)), exp(E), not input(E,or(X)).
new_influence(E,or(X),-1) :- not not new_influence(E,or(X),-1), not new_influence(E,or(X),1), vertex(or(X)), exp(E), not input(E,or(X)).
