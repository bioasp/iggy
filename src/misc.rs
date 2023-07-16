use super::cif_parser::Graph;
use super::profile_parser::{Observation, Profile};
use super::*;

fn find_node_in_observations(observations: &[Observation], node_id: &NodeId) -> bool {
    for obs in observations {
        if obs.node == *node_id {
            return true;
        }
    }
    false
}
fn find_node_in_nodes(nodes: &[NodeId], node_id: &NodeId) -> bool {
    for node in nodes {
        if *node == *node_id {
            return true;
        }
    }
    false
}

pub fn write_setting_json(mut out: impl Write, setting: &Setting) -> Result<(), std::io::Error> {
    writeln!(
        out,
        ",\"setting\":{{\"depmat\":{},\"elempath\":{},\"forward-propagation\":{},\"founded-constraints\":{}}}",
        !setting.os, setting.ep, setting.fp, setting.fc
    )
}

pub fn write_setting_md(mut out: impl Write, setting: &Setting) -> Result<(), std::io::Error> {
    writeln!(out, "\n## Settings\n")?;
    if !setting.os {
        writeln!(out, "- Dependency matrix combines multiple states.")?;
        writeln!(
            out,
            "- An elementary path from an input must exist to explain changes."
        )?;
    } else {
        writeln!(
            out,
            "- All observed changes must be explained by a predecessor."
        )?;

        if setting.ep {
            writeln!(
                out,
                "- An elementary path from an input must exist to explain changes."
            )?;
        }
        if setting.fp {
            writeln!(out, "- 0-change must be explained.")?;
        }
        if setting.fc {
            writeln!(out, "- All observed changes must be explained by an input.")?;
        }
    }
    write!(out, "")
}

#[derive(Serialize, Debug)]
pub struct ObservationsStatistics {
    observed: usize,     // observed nodes of the model
    unobserved: usize,   // unobserved nodes of the model
    not_in_model: usize, // observations without node in the model
    inputs: usize,       // number of inputs
    min: usize,          // number of MIN
    max: usize,          // number of MAX
    observations: usize, // total number of observations
    plus: usize,         // number of + observations
    minus: usize,        // number of - observations
    zero: usize,         // number of 0 observations
    not_plus: usize,     // number of NOT + observations
    not_minus: usize,    // number of NOT - observations
}

pub fn write_observation_statistics(
    mut out: impl Write,
    stats: &ObservationsStatistics,
) -> Result<(), std::io::Error> {
    // writeln!(out, "\n## Observations statistics\n")?;
    writeln!(out, "- Observed model nodes:   {}", stats.observed)?;
    writeln!(out, "- Unobserved model nodes: {}", stats.unobserved)?;
    writeln!(out, "- Observed not in model:  {}", stats.not_in_model)?;
    writeln!(out, "- Inputs:                 {}", stats.inputs)?;
    writeln!(out, "- MIN:                    {}", stats.min)?;
    writeln!(out, "- MAX:                    {}", stats.max)?;
    writeln!(out, "- Observations:           {}", stats.observations)?;
    writeln!(out, "  - +:                    {}", stats.plus)?;
    writeln!(out, "  - -:                    {}", stats.minus)?;
    writeln!(out, "  - 0:                    {}", stats.zero)?;
    writeln!(out, "  - notPlus:              {}", stats.not_plus)?;
    writeln!(out, "  - notMinus:             {}", stats.not_minus)?;
    Ok(())
}

pub fn observations_statistics(profile: &Profile, graph: &Graph) -> ObservationsStatistics {
    let model_nodes = graph.or_nodes();
    let mut unobserved = model_nodes.len();
    for node in model_nodes {
        if find_node_in_observations(&profile.observations, node) {
            unobserved -= 1;
        }
    }
    let observed = model_nodes.len() - unobserved;

    let mut plus = 0;
    let mut minus = 0;
    let mut zero = 0;
    let mut not_plus = 0;
    let mut not_minus = 0;
    for obs in &profile.observations {
        match obs.behavior {
            Behavior::Plus => plus += 1,
            Behavior::Minus => minus += 1,
            Behavior::Zero => zero += 1,
            Behavior::NotPlus => not_plus += 1,
            Behavior::NotMinus => not_minus += 1,
            Behavior::Change => panic!("Behavior Change not supported in observation"),
        }
    }

    let mut not_in_model = profile.observations.len();
    for obs in &profile.observations {
        if find_node_in_nodes(model_nodes, &obs.node) {
            not_in_model -= 1;
        }
    }
    ObservationsStatistics {
        observed,
        unobserved,
        not_in_model,
        inputs: profile.inputs.len(),
        min: profile.min.len(),
        max: profile.max.len(),
        observations: profile.observations.len(),
        plus,
        minus,
        zero,
        not_plus,
        not_minus,
    }
}

