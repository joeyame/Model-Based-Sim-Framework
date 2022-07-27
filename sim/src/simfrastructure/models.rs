use std::fmt::Debug;
use pyo3::prelude::FromPyObject;

use crate::simfrastructure::{ModelPtr, PyAny, PyErr};

#[derive(std::fmt::Debug)]
#[derive(FromPyObject)]
pub struct ModelDetails {
    pub order: i8,
}

pub trait SimModelTrait: Debug {
    fn update( &mut self ) -> bool;
    fn finalize( &mut self ) -> bool;

    fn get_model( &mut self ) -> &ModelDetails;
}

pub trait ModelFromInput {
    fn new( input: &PyAny ) -> Result<ModelPtr, PyErr>;
}

// Empty reference model
#[derive(Debug)]
pub struct Reference {
    pub details: ModelDetails,
}

impl SimModelTrait for Reference {
    fn finalize( &mut self ) -> bool {
        false
    }

    fn update( &mut self ) -> bool {
        false
    }

    fn get_model( &mut self ) -> &ModelDetails {
        &self.details
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}