use super::misc::*;
use super::*;
use pretty_assertions::assert_eq;
use std::fs::File;

pub const TEST_NETWORK: &str = "data/test/gold_comp_BN.cif";
pub const TEST_OBSERVATIONS: &str = "data/test/test.obs";

#[test]
fn test_iggy_setup() {
    let file = File::open(TEST_NETWORK).unwrap();
    let ggraph = cif_parser::read(&file).unwrap();
    let graph = ggraph.to_facts();
    let network_statistics = ggraph.statistics();
    let out = format!("{network_statistics}");
    assert_eq!(
        out,
        r"- OR nodes (species): 23
- AND nodes (complex regulation): 12
- Activations: 47
- Inhibitions: 11
- Unknowns:    0"
            .to_string()
    );
    let serialized = serde_json::to_string(&network_statistics).unwrap();
    assert_eq!(
        serialized,
        "{\"or_nodes\":23,\"and_nodes\":12,\"activations\":47,\"inhibitions\":11,\"unknowns\":0}"
            .to_string()
    );

    let file = File::open(TEST_OBSERVATIONS).unwrap();
    let pprofile = profile_parser::read(&file, "x1").unwrap();

    let observations_statistics = observations_statistics(&pprofile, &ggraph);
    let mut buf = Vec::new();
    write_observation_statistics(&mut buf, &observations_statistics).unwrap();
    assert_eq!(
        std::str::from_utf8(&buf).unwrap(),
        r"- Observed model nodes:   23
- Unobserved model nodes: 0
- Observed not in model:  0
- Inputs:                 2
- MIN:                    0
- MAX:                    0
- Observations:           23
  - +:                    7
  - -:                    5
  - 0:                    10
  - notPlus:              1
  - notMinus:             0
"
    );
    let serialized = serde_json::to_string(&observations_statistics).unwrap();
    assert_eq!(
        serialized,
        "{\"observed\":23,\"unobserved\":0,\"not_in_model\":0,\"inputs\":2,\"min\":0,\"max\":0,\"observations\":23,\"plus\":7,\"minus\":5,\"zero\":10,\"not_plus\":1,\"not_minus\":0}".to_string()
    );

    let profile = pprofile.to_facts();
    let check_result = check_observations(&profile).unwrap();
    assert_eq!(format!("{:?}", check_result), "Consistent".to_owned());

    let auto_inputs = get_auto_inputs(&graph).unwrap();
    let mut node_ids = get_node_ids_from_inputs(&auto_inputs).unwrap();
    node_ids.sort();
    let mut buf = Vec::new();
    write_auto_inputs_json(&mut buf, &node_ids).unwrap();
    assert_eq!(
        std::str::from_utf8(&buf).unwrap(),
        ",\"computed input nodes\":[\"cis\",\"depor\",\"mtor_inhibitor\",\"socs1\",\"socs3\"]\n"
            .to_owned()
    )
}

