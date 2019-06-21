pub const PRG_CONTRADICTORY_OBS: &'static str = "
% two contradictory observations
contradiction1(E,X) :- obs_v_label(E,X,1), obs_v_label(E,X,0).
contradiction2(E,X) :- obs_v_label(E,X,-1), obs_v_label(E,X,0).
contradiction3(E,X) :- obs_v_label(E,X,-1), obs_v_label(E,X,1).
contradiction4(E,X) :- obs_v_label(E,X,notMinus), obs_v_label(E,X,-1).
contradiction5(E,X) :- obs_v_label(E,X,notPlus), obs_v_label(E,X,1).

% contradictions of observed behavior and initial level
contradiction6(E,X) :- obs_v_label(E,X,-1), is_min(E,X).
contradiction7(E,X) :- obs_v_label(E,X,1), is_max(E,X).

#show contradiction1/2.
#show contradiction2/2.
#show contradiction3/2.
#show contradiction4/2.
#show contradiction5/2.
#show contradiction6/2.
#show contradiction7/2.
";

pub const PRG_GUESS_INPUTS: &'static str = "
edge(X,Y) :- obs_e_label(X,Y,S).
vertex(X) :- edge(X,Y).
vertex(Y) :- edge(X,Y).
% declare inputs
input(V) :- vertex(V), not edge(U,V) : edge(U,V), U != V.
#show input/1.
";

pub const PRG_SIGN_CONS: &'static str = "
% input facts
% obs_v_label(Experiment, Vertex , Sign)
% edge(X,Y)
% obs_e_label(X,Y,Sign).
% vertex(Name)

sign(1;-1;0).
obs(1;-1;0;notPlus;notMinus).
exp(E) :- obs_v_label(E,V,S).
exp(E) :- input(E,V).
vertex(V) :- obs_v_label(E,V,S).
edge(X,Y) :- obs_e_label(X,Y,S).
vertex(X) :- edge(X,Y).
vertex(Y) :- edge(X,Y).

% for each vertex the measurements are either changing (1,-1) or not (0)
1 {vlabel(E,V,1); vlabel(E,V,-1); vlabel(E,V,0)} :- vertex(V), exp(E).
1 {elabel(U,V,1); elabel(U,V,-1)} 1 :- edge(U,V), not remedge(U,V,1), not remedge(U,V,-1).

% keep observed labeling of the edges
error_edge(U,V) :- obs_e_label(U,V,1), obs_e_label(U,V,-1).
elabel(U,V,S) :- edge(U,V), obs_e_label(U,V,S), not error_edge(U,V), not remedge(U,V,S).

% how to hande error_edges
elabel(U,V,1)  :- edge(U,V), obs_e_label(U,V,1), obs_e_label(U,V,-1), not remedge(U,V,1), remedge(U,V,-1).
elabel(U,V,-1) :- edge(U,V), obs_e_label(U,V,1), obs_e_label(U,V,-1), remedge(U,V,1), not remedge(U,V,-1).

% influences
infl(E,V,S*T) :- elabel(U,V,S), vlabel(E,U,T).
% effects of a repair
infl(E,V,S) :- new_influence(E,V,S).

% pure influences
pinfl(E,V, 1) :- elabel(U,V, 1), vlabel(E,U, 1), not vlabel(E,U,0), not vlabel(E,U,-1).
pinfl(E,V,-1) :- elabel(U,V,-1), vlabel(E,U, 1), not vlabel(E,U,0), not vlabel(E,U,-1).
pinfl(E,V,-1) :- elabel(U,V, 1), vlabel(E,U,-1), not vlabel(E,U,0), not vlabel(E,U, 1).
pinfl(E,V, 1) :- elabel(U,V,-1), vlabel(E,U,-1), not vlabel(E,U,0), not vlabel(E,U, 1).
% effects of a repair
pinfl(E,V,S) :- new_influence(E,V,S).

% if a node has been observed only one sign
% forbidden(E,V,T) :- vlabel(E,V,S1), obs_v_label(E,V,S), sign(S), sign(T), T!=S1.
% forbidden(E,V,T) :- vlabel(E,V,S1), err(flip(E,V,N)), obs_v_label(E,V,S), sign(T), T!=S1.

