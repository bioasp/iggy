pub const PRG_CONTRADICTORY_OBS: &'static str = "% two contradictory observations
contradiction(E,X,r1) :- obs_vlabel(E,X,1), obs_vlabel(E,X,0).
contradiction(E,X,r2) :- obs_vlabel(E,X,-1), obs_vlabel(E,X,0).
contradiction(E,X,r3) :- obs_vlabel(E,X,-1), obs_vlabel(E,X,1).
contradiction(E,X,r4) :- obs_vlabel(E,X,notMinus), obs_vlabel(E,X,-1).
contradiction(E,X,r5) :- obs_vlabel(E,X,notPlus), obs_vlabel(E,X,1).

% contradictions of observed behavior and initial level
contradiction(E,X,r6) :- obs_vlabel(E,X,-1), ismin(E,X).
contradiction(E,X,r7) :- obs_vlabel(E,X,1), ismax(E,X).
#show contradiction/3.
";

pub const PRG_GUESS_INPUTS: &'static str = "edge(X,Y) :- obs_elabel(X,Y,S).
vertex(X) :- edge(X,Y).
vertex(Y) :- edge(X,Y).
% declare inputs
input(V) :- vertex(V), not edge(U,V) : edge(U,V), U != V.
#show input/1.
";

pub const PRG_SIGN_CONS: &'static str = "% input facts
% obs_vlabel(Experiment, Vertex , Sign)
% edge(X,Y)
% obs_elabel(X,Y,Sign).
% vertex(Name)

sign(1;-1;0).
obs(1;-1;0;notPlus;notMinus).
exp(E) :- obs_vlabel(E,V,S).
exp(E) :- input(E,V).
vertex(V) :- obs_vlabel(E,V,S).
edge(X,Y) :- obs_elabel(X,Y,S).
vertex(X) :- edge(X,Y).
vertex(Y) :- edge(X,Y).

% for each vertex the measurements are either changing (1,-1) or not (0)
1 {vlabel(E,V,1); vlabel(E,V,-1); vlabel(E,V,0)} :- vertex(V), exp(E).
1 {elabel(U,V,1); elabel(U,V,-1)} 1 :- edge(U,V), not rep(remedge(U,V,1)), not rep(remedge(U,V,-1)).

% keep observed labeling of the edges
error_edge(U,V) :- obs_elabel(U,V,1), obs_elabel(U,V,-1).
elabel(U,V,S) :- edge(U,V), obs_elabel(U,V,S), not error_edge(U,V), not rep(remedge(U,V,S)).

% how to hande error_edges
elabel(U,V,1)  :- edge(U,V), obs_elabel(U,V,1), obs_elabel(U,V,-1), not rep(remedge(U,V,1)), rep(remedge(U,V,-1)).
elabel(U,V,-1) :- edge(U,V), obs_elabel(U,V,1), obs_elabel(U,V,-1), rep(remedge(U,V,1)), not rep(remedge(U,V,-1)).

% influences
infl(E,V,S*T) :- elabel(U,V,S), vlabel(E,U,T).
% effects of a repair
infl(E,V,S) :- rep(new_influence(E,V,S)).

% pure influences
pinfl(E,V, 1) :- elabel(U,V, 1), vlabel(E,U, 1), not vlabel(E,U,0), not vlabel(E,U,-1).
pinfl(E,V,-1) :- elabel(U,V,-1), vlabel(E,U, 1), not vlabel(E,U,0), not vlabel(E,U,-1).
pinfl(E,V,-1) :- elabel(U,V, 1), vlabel(E,U,-1), not vlabel(E,U,0), not vlabel(E,U, 1).
pinfl(E,V, 1) :- elabel(U,V,-1), vlabel(E,U,-1), not vlabel(E,U,0), not vlabel(E,U, 1).
% effects of a repair
pinfl(E,V,S) :- rep(new_influence(E,V,S)).

% if a node has been observed only one sign
% forbidden(E,V,T) :- vlabel(E,V,S1), obs_vlabel(E,V,S), sign(S), sign(T), T!=S1.
% forbidden(E,V,T) :- vlabel(E,V,S1), err(flip(E,V,N)), obs_vlabel(E,V,S), sign(T), T!=S1.