#[test]
fn test_get_scenfit_labelings() {
    let setting = Setting {
        os: false,
        ep: true,
        fp: true,
        fc: true,
    };
    let file = File::open(TEST_NETWORK).unwrap();
    let ggraph = cif_parser::read(&file).unwrap();
    let graph = ggraph.to_facts();
    let file = File::open(TEST_OBSERVATIONS).unwrap();
    let pprofile = profile_parser::read(&file, "x1").unwrap();
    let profile = pprofile.to_facts();
    let auto_inputs = get_auto_inputs(&graph).unwrap();

    let scenfit = get_scenfit(&graph, &profile, &auto_inputs, &setting, 1).unwrap();
    assert_eq!(scenfit, 1);

    let l = get_scenfit_labelings(&graph, &profile, &auto_inputs, 0, &setting, 1).unwrap();
    let mut l: Vec<(Vec<Prediction>, Vec<RepairOp>)> = l
        .map(|(a, b)| {
            let mut a2 = a.clone();
            a2.sort();
            let mut b2 = b.clone();
            b2.sort();
            (a2, b2)
        })
        .collect();
    l.sort();
    let l = l.into_iter();
    let mut buf = Vec::new();
    write_labelings(&mut buf, l.clone()).unwrap();
    assert_eq!(
        std::str::from_utf8(&buf).unwrap(),
        r"
## Possible labelings under repair

1. Labeling:
  - akt = +
  - akt = -
  - akt = 0
  - cis = 0
  - depor = -
  - erk = +
  - gab1_bpi3k_py = +
  - gab1_bpi3k_py = -
  - gab1_bpi3k_py = 0
  - gab1_bras_py = +
  - gab1_bras_py = -
  - gab1_bras_py = 0
  - gab1_bshp2_ph_py = +
  - gab1_bshp2_ph_py = -
  - gab1_bshp2_ph_py = 0
  - gab1_ps = +
  - grb2_sos = -
  - grb2_sos = 0
  - jak2_p = -
  - jak2_p = 0
  - mek1 = +
  - mtor = +
  - mtor = -
  - mtor = 0
  - mtor_inhibitor = +
  - mtorc1 = +
  - mtorc1 = -
  - mtorc1 = 0
  - mtorc2 = +
  - mtorc2 = -
  - mtorc2 = 0
  - pi3k = +
  - pi3k = -
  - pi3k = 0
  - plcg = +
  - plcg = -
  - plcg = 0
  - ras_gap = +
  - ras_gap = -
  - ras_gap = 0
  - shp2 = -
  - shp2 = 0
  - shp2_ph = +
  - shp2_ph = -
  - shp2_ph = 0
  - socs1 = 0
  - socs3 = 0
  - stat5ab_py = -
  - stat5ab_py = 0

  Repair set:
  - flip jak2_p: + to 0
"
    );
    let mut buf = Vec::new();
    write_json_labelings(&mut buf, l).unwrap();
    assert_eq!(
        std::str::from_utf8(&buf).unwrap(),
        ",\"labels under repair\":[
  {\"labels\":[{\"node\":\"akt\",\"behavior\":\"+\"},{\"node\":\"akt\",\"behavior\":\"-\"},{\"node\":\"akt\",\"behavior\":\"0\"},{\"node\":\"cis\",\"behavior\":\"0\"},{\"node\":\"depor\",\"behavior\":\"-\"},{\"node\":\"erk\",\"behavior\":\"+\"},{\"node\":\"gab1_bpi3k_py\",\"behavior\":\"+\"},{\"node\":\"gab1_bpi3k_py\",\"behavior\":\"-\"},{\"node\":\"gab1_bpi3k_py\",\"behavior\":\"0\"},{\"node\":\"gab1_bras_py\",\"behavior\":\"+\"},{\"node\":\"gab1_bras_py\",\"behavior\":\"-\"},{\"node\":\"gab1_bras_py\",\"behavior\":\"0\"},{\"node\":\"gab1_bshp2_ph_py\",\"behavior\":\"+\"},{\"node\":\"gab1_bshp2_ph_py\",\"behavior\":\"-\"},{\"node\":\"gab1_bshp2_ph_py\",\"behavior\":\"0\"},{\"node\":\"gab1_ps\",\"behavior\":\"+\"},{\"node\":\"grb2_sos\",\"behavior\":\"-\"},{\"node\":\"grb2_sos\",\"behavior\":\"0\"},{\"node\":\"jak2_p\",\"behavior\":\"-\"},{\"node\":\"jak2_p\",\"behavior\":\"0\"},{\"node\":\"mek1\",\"behavior\":\"+\"},{\"node\":\"mtor\",\"behavior\":\"+\"},{\"node\":\"mtor\",\"behavior\":\"-\"},{\"node\":\"mtor\",\"behavior\":\"0\"},{\"node\":\"mtor_inhibitor\",\"behavior\":\"+\"},{\"node\":\"mtorc1\",\"behavior\":\"+\"},{\"node\":\"mtorc1\",\"behavior\":\"-\"},{\"node\":\"mtorc1\",\"behavior\":\"0\"},{\"node\":\"mtorc2\",\"behavior\":\"+\"},{\"node\":\"mtorc2\",\"behavior\":\"-\"},{\"node\":\"mtorc2\",\"behavior\":\"0\"},{\"node\":\"pi3k\",\"behavior\":\"+\"},{\"node\":\"pi3k\",\"behavior\":\"-\"},{\"node\":\"pi3k\",\"behavior\":\"0\"},{\"node\":\"plcg\",\"behavior\":\"+\"},{\"node\":\"plcg\",\"behavior\":\"-\"},{\"node\":\"plcg\",\"behavior\":\"0\"},{\"node\":\"ras_gap\",\"behavior\":\"+\"},{\"node\":\"ras_gap\",\"behavior\":\"-\"},{\"node\":\"ras_gap\",\"behavior\":\"0\"},{\"node\":\"shp2\",\"behavior\":\"-\"},{\"node\":\"shp2\",\"behavior\":\"0\"},{\"node\":\"shp2_ph\",\"behavior\":\"+\"},{\"node\":\"shp2_ph\",\"behavior\":\"-\"},{\"node\":\"shp2_ph\",\"behavior\":\"0\"},{\"node\":\"socs1\",\"behavior\":\"0\"},{\"node\":\"socs3\",\"behavior\":\"0\"},{\"node\":\"stat5ab_py\",\"behavior\":\"-\"},{\"node\":\"stat5ab_py\",\"behavior\":\"0\"}],
   \"repairs\":[{\"FlipNodeSign\":{\"profile\":\"x1\",\"node\":\"jak2_p\",\"direction\":\"PlusToZero\"}}]
  }
]
"
    );
}
#[test]
fn test_get_scenfit_predictions() {
    let setting = Setting {
        os: false,
        ep: true,
        fp: true,
        fc: true,
    };
    let file = File::open(TEST_NETWORK).unwrap();
    let ggraph = cif_parser::read(&file).unwrap();
    let graph = ggraph.to_facts();
    let file = File::open(TEST_OBSERVATIONS).unwrap();
    let pprofile = profile_parser::read(&file, "x1").unwrap();
    let profile = pprofile.to_facts();
    let auto_inputs = get_auto_inputs(&graph).unwrap();

    let scenfit = get_scenfit(&graph, &profile, &auto_inputs, &setting, 1).unwrap();
    assert_eq!(scenfit, 1);

    let mut l = get_predictions_under_scenfit(&graph, &profile, &auto_inputs, &setting).unwrap();
    l.sort();
    let mut buf = Vec::new();
    write_predictions(&mut buf, &l).unwrap();
    assert_eq!(
        std::str::from_utf8(&buf).unwrap(),
        r"
## Predictions

- cis = 0
- depor = -
- erk = +
- gab1_ps = +
- grb2_sos = notPlus
- jak2_p = notPlus
- mek1 = +
- mtor_inhibitor = +
- shp2 = notPlus
- socs1 = 0
- socs3 = 0
- stat5ab_py = notPlus

## Prediction statistics

- predicted +        : 4
- predicted -        : 1
- predicted 0        : 3
- predicted notPlus  : 4
- predicted notMinus : 0
- predicted CHANGE   : 0
"
    );

    let mut buf = Vec::new();
    write_json_predictions(&mut buf, &l).unwrap();
    assert_eq!(
        std::str::from_utf8(&buf).unwrap(),
        ",\"predictions\":[{\"node\":\"cis\",\"behavior\":\"0\"},{\"node\":\"depor\",\"behavior\":\"-\"},{\"node\":\"erk\",\"behavior\":\"+\"},{\"node\":\"gab1_ps\",\"behavior\":\"+\"},{\"node\":\"grb2_sos\",\"behavior\":\"notPlus\"},{\"node\":\"jak2_p\",\"behavior\":\"notPlus\"},{\"node\":\"mek1\",\"behavior\":\"+\"},{\"node\":\"mtor_inhibitor\",\"behavior\":\"+\"},{\"node\":\"shp2\",\"behavior\":\"notPlus\"},{\"node\":\"socs1\",\"behavior\":\"0\"},{\"node\":\"socs3\",\"behavior\":\"0\"},{\"node\":\"stat5ab_py\",\"behavior\":\"notPlus\"}]\n");
}

