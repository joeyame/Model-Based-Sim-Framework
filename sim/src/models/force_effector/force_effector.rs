use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use pyo3::{FromPyObject, PyAny, PyErr};

use crate::simfrastructure::models::{ModelBase, SimModelTrait, ModelFromInput};
use crate::simfrastructure::{ModelPtr};

#[derive(std::fmt::Debug)]
#[derive(FromPyObject)]
pub struct ForceEffector {
    pub fx: i128,
    pub fy: i128,
    pub fz: i128,

    pub base: ModelBase,
}

pub fn new( input: &PyAny ) -> Result<ModelPtr, PyErr> {
    // Ok( 
    //     Rc::new(
    //         ForceEffector { 
    //             // EOM-specific properties
    //             fx: input.getattr( "fx" )?.extract()?, 
    //             fy: input.getattr( "fy" )?.extract()?, 
    //             fz: input.getattr( "fz" )?.extract()?,

    //             // General Model Properties
    //             model_details: ModelDetails {
    //                 // name: input.getattr( "name" )?.extract()?,
    //                 order: input.getattr( "order" )?.extract()?,
    //             }
    //         }
    //     )
    // )
    // let model: ForceEffector = input.extract()?;
    // let modelPtr = Rc::new( model );
    
    // let model: ForceEffector = ;

    // Ok( Rc::<ForceEffector>::new( input.extract()? ) )
    Ok( Rc::<RefCell<ForceEffector>>::new( RefCell::new( input.extract()? ) ) )

    // let model: Rc<ForceEffector> = Rc::from(
    //     input.extract()?
    // );
    // Ok( modelPtr )
}

impl SimModelTrait for ForceEffector {
    fn initialize( &mut self ) -> bool {
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

impl ModelFromInput for ForceEffector {
    fn new( input: &PyAny ) -> Result<ModelPtr, PyErr> {
        // Ok( 
        //     Rc::new(
        //         ForceEffector { 
        //             // EOM-specific properties
        //             fx: input.getattr( "fx" )?.extract()?, 
        //             fy: input.getattr( "fy" )?.extract()?, 
        //             fz: input.getattr( "fz" )?.extract()?,
    
        //             // General Model Properties
        //             model_details: ModelDetails {
        //                 // name: input.getattr( "name" )?.extract()?,
        //                 order: input.getattr( "order" )?.extract()?,
        //             }
        //         }
        //     )
        // )
        Ok( Rc::<RefCell<ForceEffector>>::new( RefCell::new( input.extract()? ) ) )
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
