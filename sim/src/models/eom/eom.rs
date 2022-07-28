use std::cell::RefCell;
use std::rc::Rc;
use pyo3::FromPyObject;

use crate::models::force_effector::ForceEffector;
use crate::simfrastructure::models::*;
use crate::simfrastructure::{PyAny, PyErr};
use crate::simfrastructure::{ModelPtr};

#[derive(std::fmt::Debug)]
#[derive(FromPyObject)]
pub struct EOM {
    pub x: i128,
    pub y: i128,
    pub z: i128,

    pub force_effectors: ReferenceList<dyn SimModelTrait>,
    // pub test: ReferenceList<ForceEffector>,

    pub base: ModelBase,
}

impl ModelFromInput for EOM {
    fn new( input: &PyAny ) -> Result<ModelPtr, PyErr> {
        Ok( Rc::<RefCell<EOM>>::new( RefCell::new( input.extract()? ) ) )
    }
}

impl SimModelTrait for EOM {
    fn initialize( &mut self ) -> bool {
        println!( "{:?}", self.force_effectors );
        true
    }

    fn update( &mut self ) -> bool {
        true
    }

    fn finalize( &mut self ) -> bool {
        true
    }

    fn get_details( &mut self ) -> &mut ModelBase {
        &mut self.base
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
