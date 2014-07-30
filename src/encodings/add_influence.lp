% repair model
% define possible repair operations
0 { rep(new_influence(E,X,-1)), rep(new_influence(E,X,1)) } 1 :- vertex(X), exp(E).

