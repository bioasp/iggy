% repair model
% define possible repair operations
0 { rep(new_influence(E,gen(X),-1)), rep(new_influence(E,gen(X),1)) } 1 :- vertex(gen(X)), exp(E).

