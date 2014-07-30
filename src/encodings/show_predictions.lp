pred(E,X,1) :- vlabel(E,X,1). 
pred(E,X,-1) :- vlabel(E,X,-1). 
pred(E,X,0) :- vlabel(E,X,0). 

% further rules for predicting signs

pred(E,V,notPlus):- vlabel(E,V, 0). 
pred(E,V,notPlus):- vlabel(E,V, -1). 

pred(E,V,notMinus):- vlabel(E,V, 0). 
pred(E,V,notMinus):- vlabel(E,V, 1).
 
pred(E,V,change):- vlabel(E,V, 1).
pred(E,V,change):- vlabel(E,V, -1). 


new_pred(E,V,1) :- pred(E,V,1), not obs_vlabel(E,V,1).
new_pred(E,V,0) :- pred(E,V,0), not obs_vlabel(E,V,0).
new_pred(E,V,-1) :- pred(E,V,-1), not obs_vlabel(E,V,-1).

new_pred(E,V,notPlus) :- pred(E,V,notPlus), not obs_vlabel(E,V,notPlus), not obs_vlabel(E,V,-1), not obs_vlabel(E,V,0).
new_pred(E,V,notMinus) :- pred(E,V,notMinus), not obs_vlabel(E,V,notMinus), not obs_vlabel(E,V,1), not obs_vlabel(E,V,0).
new_pred(E,V,change) :- pred(E,V,change), not obs_vlabel(E,V,1), not obs_vlabel(E,V,-1).





%#show pred/3.
#show new_pred/3.
