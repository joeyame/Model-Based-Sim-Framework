use std::{fmt::Debug, collections::HashMap, rc::Rc, any::Any};
use pyo3::prelude::*;

use super::*;
use super::references::ReferenceList;

#[derive(Debug, FromPyObject)]
pub struct ModelBase {
    pub order: i8,
    pub id: ModelID,
    pub local_refs: ReferenceList<dyn SimModelTrait>,
    // pub global_model_list: HashMap<ModelID, ModelPtr>
}

// impl FromPyObject<'_> for ModelBase {
//     fn extract(ob: &PyAny) -> PyResult<Self> {
//         Ok(
//             Self {
//                 order: ob.getattr( "order" )?.extract()?,
//                 id: ob.getattr( "id" )?.extract()?,
//                 local_refs: ob.getattr( "local_refs" )?.extract()?,
//                 // runtime: Runtime { run: -1, model_list: HashMap::new() }
//             }
//         )
//     }
// }

pub trait ModelFromInput {
    fn new( input: &PyAny ) -> Result<ModelPtr, PyErr>;
}

pub trait SimModelTrait: Debug {
    
    /// A model should provide a custom implementation of this function if
    /// it needs to fill its reference lists
    fn resolve_references( &mut self, global_model_list: &HashMap<ModelID, ModelPtr> ) {
        let _temp = global_model_list;
    }

    /// ## ***This function should never be modified!!!***
    /// It populates a model's local_refs, which is an operation that 
    /// shouldn't ever need to be changed. If you need to connect references 
    /// manually, consider using ``` resolve_references ``` instead!
    fn resolve_global_refs( &mut self, global_model_list: &HashMap<ModelID, ModelPtr> ) {
        let locals = &mut self.get_details().local_refs;
        for id in &locals.reference_ids {
            locals.reference_list.push( 
                Rc::downgrade( &global_model_list.get( id ).unwrap() )
            )
        }
    }

    // Runtime methods
    fn initialize( &mut self ) -> bool;
    fn update( &mut self ) -> bool;
    fn finalize( &mut self ) -> bool;
    fn as_any(&mut self) -> &mut dyn Any;

    // Necessary return functions
    fn get_details( &mut self ) -> &mut ModelBase;
}

pub trait Model: ModelFromInput + SimModelTrait {}

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