% constraints
:- forbidden(E,V,S), vlabel(E,V,S).

% levelbound constraints
% if the initial level is at the minimum it cannot decrease anymore
forbidden(E,V,-1) :- ismin(E,V).
% if the initial level is at the maximum it cannot increase anymore
forbidden(E,V, 1) :- ismax(E,V).
";

pub const PRG_BWD_PROP: &'static str = "% measured variations must be explained by  predecessor
forbidden(E,V, 1) :- exp(E), vertex(V), not infl(E,V, 1), not input(E,V).
forbidden(E,V,-1) :- exp(E), vertex(V), not infl(E,V,-1), not input(E,V).
";

pub const PRG_ONE_STATE: &'static str = "forbidden(E,V,S) :- vlabel(E,V,T), sign(S), S!=T.";

pub const PRG_FWD_PROP: &'static str = "% propagate forward
vlabel(E,V, 0) :- exp(E), vertex(V), not forbidden(E,V, 0).
vlabel(E,V, 1) :- infl(E,V, 1), not forbidden(E,V, 1).
vlabel(E,V,-1) :- infl(E,V,-1), not forbidden(E,V,-1).

% constraint zero for or nodes
forbidden(E,or(V),0) :- pinfl(E,or(V), 1), not infl(E,or(V),-1), not input(E,or(V)), not ismax(E,or(V)).
forbidden(E,or(V),0) :- pinfl(E,or(V),-1), not infl(E,or(V), 1), not input(E,or(V)), not ismin(E,or(V)) .

% constraint zero for and nodes
forbidden(E,and(V),0) :- infl(E,and(V), 1), not infl(E,and(V),-1), not infl(E,and(V),0), not input(E,and(V)), not ismax(E,and(V)).
forbidden(E,and(V),0) :- infl(E,and(V),-1), not infl(E,and(V), 1), not infl(E,and(V),0), not input(E,and(V)), not ismin(E,and(V)).

% constraint zero for depmat does not work with and nodes
forbidden(E,or(V), 0) :- prinfl(E,or(V), 1), not neg_path(E,or(V)), not input(E,or(V)), not ismax(E,or(V)).
forbidden(E,or(V), 0) :- prinfl(E,or(V),-1), not pos_path(E,or(V)), not input(E,or(V)), not ismin(E,or(V)).
";

pub const PRG_FOUNDEDNESS: &'static str = "% founded rules
founded(E,X,-1) :- input(E,X).
founded(E,X, 1) :- input(E,X).
founded(E,X,-1) :- founded(E,Y,-1), elabel(Y,X, 1).
founded(E,X,-1) :- founded(E,Y, 1), elabel(Y,X,-1).
founded(E,X, 1) :- founded(E,Y,-1), elabel(Y,X,-1).
founded(E,X, 1) :- founded(E,Y, 1), elabel(Y,X, 1).

founded(E,X,S) :- vlabel(E,X,S), rep(new_influence(E,X,S)).

forbidden(E,V, 1):- exp(E), vertex(V), not founded(E,V, 1).
forbidden(E,V,-1):- exp(E), vertex(V), not founded(E,V,-1).
";

