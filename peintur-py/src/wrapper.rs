use pyo3::prelude::*;
use peintur::{Plot, Scatter};


#[pyclass]
pub struct TraceWrapper {
    trace: Box<Scatter<i32, i32>>,
}

#[pymethods]
impl TraceWrapper {
    pub fn show(&self){
        let mut plot = Plot::new();
        plot.add_trace(self.trace.clone());
        plot.notebook_display()
    }
}

impl TraceWrapper {
    pub fn new(x: Vec<i32>, y: Vec<i32>) -> TraceWrapper {
        TraceWrapper{trace: Scatter::new(x, y)}
    }
}