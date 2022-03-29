use iggy::cif_parser;
use pyo3::exceptions::{PyException, PyIOError};
use pyo3::prelude::*;

#[pyclass]
struct PGraph(cif_parser::Graph);

/// reads a file in CIF and returns the corresponding Graph.
#[pyfunction]
fn read_cif(file_name: &str) -> PyResult<PGraph> {
    use std::fs::File;
    match File::open(file_name) {
        Ok(file) => match cif_parser::read(&file) {
            Ok(graph) => Ok(PGraph(graph)),
            Err(e) => Err(PyException::new_err(format!("{e}"))),
        },
        Err(e) => Err(PyIOError::new_err(e)),
    }
}

#[pyfunction]
fn print_graph(graph: &PGraph) -> PyResult<bool> {
    println!("{:?}", graph.0);
    Ok(true)
}

/// A Python module implemented in Rust.
#[pymodule]
fn iggy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(read_cif, m)?)?;
    m.add_function(wrap_pyfunction!(print_graph, m)?)?;
    Ok(())
}
