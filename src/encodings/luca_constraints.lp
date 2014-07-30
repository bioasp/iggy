
% When a non input vertex receives only +1 influences refute -1 and 0 labels.
:- vlabel(E,Y,-1), infl(E,Y,1), not infl(E,Y,0), not infl(E,Y,-1), not input(E,Y).
:- vlabel(E,Y,0),  infl(E,Y,1), not infl(E,Y,0), not infl(E,Y,-1), not input(E,Y).

% When a non input vertex receives only -1 influences refute 0 and 1 labels.
:- vlabel(E,Y,0), infl(E,Y,-1), not infl(E,Y,0), not infl(E,Y,1), not input(E,Y).
:- vlabel(E,Y,1), infl(E,Y,-1), not infl(E,Y,0), not infl(E,Y,1), not input(E,Y).


