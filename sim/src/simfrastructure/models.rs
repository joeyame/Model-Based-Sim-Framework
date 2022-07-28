use std::{fmt::Debug, collections::HashMap, rc::Rc};
use pyo3::prelude::*;

use super::*;

#[derive(std::fmt::Debug)]
#[derive(FromPyObject)]
pub struct ModelBase {
    pub order: i8,
    pub id: ModelID,
    pub local_refs: ReferenceList<dyn SimModelTrait>,
}

pub trait SimModelTrait: Debug {
    fn resolve_references( &mut self, global_model_list: &HashMap<ModelID, ModelPtr> ) {
        let locals = &mut self.get_details().local_refs;
        for id in &locals.reference_ids {
            locals.reference_list.push( 
                Rc::downgrade( global_model_list.get( id ).unwrap() )
            )
        }
    }

    // Runtime methods
    fn initialize( &mut self ) -> bool;
    fn update( &mut self ) -> bool;
    fn finalize( &mut self ) -> bool;

    // Necessary return functions
    fn get_details( &mut self ) -> &mut ModelBase;
}

pub trait ModelFromInput {
    fn new( input: &PyAny ) -> Result<ModelPtr, PyErr>;
}

#[derive(Debug)]
pub struct ReferenceList<T: ?Sized + SimModelTrait> {
    pub reference_ids: Vec<ModelID>,
    pub reference_list: Vec<WeakModelPtr<T>>,
}

impl<T: ?Sized + SimModelTrait> FromPyObject<'_> for ReferenceList<T> {
    fn extract(ob: &PyAny) -> PyResult< Self > {
        Ok( 
            Self {
                reference_ids: ob.getattr( "reference_ids" )?.extract()?,
                reference_list: vec![],
            } 
        )
    }
}

// #[derive(Debug, FromPyObject)]
// pub enum ModelRef {
//     Preref( i16 ),
//     // Reference( ModelPtr ),
// }

// impl ModelFromInput for Reference {
//     fn new( input: &PyAny ) -> Result<ModelPtr, PyErr> {
//         Ok( Rc::<Reference>::new( input.extract()? ) )
//     }
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}