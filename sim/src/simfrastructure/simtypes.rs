// External imports
use std::collections::HashMap;
use std::rc::Rc;

// // Local crate imports
use crate::simfrastructure::models::SimModelTrait;
use pyo3::{ PyAny, PyErr };

// Type shorthand to make code cleaner
pub type ModelPtr = Rc<dyn SimModelTrait>;
pub type ModelCreatorFn = fn( &PyAny )->Result<ModelPtr, PyErr>;
pub type ModelCreatorMap = HashMap<String, ModelCreatorFn>;