% constraints
:- forbidden(E,V,S), vlabel(E,V,S).

% levelbound constraints
% if the initial level is at the minimum it cannot decrease anymore
forbidden(E,V,-1) :- is_min(E,V).
% if the initial level is at the maximum it cannot increase anymore
forbidden(E,V, 1) :- is_max(E,V).
";

pub const PRG_BWD_PROP: &'static str = "
% measured variations must be explained by  predecessor
forbidden(E,V, 1) :- exp(E), vertex(V), not infl(E,V, 1), not input(E,V).
forbidden(E,V,-1) :- exp(E), vertex(V), not infl(E,V,-1), not input(E,V).
";

pub const PRG_ONE_STATE: &'static str = "forbidden(E,V,S) :- vlabel(E,V,T), sign(S), S!=T.";

pub const PRG_FWD_PROP: &'static str = "
% propagate forward
vlabel(E,V, 0) :- exp(E), vertex(V), not forbidden(E,V, 0).
vlabel(E,V, 1) :- infl(E,V, 1), not forbidden(E,V, 1).
vlabel(E,V,-1) :- infl(E,V,-1), not forbidden(E,V,-1).

% constraint zero for or nodes
forbidden(E,or(V),0) :- pinfl(E,or(V), 1), not infl(E,or(V),-1), not input(E,or(V)), not is_max(E,or(V)).
forbidden(E,or(V),0) :- pinfl(E,or(V),-1), not infl(E,or(V), 1), not input(E,or(V)), not is_min(E,or(V)) .

% constraint zero for and nodes
forbidden(E,and(V),0) :- infl(E,and(V), 1), not infl(E,and(V),-1), not infl(E,and(V),0), not input(E,and(V)), not is_max(E,and(V)).
forbidden(E,and(V),0) :- infl(E,and(V),-1), not infl(E,and(V), 1), not infl(E,and(V),0), not input(E,and(V)), not is_min(E,and(V)).

% constraint zero for depmat does not work with and nodes
forbidden(E,or(V), 0) :- prinfl(E,or(V), 1), not neg_path(E,or(V)), not input(E,or(V)), not is_max(E,or(V)).
forbidden(E,or(V), 0) :- prinfl(E,or(V),-1), not pos_path(E,or(V)), not input(E,or(V)), not is_min(E,or(V)).
";

pub const PRG_FOUNDEDNESS: &'static str = "
% founded rules
founded(E,X,-1) :- input(E,X).
founded(E,X, 1) :- input(E,X).
founded(E,X,-1) :- founded(E,Y,-1), elabel(Y,X, 1).
founded(E,X,-1) :- founded(E,Y, 1), elabel(Y,X,-1).
founded(E,X, 1) :- founded(E,Y,-1), elabel(Y,X,-1).
founded(E,X, 1) :- founded(E,Y, 1), elabel(Y,X, 1).

founded(E,X,S) :- vlabel(E,X,S), new_influence(E,X,S).

forbidden(E,V, 1):- exp(E), vertex(V), not founded(E,V, 1).
forbidden(E,V,-1):- exp(E), vertex(V), not founded(E,V,-1).
";

pub const PRG_ELEM_PATH: &'static str = "
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
";

pub const PRG_ERROR_MEASURE: &'static str = "
err(flip(E,X,1)) :- obs_v_label(E,X, 1), not vlabel(E,X, 1),     vlabel(E,X,0).
err(flip(E,X,2)) :- obs_v_label(E,X, 1), not vlabel(E,X, 1), not vlabel(E,X,0), vlabel(E,X,-1).

err(flip(E,X,1)) :- obs_v_label(E,X,-1), not vlabel(E,X,-1),     vlabel(E,X,0).
err(flip(E,X,2)) :- obs_v_label(E,X,-1), not vlabel(E,X,-1), not vlabel(E,X,0), vlabel(E,X, 1).

err(flip(E,X,1)) :- obs_v_label(E,X, 0), not vlabel(E,X, 0).

