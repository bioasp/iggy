// preprocessing
pub const PRG_CONTRADICTORY_OBS: &str = include_str!("encodings/contradictory_obs.lp");
pub const PRG_GUESS_INPUTS: &str = include_str!("encodings/guess_inputs.lp");

// minimal inconsistent cores
pub const PRG_MICS: &str = include_str!("encodings/mics.lp");

// basic sign consistency
pub const PRG_SIGN_CONS: &str = include_str!("encodings/sign_cons.lp");

// additional constraints
pub const PRG_BWD_PROP: &str = include_str!("encodings/bwd_prop.lp");
pub const PRG_FWD_PROP: &str = include_str!("encodings/fwd_prop.lp");
pub const PRG_FOUNDEDNESS: &str = include_str!("encodings/foundedness.lp");
pub const PRG_ELEM_PATH: &str = include_str!("encodings/elem_path.lp");
pub const PRG_ONE_STATE: &str = include_str!("encodings/one_state.lp");

pub const PRG_KEEP_INPUTS: &str = include_str!("encodings/keep_inputs.lp");
pub const PRG_KEEP_OBSERVATIONS: &str = include_str!("encodings/keep_observations.lp");

pub const PRG_ERROR_MEASURE: &str = include_str!("encodings/error_measure.lp");
pub const PRG_MIN_WEIGHTED_ERROR: &str = include_str!("encodings/min_weighted_error.lp");

pub const PRG_PREDICTIONS: &str = include_str!("encodings/predictions.lp");
pub const PRG_PREDICTIONS_DM: &str = include_str!("encodings/predictions_depmat.lp");

// repair operations
pub const PRG_ADD_INFLUENCES: &str = include_str!("encodings/add_influences.lp");
pub const PRG_MIN_ADDED_INFLUENCES: &str = include_str!("encodings/min_added_influences.lp");

pub const PRG_REMOVE_EDGES: &str = include_str!("encodings/remove_edges.lp");
pub const PRG_ADD_EDGES: &str = include_str!("encodings/add_edges.lp");
pub const PRG_FLIP_EDGE_DIRECTIONS: &str = include_str!("encodings/flip_edge_directions.lp");
pub const PRG_MIN_WEIGHTED_REPAIRS: &str = include_str!("encodings/min_weighted_repairs.lp");

pub const PRG_BEST_ONE_EDGE: &str = "
% guess one edge end to add
0{addeddy(or(V))}  :-     vertex(or(V)).

% new inputs through repair
input(E,\"unknown\")      :- exp(E).
vertex(\"unknown\").
elabel(\"unknown\", V,1)  :- addeddy(V).
:- addeddy(U), addeddy(V),U!=V.";

pub const PRG_BEST_EDGE_START: &str = "
% guess one edge start to add
0{addedge(or(V),X,1); addedge(or(V),X,-1)}1 :- vertex(or(V)), edge_end(X).

% add only one edge !!!
:- addedge(Y1,X,1), addedge(Y2,X,-1).
:- addedge(Y1,X,S), addedge(Y2,X,T), Y1!=Y2.";

pub const PRG_SHOW_ERRORS: &str = "
#show flip_node_sign_Plus_to_0/2.
#show flip_node_sign_Plus_to_Minus/2.
#show flip_node_sign_Minus_to_0/2.
#show flip_node_sign_Minus_to_Plus/2.
#show flip_node_sign_0_to_Plus/2.
#show flip_node_sign_0_to_Minus/2.
#show flip_node_sign_notMinus_to_Minus/2.
#show flip_node_sign_notPlus_to_Plus/2.
";
pub const PRG_SHOW_LABELS: &str = "
#show vlabel(X,or(V),S) : vlabel(X,or(V),S).
";
pub const PRG_SHOW_REPAIRS: &str = "
#show remedge/3.
#show addedge/3.
#show new_influence/3.
";
pub const PRG_SHOW_FLIP: &str = "#show flip/3.";
pub const PRG_SHOW_ADD_EDGE_END: &str = "#show addeddy/1.";
