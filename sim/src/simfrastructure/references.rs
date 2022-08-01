use std::{fmt::Debug, collections::HashMap, rc::{Rc, Weak}, cell::{RefCell, Ref}, borrow::BorrowMut, ops::Deref};

use pyo3::{FromPyObject, PyResult, PyAny};

use super::{models::SimModelTrait, ModelID, WeakModelPtr, ModelPtr, ModelIdMap};

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

impl<T: ?Sized + SimModelTrait> ReferenceList<T> {
    pub fn populate_refs( &mut self, global_model_list: &HashMap<ModelID, Rc<RefCell<T>>> ) -> Result<(), ()> {
        for id in &self.reference_ids {
            // let temp_rc = global_model_list.get( id ).unwrap().clone();
            // let object = temp_rc.borrow().as_any().downcast_mut::<T>().unwrap();
            // let mut contents = temp_rc.borrow_mut();
            // if let Some( concrete ) = contents.as_any().downcast_mut::<T>() {
            //     self.reference_list.push(
            //         Rc::downgrade( concrete )
            //     )
            // }
            self.reference_list.push(
                Rc::downgrade( global_model_list.get( id ).unwrap() )
            )
            // let conc_rc = Rc::downcast::<RefCell<T>>( temp_rc ).unwrap();
            // let concrete: T = temp_rc.borrow().as_any().downcast_ref().unwrap();
            // self.reference_list.push(
            //     Rc::downgrade( conc_rc)
            //     // Rc::downcast( global_model_list.get( id ).unwrap() )
            // )
        }
        
        Ok(())
    }
}

pub struct ReferenceListItem {
    pub id: ModelID,
    obj: Option<WeakModelPtr<dyn SimModelTrait>>
}

impl ReferenceListItem {
    fn get_mut<T: SimModelTrait + 'static>( self ) -> Option<&'static mut T> {
        let option = self.obj.unwrap().upgrade().unwrap();
        let mut ting = option.deref().borrow_mut();
        let model = ting.as_any().downcast_mut::<T>();
        model
        // let mut upgraded_pointer = option.unwrap();
        // let model = upgraded_pointer.borrow_mut();
        // let model = upgraded_pointer.borrow().as_any();
        // let output = model.downcast_mut::<T>().unwrap();
        // output
    }

    fn pop_obj( &mut self, obj: WeakModelPtr<dyn SimModelTrait> ) {
        self.obj = Some( obj );
    }

    fn new( id: ModelID ) -> Self {
        ReferenceListItem {
            id: id, 
            obj: None, 
        }
    }
}

pub struct ReferenceList2 {
    list: Vec<ReferenceListItem>
}

impl FromPyObject<'_> for ReferenceList2 {
    fn extract(ob: &PyAny) -> PyResult<Self> {
        let refs: Vec<ModelID> = ob.getattr( "reference_ids" )?.extract()?;
        let mut new_list = Self { list: vec![] };

        for id in refs {
            new_list.list.push(
                ReferenceListItem::new( id )
            )
        }

        Ok( new_list )
    }
}

impl ReferenceList2 {
    pub fn populate_refs( &mut self, global_model_list: &ModelIdMap) {
        for obj in &mut self.list {
            let weak = Rc::downgrade(global_model_list.get( &obj.id ).unwrap());
            obj.pop_obj( weak );
        }
    }
}

// #[derive(Debug)]
// pub struct ReferenceList2<T: ?Sized + SimModelTrait> {
//     pub reference_ids: Vec<ModelID>,
//     pub reference_list: Vec<&ref>
//     pub reference_list: Vec<WeakModelPtr<T>>,
// }

// impl<T: ?Sized + SimModelTrait> FromPyObject<'_> for ReferenceList2<T> {
//     fn extract(ob: &PyAny) -> PyResult< Self > {
//         Ok( 
//             Self {
//                 reference_ids: ob.getattr( "reference_ids" )?.extract()?,
//                 reference_list: vec![],
//             } 
//         )
//     }
// }

// impl<T: ?Sized + SimModelTrait> ReferenceList2<T> {
//     pub fn populate_refs( &mut self, global_model_list: &HashMap<ModelID, Rc<RefCell<T>>> ) -> Result<(), ()> {
//         for id in &self.reference_ids {
//             let x = global_model_list.get( id ).unwrap();
//             let ptr = x.borrow();
//             Ref::map(ptr, |t| {
//                 Some( t.as_any().downcast_mut::<T>() )
//             });
//             // let temp_rc = global_model_list.get( id ).unwrap().clone();
//             // let object = temp_rc.borrow().as_any().downcast_mut::<T>().unwrap();
//             // let mut contents = temp_rc.borrow_mut();
//             // if let Some( concrete ) = contents.as_any().downcast_mut::<T>() {
//             //     self.reference_list.push(
//             //         Rc::downgrade( concrete )
//             //     )
//             // }
//             self.reference_list.push(
//                 Rc::downgrade( global_model_list.get( id ).unwrap() )
//             )
//             // let conc_rc = Rc::downcast::<RefCell<T>>( temp_rc ).unwrap();
//             // let concrete: T = temp_rc.borrow().as_any().downcast_ref().unwrap();
//             // self.reference_list.push(
//             //     Rc::downgrade( conc_rc)
//             //     // Rc::downcast( global_model_list.get( id ).unwrap() )
//             // )
//         }
        
//         Ok(())
//     }
// }