err(flip(E,X,2)) :- obs_v_label(E,X, notMinus), not vlabel(E,X, 0), not vlabel(E,X, 1).
err(flip(E,X,2)) :- obs_v_label(E,X, notPlus), not vlabel(E,X, 0), not vlabel(E,X,-1).";

pub const PRG_MIN_WEIGHTED_ERROR: &'static str = "#minimize{V@2,(E,X,V) : err(flip(E,X,V)) }.";

pub const PRG_KEEP_INPUTS: &'static str = "
% keep observed input variation
forbidden(E,V,T) :- input(E,V), obs_v_label(E,V,S), sign(S), sign(T), S!=T.

% A weak input
forbidden(E,V, 1) :- input(E,V), obs_v_label(E,V,notPlus).
forbidden(E,V,-1) :- input(E,V), obs_v_label(E,V,notMinus).";

pub const PRG_SHOW_ERRORS: &'static str = "#show err/1.";
pub const PRG_SHOW_LABELS: &'static str = "#show vlabel(X,or(V),S) : vlabel(X,or(V),S).";
pub const PRG_SHOW_REPAIRS: &'static str = "#show remedge/3.
#show addedge/3.
#show new_influence/1.";
pub const PRG_SHOW_ADD_EDGE_END: &'static str = "
#show addeddy/1.
";
pub const PRG_ADD_INFLUENCES: &'static str = "
% repair model
% define possible repair operations
new_influence(E,or(X),1) :- not not new_influence(E,or(X),1), not new_influence(E,or(X),-1), vertex(or(X)), exp(E), not input(E,or(X)).
new_influence(E,or(X),-1) :- not not new_influence(E,or(X),-1), not new_influence(E,or(X),1), vertex(or(X)), exp(E), not input(E,or(X)).
";
pub const PRG_MIN_ADDED_INFLUENCES: &'static str =
    "#minimize{ 1,(E,X,S):new_influence(E,or(X),S)}.";
pub const PRG_KEEP_OBSERVATIONS: &'static str = "% keep observed variations
forbidden(E,V,T) :- obs_v_label(E,V,S), sign(S), sign(T), S!=T.

% A weak vertex variation has been observed
forbidden(E,V, 1) :- vertex(V), obs_v_label(E,V,notPlus).
forbidden(E,V,-1) :- vertex(V), obs_v_label(E,V,notMinus).";

pub const PRG_SHOW_PREDICTIONS: &'static str = "
pred(E,X, 1) :- vlabel(E,X, 1).
pred(E,X,-1) :- vlabel(E,X,-1).
pred(E,X, 0) :- vlabel(E,X, 0).

% further rules for predicting signs

pred(E,V,notPlus) :- vlabel(E,V, 0).
pred(E,V,notPlus) :- vlabel(E,V,-1).

pred(E,V,notMinus):- vlabel(E,V, 0).
pred(E,V,notMinus):- vlabel(E,V, 1).

pred(E,V,change) :- vlabel(E,V, 1).
pred(E,V,change) :- vlabel(E,V,-1).


#show pred/3.

%new_pred(E,V,1) :- pred(E,V,1), not obs_v_label(E,V,1).
%new_pred(E,V,0) :- pred(E,V,0), not obs_v_label(E,V,0).
%new_pred(E,V,-1) :- pred(E,V,-1), not obs_v_label(E,V,-1).

%new_pred(E,V,notPlus) :- pred(E,V,notPlus), not obs_v_label(E,V,notPlus), not obs_v_label(E,V,-1), not obs_v_label(E,V,0).
%new_pred(E,V,notMinus) :- pred(E,V,notMinus), not obs_v_label(E,V,notMinus), not obs_v_label(E,V,1), not obs_v_label(E,V,0).
%new_pred(E,V,change) :- pred(E,V,change), not obs_v_label(E,V,1), not obs_v_label(E,V,-1).


%#show new_pred/3.";

