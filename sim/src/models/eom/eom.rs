use std::any::Any;
use std::cell::{RefCell};
use std::ops::Deref;
use std::rc::Rc;
use pyo3::FromPyObject;

use crate::models::force_effector::ForceEffector;
use crate::simfrastructure::{models::*};
use crate::simfrastructure::{PyAny, PyErr};
use crate::simfrastructure::{ModelPtr};
use crate::simfrastructure::models::SimModelTrait;

#[derive(std::fmt::Debug)]
#[derive(FromPyObject)]
pub struct EOM {
    pub x: i128,
    pub y: i128,
    pub z: i128,

    pub force_effectors: ReferenceList<dyn SimModelTrait>,

    pub base: ModelBase,
}

impl ModelFromInput for EOM {
    fn new( input: &PyAny ) -> Result<ModelPtr, PyErr> {
        Ok( Rc::<RefCell<EOM>>::new( RefCell::new( input.extract()? ) ) )
    }
}

impl SimModelTrait for EOM {
    fn initialize( &mut self ) -> bool {
        println!( "EOM Model is referencing:" );
        for reference in &self.base.local_refs.reference_list {
            let upgraded = reference.upgrade().unwrap();
            let mut contents = upgraded.deref().borrow_mut();
            if let Some( force_model ) = contents.as_any().downcast_mut::<ForceEffector>() {
                println!( " - Force ID: {}", force_model.get_details().id );
                force_model.get_details().id = 5;
                println!( " - Force ID (new): {}", force_model.get_details().id );
            }
        }
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

    fn as_any(&mut self) -> &mut dyn Any {
        self
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
