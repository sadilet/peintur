#![recursion_limit = "256"]

pub mod traces;
pub mod plot;
pub mod configuration;
pub mod layout;
pub mod common;

pub use common::color;
pub use configuration::Configuration;
pub trait Restyle: serde::Serialize {}
pub trait Relayout {}
pub use layout::Layout;
pub use plot::{ImageFormat, Plot, Trace};

pub use traces::Scatter;

#[doc(hidden)]
mod private;

