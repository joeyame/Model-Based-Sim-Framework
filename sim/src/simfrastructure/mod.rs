/// # Simfrastructure
/// Simfrastructure is the infrastructure of the simulation

pub mod models;
pub mod references;

pub use references::*;

// The simulation infrastructure that powers everything
pub mod runtime;
pub use runtime::*;

// Short hand datatypes for sim infrastructure
pub mod simtypes;
pub use simtypes::*;