use std::cell::RefCell;
// External imports
use std::{collections::HashMap, rc::Weak};
use std::rc::Rc;

// // Local crate imports
use crate::simfrastructure::models::SimModelTrait;
use pyo3::{ PyAny, PyErr };

// Type shorthand to make code cleaner and more maintainable

/// An ID for a model, simply a 16 bit int, but if this ever
/// needs to change it can be done in one location!
pub type ModelID = i16;

/// A reference-counted pointer for anything implementing SimModelTrait
pub type ModelPtr = Rc<RefCell<dyn SimModelTrait>>;

/// A weak reference to something implementing SimModelTrait. Models should
/// refer to other models with a weak model pointer!
pub type WeakModelPtr<T> = Weak<RefCell<T>>;

/// A function that is used to create a Rust model, given an equivalent Python
/// input model.
pub type ModelCreatorFn = fn( &PyAny )->Result<ModelPtr, PyErr>;

/// A map of model creation functions that sorts them by the type name
/// string. This can easily be called on a Python object to get its type
/// and then delegate it to the proper Rust model.
pub type ModelCreatorMap = HashMap<String, ModelCreatorFn>;