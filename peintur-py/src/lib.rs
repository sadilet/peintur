mod wrapper;

use pyo3::prelude::*;
use polars::prelude::*;
use pyo3_polars::{PySeries, PyDataFrame};
use wrapper::TraceWrapper;

#[pyfunction]
fn scatter(pydf: PyDataFrame, col_a: &str, col_b: &str) -> PyResult<TraceWrapper> {
    let df: DataFrame  = pydf.into();

    let x = df.column(col_a).unwrap();
    let y = df.column(col_b).unwrap();

    let as_vec_x: Vec<i32> = x.i32().unwrap().into_no_null_iter().collect();
    let as_vec_y: Vec<i32> = y.i32().unwrap().into_no_null_iter().collect();

    Ok(TraceWrapper::new(as_vec_x, as_vec_y))
}

#[pymodule]
fn peintur(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(scatter, m)?)?;
    Ok(())
}