pub fn write_auto_inputs_md(
    mut out: impl Write,
    inputs: &Vec<NodeId>,
) -> Result<(), std::io::Error> {
    writeln!(out, "\nComputed input nodes: {}", inputs.len())?;
    for y in inputs {
        writeln!(out, "- {y}")?;
    }
    Ok(())
}
pub fn write_auto_inputs_json(
    mut out: impl Write,
    inputs: &[NodeId],
) -> Result<(), std::io::Error> {
    let serialized = serde_json::to_string(&inputs)?;
    writeln!(out, ",\"computed input nodes\":{serialized}")?;
    Ok(())
}

pub fn write_mics(
    mut out: impl Write,
    mics: impl Iterator<Item = Vec<NodeId>>,
) -> Result<(), std::io::Error> {
    writeln!(out, "\n## Minimal inconsistent cores")?;
    let mut oldmic = vec![];
    for (count, mic) in mics.enumerate() {
        if oldmic != *mic {
            write!(out, "\n{}. Mic:\n", count + 1)?;
            for node in mic.clone() {
                writeln!(out, "  - {node}")?;
            }
            oldmic = mic;
        }
    }
    Ok(())
}
pub fn write_json_mics(
    mut out: impl Write,
    mics: impl Iterator<Item = Vec<NodeId>>,
) -> Result<(), std::io::Error> {
    writeln!(out, ",\"mics\":[")?;

    let mut oldmic = vec![];
    for (c, mic) in mics.enumerate() {
        if c == 0 {
            writeln!(out, "  {}", serde_json::to_string(&mic)?)?;
            oldmic = mic;
        } else if oldmic != mic {
            writeln!(out, " ,{}", serde_json::to_string(&mic)?)?;
            oldmic = mic;
        }
    }
    writeln!(out, "]")?;
    Ok(())
}

pub fn write_labelings(
    mut out: impl Write,
    labelings: impl Iterator<Item = (Vec<Prediction>, Vec<RepairOp>)>,
) -> Result<(), std::io::Error> {
    writeln!(out, "\n## Possible labelings under repair")?;
    for (c, (labels, repairs)) in labelings.enumerate() {
        writeln!(out, "\n{}. Labeling:", c + 1)?;
        write_labels(&mut out, &labels)?;

        writeln!(out, "\n  Repair set:")?;
        for fix in repairs {
            writeln!(out, "  - {fix}")?;
        }
    }
    Ok(())
}
pub fn write_json_labelings(
    mut out: impl Write,
    labelings: impl Iterator<Item = (Vec<Prediction>, Vec<RepairOp>)>,
) -> Result<(), std::io::Error> {
    writeln!(out, ",\"labels under repair\":[")?;

    for (idx, (labels, repairs)) in labelings.enumerate() {
        if idx == 0 {
            write!(out, "  ")?;
        } else {
            write!(out, " ,")?;
        }
        let serialized = serde_json::to_string(&labels)?;
        writeln!(out, "{{\"labels\":{serialized},")?;
        let serialized = serde_json::to_string(&repairs)?;
        writeln!(out, "   \"repairs\":{serialized}\n  }}")?;
    }
    writeln!(out, "]")?;
    Ok(())
}

pub fn write_labels(mut out: impl Write, labels: &[Prediction]) -> Result<(), std::io::Error> {
    for assign in labels {
        writeln!(out, "  - {} = {}", assign.node, assign.behavior)?;
    }
    Ok(())
}

pub fn write_json_predictions(
    mut out: impl Write,
    predictions: &[Prediction],
) -> Result<(), std::io::Error> {
    let serialized = serde_json::to_string(&predictions)?;
    writeln!(&mut out, ",\"predictions\":{serialized}")?;
    Ok(())
}
pub fn write_predictions(
    mut out: impl Write,
    predictions: &[Prediction],
) -> Result<(), std::io::Error> {
    let mut plus = 0;
    let mut minus = 0;
    let mut zero = 0;
    let mut not_plus = 0;
    let mut not_minus = 0;
    let mut change = 0;
    writeln!(out, "\n## Predictions\n")?;
    for pred in predictions {
        writeln!(out, "- {}", pred)?;
        match pred.behavior {
            Behavior::Plus => plus += 1,
            Behavior::Minus => minus += 1,
            Behavior::Zero => zero += 1,
            Behavior::NotPlus => not_plus += 1,
            Behavior::NotMinus => not_minus += 1,
            Behavior::Change => change += 1,
        }
    }
    writeln!(out, "\n## Prediction statistics\n")?;
    writeln!(out, "- predicted +        : {plus}")?;
    writeln!(out, "- predicted -        : {minus}")?;
    writeln!(out, "- predicted 0        : {zero}")?;
    writeln!(out, "- predicted notPlus  : {not_plus}")?;
    writeln!(out, "- predicted notMinus : {not_minus}")?;
    writeln!(out, "- predicted CHANGE   : {change}")?;
    Ok(())
}
