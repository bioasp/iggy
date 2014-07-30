% obs(1;-1;0;notPlus;notMinus).
% vertex(V) :- obs_vlabel(E,V,S).
 edge(X,Y) :- obs_elabel(X,Y,S).
 vertex(X) :- edge(X,Y).
 vertex(Y) :- edge(X,Y).


% obss_elabel fixes problems with contradictory influences
obss_elabel(U,V, 1)  :- obs_elabel(U,V, 1),  not obs_elabel(U,V, -1).
obss_elabel(U,V, -1) :- obs_elabel(U,V, -1), not obs_elabel(U,V, 1).
obs_vlabel(U,S) :- obs_vlabel(E,U,S).
input(V) :- input(E,V).

zign(1;-1).
sign(1;-1;0).


diff(V,V)  :- obs_vlabel(V,1),  edge(V,V), obss_elabel(V,V,-1), not obss_elabel(V,V, 1).
diff(V,V)  :- obs_vlabel(V,-1), edge(V,V), obss_elabel(V,V,-1), not obss_elabel(V,V, 1).

diff(U,V)  :- edge(U,V), obss_elabel(U,V,-1), not obss_elabel(U,V, 1), obs_vlabel(U,1),  obs_vlabel(V,1).
diff(U,V)  :- edge(U,V), obss_elabel(U,V,-1), not obss_elabel(U,V, 1), obs_vlabel(U,-1), obs_vlabel(V,-1).

diff(U,V)  :- edge(U,V), obss_elabel(U,V, 1), not obss_elabel(U,V,-1), obs_vlabel(U,1),  obs_vlabel(V,-1).
diff(U,V)  :- edge(U,V), obss_elabel(U,V, 1), not obss_elabel(U,V,-1), obs_vlabel(U,-1), obs_vlabel(V,1).

trivial(V) :- vertex(V), not input(V), diff(U,V) : edge(U,V).

reach(U,V) :- edge(U,V), not trivial(V).
reach(V,U) :- edge(U,V), not trivial(V),                        not obs_vlabel(U,S) : sign(S).
reach(U,W) :- edge(U,V), not trivial(V), reach(V,W), vertex(W).
reach(V,W) :- edge(U,V), not trivial(V), reach(U,W), vertex(W), not obs_vlabel(U,S) : sign(S).


%%%%%%%%%%%%%
% Generator %
%%%%%%%%%%%%%

singleton | nonsingleton.

active(V) : trivial(V)  :-    singleton.
active(V) | inactive(V) :- nonsingleton, vertex(V), not trivial(V), not input(V).

:- active(V;W), not trivial(V;W), not reach(V,W).
:- active(V),   not trivial(V),   not obs_vlabel(V,S) : sign(S), not active(W) : edge(V,W).


%%%%%%%%%%%%%%%%%%%%%%
% Inconsistency Test %
%%%%%%%%%%%%%%%%%%%%%%

% for each vertex the measurements are either changing (1,-1) or not (0)
vlabel(V,1) | vlabel(V,-1) | vlabel(V,0) :- active(V), not trivial(V), vertex(V), not obs_vlabel(V,T) : sign(T).
vlabel(U,1) | vlabel(U,-1) | vlabel(U,0) :- active(V), not trivial(V), edge(U,V), not obs_vlabel(U,T) : sign(T).

vlabel(V,1)  | vlabel(V,0) :- active(V), vertex(V), obs_vlabel(V,notMinus),   not trivial(V), not input(V).
vlabel(U,1)  | vlabel(U,0) :- active(V), edge(U,V), obs_vlabel(U,notMinus),   not trivial(V), not input(V).

vlabel(V,-1) | vlabel(V,0) :- active(V), vertex(V), obs_vlabel(V,notPlus),   not trivial(V), not input(V).
vlabel(U,-1) | vlabel(U,0) :- active(V), edge(U,V), obs_vlabel(U,notPlus),   not trivial(V), not input(V).
vlabel(V,1)                :- active(V), vertex(V), obs_vlabel(V,1),   not trivial(V), not input(V).
vlabel(U,1)                :- active(V), edge(U,V), obs_vlabel(U,1),   not trivial(V), not input(V).

vlabel(V,-1)               :- active(V), vertex(V), obs_vlabel(V,-1),   not trivial(V), not input(V).
vlabel(U,-1)               :- active(V), edge(U,V), obs_vlabel(U,-1),   not trivial(V), not input(V).

elabel(U,V,1) | elabel(U,V,-1) :- active(V),   not trivial(V), edge(U,V),    not obss_elabel(U,V,T) : zign(T).
elabel(U,V,S)                  :- active(V), edge(U,V), obss_elabel(U,V,S), not trivial(V), not input(V).


nopos(U,V) :- elabel(U,V,1),  not obss_elabel(U,V, -1), vlabel(U,-1), active(V),  not trivial(V), not input(V).
nopos(U,V) :- elabel(U,V,-1), not obss_elabel(U,V, 1),  vlabel(U,1),  active(V),  not trivial(V), not input(V).
nopos(U,V) :- edge(U,V),   vlabel(U,0),  active(V),  not trivial(V), not input(V).
 
noneg(U,V) :- elabel(U,V,1),  not obss_elabel(U,V, -1), vlabel(U,1),  active(V),  not trivial(V), not input(V).
noneg(U,V) :- elabel(U,V,-1), not obss_elabel(U,V, 1), vlabel(U,-1),  active(V),  not trivial(V), not input(V).
noneg(U,V) :- edge(U,V),   vlabel(U,0),  active(V),  not trivial(V), not input(V).

pos(U,V) :- elabel(U,V,1),  not obss_elabel(U,V, -1), vlabel(U,1),  active(V),  not trivial(V), not input(V).
pos(U,V) :- elabel(U,V,-1), not obss_elabel(U,V, 1), vlabel(U,-1),  active(V),  not trivial(V), not input(V).

neg(U,V) :- elabel(U,V,1),  not obss_elabel(U,V, -1), vlabel(U,-1),  active(V),  not trivial(V), not input(V).
neg(U,V) :- elabel(U,V,-1), not obss_elabel(U,V, 1), vlabel(U,1),  active(V),  not trivial(V), not input(V).


infl(V,S*T) :- elabel(U,V,S), vlabel(U,T), active(V).

bot :- singleton.

bot :- active(V),      not trivial(V), vlabel(V, 1),  nopos(U,V) : edge(U,V).
bot :- active(V),      not trivial(V), vlabel(V,-1),  noneg(U,V) : edge(U,V).

vlabel(V,S)    :- bot, vertex(V), sign(S),      not trivial(V), not input(V), not obs_vlabel(V,T)    : sign(T).
vlabel(U,S)    :- bot, edge(U,V), sign(S),      not trivial(V), not input(V), not obs_vlabel(U,T)    : sign(T).
elabel(U,V,1)  :- bot, edge(U,V),               not trivial(V), not input(V), not obss_elabel(U,V,T) : zign(T).
elabel(U,V,-1) :- bot, edge(U,V),               not trivial(V), not input(V), not obss_elabel(U,V,T) : zign(T).
:- not bot.



#hide.
#show active/1.