pub const PRG_ELEM_PATH: &'static str = "
% new inputs through repair
input(E,or(\"unknownup\"))    :- rep(new_influence(E,X,S)).
vlabel(E,or(\"unknownup\"),1) :- rep(new_influence(E,X,S)).
elabel(or(\"unknownup\"), X, 1)   :- rep(new_influence(E,X,1)).     
elabel(or(\"unknownup\"), X,-1)   :- rep(new_influence(E,X,-1)).     

% in a network exists under Condition E a positive path to X

pos_path(E,X,@str(X)) :- input(E,X), vlabel(E,X, 1), not ismax(E,X).

neg_path(E,X,@str(X)) :- input(E,X), vlabel(E,X,-1), not ismin(E,X).

pos_path(E,X,@strconc(P,X)) :- pos_path(E,Y,P), not ismax(E,X), 
                               elabel(Y,X, 1), not input(E,X), X!=Y,
	                       0==@member(X,P).     

                          
neg_path(E,X,@strconc(P,X)) :- pos_path(E,Y,P), not ismin(E,X), 
                               elabel(Y,X,-1), not input(E,X), X!=Y,
	                       0==@member(X,P).

pos_path(E,X,@strconc(P,X)) :- neg_path(E,Y,P), not ismax(E,X), 
                               elabel(Y,X,-1), not input(E,X), X!=Y,
	                       0==@member(X,P).     
                       
neg_path(E,X,@strconc(P,X)) :- neg_path(E,Y,P), not ismin(E,X), 
                               elabel(Y,X, 1), not input(E,X), X!=Y,
	                       0==@member(X,P).

pos_path(E,V) :- pos_path(E,V,P).
neg_path(E,V) :- neg_path(E,V,P).



% pure influences
prinfl(E,V, 1) :- elabel(U,V, 1),
                  pos_path(E,V,P),1==@member(U,P),
                  vlabel(E,U, 1), not vlabel(E,U,0), not vlabel(E,U,-1).
prinfl(E,V,-1) :- elabel(U,V,-1),
                  neg_path(E,V,P),1==@member(U,P),
                  vlabel(E,U, 1), not vlabel(E,U,0), not vlabel(E,U,-1).
prinfl(E,V,-1) :- elabel(U,V, 1),
                  neg_path(E,V,P),1==@member(U,P),
                  vlabel(E,U,-1), not vlabel(E,U,0), not vlabel(E,U, 1).
prinfl(E,V, 1) :- elabel(U,V,-1),
                  pos_path(E,V,P),1==@member(U,P),
                  vlabel(E,U,-1), not vlabel(E,U,0), not vlabel(E,U, 1).

forbidden(E,V, 1) :- exp(E), vertex(V), not pos_path(E,V), not input(E,V).
forbidden(E,V,-1) :- exp(E), vertex(V), not neg_path(E,V), not input(E,V).
";

pub const PRG_ERROR_MEASURE: &'static str =
    "err(flip(E,X,1)) :- obs_vlabel(E,X, 1), not vlabel(E,X, 1),     vlabel(E,X,0).
err(flip(E,X,2)) :- obs_vlabel(E,X, 1), not vlabel(E,X, 1), not vlabel(E,X,0), vlabel(E,X,-1).

err(flip(E,X,1)) :- obs_vlabel(E,X,-1), not vlabel(E,X,-1),     vlabel(E,X,0).
err(flip(E,X,2)) :- obs_vlabel(E,X,-1), not vlabel(E,X,-1), not vlabel(E,X,0), vlabel(E,X, 1).

err(flip(E,X,1)) :- obs_vlabel(E,X, 0), not vlabel(E,X, 0).

err(flip(E,X,2)) :- obs_vlabel(E,X, notMinus), not vlabel(E,X, 0), not vlabel(E,X, 1).
err(flip(E,X,2)) :- obs_vlabel(E,X, notPlus), not vlabel(E,X, 0), not vlabel(E,X,-1).";

pub const PRG_MIN_WEIGHTED_ERROR: &'static str = "#minimize{V@2 : err(flip(E,X,V)) }.
scenfit(S) :-  S = #sum {1 : err(flip(E,X,V)) }.
#show err/1.
#show scenfit/1.
";

pub const PRG_KEEP_INPUTS: &'static str = "% keep observed input variation
forbidden(E,V,T) :- input(E,V), obs_vlabel(E,V,S), sign(S), sign(T), S!=T.

% A weak input
forbidden(E,V, 1) :- input(E,V), obs_vlabel(E,V,notPlus).
forbidden(E,V,-1) :- input(E,V), obs_vlabel(E,V,notMinus).";

// pub const PRG_SCENFIT = PRG_ERROR_MEASURE+ PRG_MIN_WEIGHTED_ERROR+ PRG_KEEP_INPUTS;