pub const PRG_SHOW_PREDICTIONS_DM: &'static str = "
pred(E,X, 1) :- vlabel(E,X, 1), not vlabel(E,X,0), not vlabel(E,X,-1).
pred(E,X,-1) :- vlabel(E,X,-1), not vlabel(E,X,0), not vlabel(E,X, 1).
pred(E,X, 0) :- vlabel(E,X, 0), not vlabel(E,X,1), not vlabel(E,X,-1).

pred(E,X,notPlus)  :- vlabel(E,X, 0), vlabel(E,X,-1), not vlabel(E,X, 1).
pred(E,X,notMinus) :- vlabel(E,X, 0), vlabel(E,X, 1), not vlabel(E,X,-1).
pred(E,X,change)   :- vlabel(E,X,-1), vlabel(E,X, 1), not vlabel(E,X, 0).

#show pred/3.";

pub const PRG_MICS: &'static str = "
edge(X,Y) :- obs_e_label(X,Y,S).
vertex(X) :- edge(X,Y).
vertex(Y) :- edge(X,Y).

% obss_elabel fixes problems with contradictory influences
obss_elabel(U,V, 1)  :- obs_e_label(U,V, 1),  not obs_e_label(U,V, -1).
obss_elabel(U,V, -1) :- obs_e_label(U,V, -1), not obs_e_label(U,V, 1).
obs_v_label(U,S) :- obs_v_label(E,U,S).
input(V) :- input(E,V).

zign(1;-1).
sign(1;-1;0).


diff(V,V)  :- obs_v_label(V,1),  edge(V,V), obss_elabel(V,V,-1), not obss_elabel(V,V, 1).
diff(V,V)  :- obs_v_label(V,-1), edge(V,V), obss_elabel(V,V,-1), not obss_elabel(V,V, 1).

diff(U,V)  :- edge(U,V), obss_elabel(U,V,-1), not obss_elabel(U,V, 1), obs_v_label(U,1),  obs_v_label(V,1).
diff(U,V)  :- edge(U,V), obss_elabel(U,V,-1), not obss_elabel(U,V, 1), obs_v_label(U,-1), obs_v_label(V,-1).

diff(U,V)  :- edge(U,V), obss_elabel(U,V, 1), not obss_elabel(U,V,-1), obs_v_label(U,1),  obs_v_label(V,-1).
diff(U,V)  :- edge(U,V), obss_elabel(U,V, 1), not obss_elabel(U,V,-1), obs_v_label(U,-1), obs_v_label(V,1).

trivial(V) :- vertex(V), not input(V), diff(U,V) : edge(U,V).

reach(U,V) :- edge(U,V), not trivial(V).
reach(V,U) :- edge(U,V), not trivial(V),                        not obs_v_label(U,S) : sign(S).
reach(U,W) :- edge(U,V), not trivial(V), reach(V,W), vertex(W).
reach(V,W) :- edge(U,V), not trivial(V), reach(U,W), vertex(W), not obs_v_label(U,S) : sign(S).


%%%%%%%%%%%%%
% Generator %
%%%%%%%%%%%%%

singleton | nonsingleton.

active(V) :-    singleton, trivial(V).
active(V) | inactive(V) :- nonsingleton, vertex(V), not trivial(V), not input(V).

:- active(V), active(W), not trivial(V;W), not reach(V,W).
:- active(V),   not trivial(V),   not obs_v_label(V,S); sign(S), not active(W); edge(V,W).


%%%%%%%%%%%%%%%%%%%%%%
% Inconsistency Test %
%%%%%%%%%%%%%%%%%%%%%%

% for each vertex the measurements are either changing (1,-1) or not (0)
vlabel(V,1) | vlabel(V,-1) | vlabel(V,0) :- active(V), not trivial(V), vertex(V), not obs_v_label(V,T) : sign(T).
vlabel(U,1) | vlabel(U,-1) | vlabel(U,0) :- active(V), not trivial(V), edge(U,V), not obs_v_label(U,T) : sign(T).

vlabel(V,1)  | vlabel(V,0) :- active(V), vertex(V), obs_v_label(V,notMinus),   not trivial(V), not input(V).
vlabel(U,1)  | vlabel(U,0) :- active(V), edge(U,V), obs_v_label(U,notMinus),   not trivial(V), not input(V).

