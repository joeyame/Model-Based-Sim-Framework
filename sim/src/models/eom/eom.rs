use std::rc::Rc;

use crate::simfrastructure::models::{ModelDetails, SimModelTrait, ModelFromInput};
use crate::simfrastructure::{PyAny, PyErr};
use crate::simfrastructure::{ModelPtr};

// pub fn new( input: &PyAny ) -> Result<ModelPtr, PyErr> {
//     Ok( 
//         Rc::new(
//             EOM { 
//                 // EOM-specific properties
//                 x: input.getattr( "x" )?.extract()?, 
//                 y: input.getattr( "y" )?.extract()?, 
//                 z: input.getattr( "z" )?.extract()?, 

//                 force_effectors: vec![],

//                 // General Model Properties
//                 model_details: ModelDetails {
//                     name: input.getattr( "name" )?.extract()?,
//                     order: input.getattr( "order" )?.extract()?,
//                 }
//             }
//         ) 
//     )
// }

#[derive(std::fmt::Debug)]
pub struct EOM {
    pub x: i128,
    pub y: i128,
    pub z: i128,

    pub force_effectors: Vec<ModelPtr>,

    pub model_details: ModelDetails,
}

impl ModelFromInput for EOM {
    fn new( input: &PyAny ) -> Result<ModelPtr, PyErr> {
        Ok( 
            Rc::new(
                EOM { 
                    // EOM-specific properties
                    x: input.getattr( "x" )?.extract()?, 
                    y: input.getattr( "y" )?.extract()?, 
                    z: input.getattr( "z" )?.extract()?, 
    
                    force_effectors: vec![],
    
                    // General Model Properties
                    model_details: ModelDetails {
                        name: input.getattr( "name" )?.extract()?,
                        order: input.getattr( "order" )?.extract()?,
                    }
                }
            ) 
        )
    }
}

impl SimModelTrait for EOM {
    fn update( &mut self ) -> bool {
        true
    }

    fn finalize( &mut self ) -> bool {
        true
    }

    fn get_model( &mut self ) -> &ModelDetails {
        &self.model_details
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
