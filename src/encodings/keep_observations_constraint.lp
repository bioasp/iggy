 
 % keep observed variations
:- vlabel(E,V,1), vertex(V), obs_vlabel(E,V,-1).
:- vlabel(E,V,0), vertex(V), obs_vlabel(E,V,-1).

:- vlabel(E,V,-1), vertex(V), obs_vlabel(E,V,0).
:- vlabel(E,V,1), vertex(V), obs_vlabel(E,V,0).

:- vlabel(E,V,-1), vertex(V), obs_vlabel(E,V,1).
:- vlabel(E,V,0), vertex(V), obs_vlabel(E,V,1).


% A weak vertex variation has been observed
:- vlabel(E,V,1), vertex(V), obs_vlabel(E,V,notPlus).
:- vlabel(E,V,-1), vertex(V), obs_vlabel(E,V,notMinus).

