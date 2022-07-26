use std::fmt::Debug;

use crate::simfrastructure::{ModelPtr, PyAny, PyErr};

#[derive(std::fmt::Debug)]
pub struct ModelDetails {
    pub order: i8,
    pub name: String,
}

pub trait SimModelTrait: Debug {
    fn update( &mut self ) -> bool;
    fn finalize( &mut self ) -> bool;

    fn get_model( &mut self ) -> &ModelDetails;
}

pub trait ModelFromInput {
    fn new( input: &PyAny ) -> Result<ModelPtr, PyErr>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}