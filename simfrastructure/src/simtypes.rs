// External imports
use std::collections::HashMap;
use std::rc::Rc;

// // Local crate imports
use crate::models::SimModelTrait;
use crate::{ PyAny, PyErr };

// Type shorthand to make code cleaner
pub type ModelPtr = Rc<dyn SimModelTrait>;
pub type ModelCreatorFn = fn( &PyAny )->Result<ModelPtr, PyErr>;
pub type ModelCreatorMap = HashMap<String, ModelCreatorFn>;