#[test]
fn test_get_mcos_labelings() {
    let setting = Setting {
        os: false,
        ep: true,
        fp: true,
        fc: true,
    };
    let file = File::open(TEST_NETWORK).unwrap();
    let ggraph = cif_parser::read(&file).unwrap();
    let graph = ggraph.to_facts();
    let file = File::open(TEST_OBSERVATIONS).unwrap();
    let pprofile = profile_parser::read(&file, "x1").unwrap();
    let profile = pprofile.to_facts();
    let auto_inputs = get_auto_inputs(&graph).unwrap();

    let mcos = get_mcos(&graph, &profile, &auto_inputs, &setting, 1).unwrap();
    assert_eq!(mcos, 3);

    let l = get_mcos_labelings(&graph, &profile, &auto_inputs, 0, &setting, 1).unwrap();
    let mut l: Vec<(Vec<Prediction>, Vec<RepairOp>)> = l
        .map(|(a, b)| {
            let mut a2 = a.clone();
            a2.sort();
            let mut b2 = b.clone();
            b2.sort();
            (a2, b2)
        })
        .collect();
    l.sort();
    let l = l.into_iter();
    let mut buf = Vec::new();
    write_labelings(&mut buf, l.clone()).unwrap();
    assert_eq!(
        std::str::from_utf8(&buf).unwrap(),
        r"
## Possible labelings under repair

1. Labeling:
  - akt = -
  - cis = 0
  - depor = -
  - erk = +
  - gab1_bpi3k_py = 0
  - gab1_bras_py = 0
  - gab1_bshp2_ph_py = +
  - gab1_ps = +
  - grb2_sos = -
  - grb2_sos = 0
  - jak2_p = +
  - mek1 = +
  - mtor = 0
  - mtor_inhibitor = +
  - mtorc1 = -
  - mtorc2 = -
  - pi3k = 0
  - plcg = +
  - ras_gap = 0
  - shp2 = 0
  - shp2_ph = -
  - socs1 = 0
  - socs3 = 0
  - stat5ab_py = 0
  - unknownup = +

  Repair set:
  - new decreasing influence on gab1_bpi3k_py
  - new increasing influence on jak2_p
  - new decreasing influence on shp2_ph
"
    );
    let mut buf = Vec::new();
    write_json_labelings(&mut buf, l).unwrap();
    assert_eq!(
        std::str::from_utf8(&buf).unwrap(),
        ",\"labels under repair\":[
  {\"labels\":[{\"node\":\"akt\",\"behavior\":\"-\"},{\"node\":\"cis\",\"behavior\":\"0\"},{\"node\":\"depor\",\"behavior\":\"-\"},{\"node\":\"erk\",\"behavior\":\"+\"},{\"node\":\"gab1_bpi3k_py\",\"behavior\":\"0\"},{\"node\":\"gab1_bras_py\",\"behavior\":\"0\"},{\"node\":\"gab1_bshp2_ph_py\",\"behavior\":\"+\"},{\"node\":\"gab1_ps\",\"behavior\":\"+\"},{\"node\":\"grb2_sos\",\"behavior\":\"-\"},{\"node\":\"grb2_sos\",\"behavior\":\"0\"},{\"node\":\"jak2_p\",\"behavior\":\"+\"},{\"node\":\"mek1\",\"behavior\":\"+\"},{\"node\":\"mtor\",\"behavior\":\"0\"},{\"node\":\"mtor_inhibitor\",\"behavior\":\"+\"},{\"node\":\"mtorc1\",\"behavior\":\"-\"},{\"node\":\"mtorc2\",\"behavior\":\"-\"},{\"node\":\"pi3k\",\"behavior\":\"0\"},{\"node\":\"plcg\",\"behavior\":\"+\"},{\"node\":\"ras_gap\",\"behavior\":\"0\"},{\"node\":\"shp2\",\"behavior\":\"0\"},{\"node\":\"shp2_ph\",\"behavior\":\"-\"},{\"node\":\"socs1\",\"behavior\":\"0\"},{\"node\":\"socs3\",\"behavior\":\"0\"},{\"node\":\"stat5ab_py\",\"behavior\":\"0\"},{\"node\":\"unknownup\",\"behavior\":\"+\"}],
   \"repairs\":[{\"NewInfluence\":{\"profile\":\"x1\",\"target\":\"gab1_bpi3k_py\",\"sign\":\"Minus\"}},{\"NewInfluence\":{\"profile\":\"x1\",\"target\":\"jak2_p\",\"sign\":\"Plus\"}},{\"NewInfluence\":{\"profile\":\"x1\",\"target\":\"shp2_ph\",\"sign\":\"Minus\"}}]
  }
]
"
    );
}
#[test]
fn test_get_mcos_predictions() {
    let setting = Setting {
        os: false,
        ep: true,
        fp: true,
        fc: true,
    };
    let file = File::open(TEST_NETWORK).unwrap();
    let ggraph = cif_parser::read(&file).unwrap();
    let graph = ggraph.to_facts();
    let file = File::open(TEST_OBSERVATIONS).unwrap();
    let pprofile = profile_parser::read(&file, "x1").unwrap();
    let profile = pprofile.to_facts();
    let auto_inputs = get_auto_inputs(&graph).unwrap();

    let scenfit = get_scenfit(&graph, &profile, &auto_inputs, &setting, 1).unwrap();
    assert_eq!(scenfit, 1);

    let mut l = get_predictions_under_mcos(&graph, &profile, &auto_inputs, &setting).unwrap();
    l.sort();
    let mut buf = Vec::new();
    write_predictions(&mut buf, &l).unwrap();
    assert_eq!(
        std::str::from_utf8(&buf).unwrap(),
        r"
## Predictions

- akt = -
- cis = 0
- depor = -
- erk = +
- gab1_bpi3k_py = 0
- gab1_bras_py = 0
- gab1_bshp2_ph_py = +
- gab1_ps = +
- grb2_sos = notPlus
- jak2_p = +
- mek1 = +
- mtor = 0
- mtor_inhibitor = +
- mtorc1 = -
- mtorc2 = -
- pi3k = 0
- plcg = +
- ras_gap = 0
- shp2 = 0
- shp2_ph = -
- socs1 = 0
- socs3 = 0
- stat5ab_py = 0
- unknownup = +

## Prediction statistics

- predicted +        : 8
- predicted -        : 5
- predicted 0        : 10
- predicted notPlus  : 1
- predicted notMinus : 0
- predicted CHANGE   : 0
"
    );

    let mut buf = Vec::new();
    write_json_predictions(&mut buf, &l).unwrap();
    assert_eq!(
        std::str::from_utf8(&buf).unwrap(),
        ",\"predictions\":[{\"node\":\"akt\",\"behavior\":\"-\"},{\"node\":\"cis\",\"behavior\":\"0\"},{\"node\":\"depor\",\"behavior\":\"-\"},{\"node\":\"erk\",\"behavior\":\"+\"},{\"node\":\"gab1_bpi3k_py\",\"behavior\":\"0\"},{\"node\":\"gab1_bras_py\",\"behavior\":\"0\"},{\"node\":\"gab1_bshp2_ph_py\",\"behavior\":\"+\"},{\"node\":\"gab1_ps\",\"behavior\":\"+\"},{\"node\":\"grb2_sos\",\"behavior\":\"notPlus\"},{\"node\":\"jak2_p\",\"behavior\":\"+\"},{\"node\":\"mek1\",\"behavior\":\"+\"},{\"node\":\"mtor\",\"behavior\":\"0\"},{\"node\":\"mtor_inhibitor\",\"behavior\":\"+\"},{\"node\":\"mtorc1\",\"behavior\":\"-\"},{\"node\":\"mtorc2\",\"behavior\":\"-\"},{\"node\":\"pi3k\",\"behavior\":\"0\"},{\"node\":\"plcg\",\"behavior\":\"+\"},{\"node\":\"ras_gap\",\"behavior\":\"0\"},{\"node\":\"shp2\",\"behavior\":\"0\"},{\"node\":\"shp2_ph\",\"behavior\":\"-\"},{\"node\":\"socs1\",\"behavior\":\"0\"},{\"node\":\"socs3\",\"behavior\":\"0\"},{\"node\":\"stat5ab_py\",\"behavior\":\"0\"},{\"node\":\"unknownup\",\"behavior\":\"+\"}]\n");
}

