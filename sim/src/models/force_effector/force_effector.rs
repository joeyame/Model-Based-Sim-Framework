use std::rc::Rc;

use crate::simfrastructure::models::{ModelDetails, SimModelTrait, ModelFromInput};
use crate::simfrastructure::{PyAny, PyErr};
use crate::simfrastructure::{ModelPtr};

#[derive(std::fmt::Debug)]
pub struct ForceEffector {
    pub fx: i128,
    pub fy: i128,
    pub fz: i128,

    pub model_details: ModelDetails,
}

pub fn new( input: &PyAny ) -> Result<ModelPtr, PyErr> {
    Ok( 
        Rc::new(
            ForceEffector { 
                // EOM-specific properties
                fx: input.getattr( "fx" )?.extract()?, 
                fy: input.getattr( "fy" )?.extract()?, 
                fz: input.getattr( "fz" )?.extract()?,

                // General Model Properties
                model_details: ModelDetails {
                    name: input.getattr( "name" )?.extract()?,
                    order: input.getattr( "order" )?.extract()?,
                }
            }
        )
    )
}

impl SimModelTrait for ForceEffector {
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

impl ModelFromInput for ForceEffector {
    fn new( input: &PyAny ) -> Result<ModelPtr, PyErr> {
        Ok( 
            Rc::new(
                ForceEffector { 
                    // EOM-specific properties
                    fx: input.getattr( "fx" )?.extract()?, 
                    fy: input.getattr( "fy" )?.extract()?, 
                    fz: input.getattr( "fz" )?.extract()?,
    
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


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