vlabel(V,-1) | vlabel(V,0) :- active(V), vertex(V), obs_v_label(V,notPlus),   not trivial(V), not input(V).
vlabel(U,-1) | vlabel(U,0) :- active(V), edge(U,V), obs_v_label(U,notPlus),   not trivial(V), not input(V).
vlabel(V,1)                :- active(V), vertex(V), obs_v_label(V,1),   not trivial(V), not input(V).
vlabel(U,1)                :- active(V), edge(U,V), obs_v_label(U,1),   not trivial(V), not input(V).

vlabel(V,-1)               :- active(V), vertex(V), obs_v_label(V,-1),   not trivial(V), not input(V).
vlabel(U,-1)               :- active(V), edge(U,V), obs_v_label(U,-1),   not trivial(V), not input(V).

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

vlabel(V,S)    :- bot, vertex(V), sign(S),      not trivial(V), not input(V), not obs_v_label(V,T)    : sign(T).
vlabel(U,S)    :- bot, edge(U,V), sign(S),      not trivial(V), not input(V), not obs_v_label(U,T)    : sign(T).
elabel(U,V,1)  :- bot, edge(U,V),               not trivial(V), not input(V), not obss_elabel(U,V,T) : zign(T).
elabel(U,V,-1) :- bot, edge(U,V),               not trivial(V), not input(V), not obss_elabel(U,V,T) : zign(T).
:- not bot.


#heuristic active(X). [1,false]
#show  active/1.
";

pub const PRG_REMOVE_EDGES: &'static str = "
0{remedge(U,V,S)}1 :- not mandatory(U,V), obs_e_label(U,V,S).
0{remedge(U,V,1); remedge(U,V,-1)}2 :- not mandatory(U,V), edge(U,V), not obs_e_label(U,V,1), not obs_e_label(U,V,-1).
";

pub const PRG_MIN_WEIGHTED_REPAIRS: &'static str = "
#minimize{ 1@1, U,V,S : remedge(U,V,S)}.
#minimize{ 2@1, U,V,S : addedge(U,V,S)}.
#minimize{ 2@1, V : addeddy(V) }.
";

pub const PRG_BEST_ONE_EDGE: &'static str = "
% guess one edge end to add
0{addeddy(or(V))}  :-     vertex(or(V)).

% new inputs through repair
input(E,\"unknown\")      :- exp(E).
vertex(\"unknown\").
elabel(\"unknown\", V,1)  :- addeddy(V).
:- addeddy(U), addeddy(V),U!=V.";

pub const PRG_BEST_EDGE_START: &'static str = "
% guess one edge start to add
0{addedge(or(V),X,1); addedge(or(V),X,-1)}1 :- vertex(or(V)), edge_end(X).

% input(E,or(\"unknown\"))      :- exp(E).
% vertex(or(\"unknown\")).

% add only one edge !!!
:- addedge(Y1,X,1), addedge(Y2,X,-1).
:- addedge(Y1,X,S), addedge(Y2,X,T), Y1!=Y2.

% new inputs through repair

elabel(U,V,1) :- addedge(U,V,1).
elabel(U,V,-1) :-addedge(U,V,-1).";

pub const PRG_ADD_EDGES: &'static str = "
% guess edges to add
0{addedge(or(U),or(V),1); addedge(or(U),or(V),-1)}1 :- vertex(or(U)), vertex(or(V)), U!=V,
                                               not obs_e_label(or(U),or(V),1), not obs_e_label(or(U),or(V),-1).

% labels for newly added edgesm
elabel(U,V,1) :- addedge(U,V,1).
elabel(U,V,-1) :-addedge(U,V,-1).
";

pub const PRG_FLIP_EDGES: &'static str = "
0{flip(U,V,S)}1 :- not mandatory(U,V), obs_e_label(U,V,S).


% labels for fliped edges
remedge(U,V,S) :- flip(U,V,S).
elabel(V,U,S) :- flip(U,V,S).
";