#[test]
fn test_get_mics() {
    let setting = Setting {
        os: false,
        ep: true,
        fp: true,
        fc: true,
    };
    let file = File::open(TEST_NETWORK).unwrap();
    let ggraph = cif_parser::read(&file).unwrap();
    let graph = ggraph.to_facts();
    let file = File::open(TEST_OBSERVATIONS).unwrap();
    let pprofile = profile_parser::read(&file, "x1").unwrap();
    let profile = pprofile.to_facts();
    let auto_inputs = FactBase::new();

    let mics = get_minimal_inconsistent_cores(&graph, &profile, &auto_inputs, &setting, 1).unwrap();
    let mut mics: Vec<Vec<NodeId>> = mics
        .map(|a| {
            let mut a2 = a.clone();
            a2.sort();
            a2
        })
        .collect();
    mics.sort();
    let mics = mics.into_iter();
    let mut buf = Vec::new();
    write_mics(&mut buf, mics.clone()).unwrap();
    assert_eq!(
        std::str::from_utf8(&buf).unwrap(),
        r"
## Minimal inconsistent cores

1. Mic:
  - cis
  - mtor_inhibitor
  - socs1
  - socs3
"
    );
    let mut buf = Vec::new();
    write_json_mics(&mut buf, mics).unwrap();
    assert_eq!(
        std::str::from_utf8(&buf).unwrap(),
        ",\"mics\":[\n  [\"cis\",\"mtor_inhibitor\",\"socs1\",\"socs3\"]\n]\n"
    );
}

#[test]
fn test_optgraph_setup() {}
