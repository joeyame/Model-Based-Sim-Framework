use std::collections::HashMap;

// External imports
use pyo3::{prelude::*, types::PyList};

// Local crate imports
use crate::simfrastructure::{ModelCreatorMap, ModelPtr, ModelID};

pub struct Runtime {
    pub run: i8,
    pub model_list: HashMap<ModelID, ModelPtr>
}

impl Runtime {
    pub fn from_config( path: &String, model_registry: &ModelCreatorMap, args: &Vec<String> ) -> Self {
        let config_path = path.clone()+"/runconfig.py";
        println!( "Generating runtime from {}!", config_path );

        // Create runtime
        let mut runtime = Runtime {
            run: 0,
            model_list: HashMap::new(),
        };

        // Start python interpreter
        Python::with_gil(|py| -> PyResult<()> {

            // Add current scenario folder to path
            let syspath: &PyList = py.import( "sys" )?.getattr( "path" )?.extract()?;
            let sysargs: &PyList = py.import( "sys" )?.getattr( "argv" )?.extract()?;
            
            
            args[2..args.len()].iter().for_each(|arg| {
                sysargs.append( arg ).unwrap();
            });

            syspath.insert(0, path)?;
            
            // Run the configuration module
            let code = std::fs::read_to_string( config_path )?;
            let config_module = PyModule::from_code(py, &code, "runconfig.py", "runconfig")?;

            // Get the configuration object, and its corresponding model list
            let config = config_module.getattr( "config" )?;
            let description: String = config.getattr( "description" )?.extract()?;
            let model_list: Vec<&PyAny> = config.getattr( "model_list" )?.extract()?;

            println!("Simulation Description: {}", description);

            for py_model in model_list {

                // Get model identifier
                let model_type: String = py_model.getattr("__class__")?.getattr("__name__")?.extract()?;

                println!( "Attempting to generate {}", &model_type );

                // Generate model and add it to the runtime list by ID
                let new_model: ModelPtr = model_registry.get( &model_type ).unwrap()( py_model )?;
                let id = new_model.borrow_mut().get_details().id;
                runtime.model_list.insert( 
                    id, 
                    new_model
                );
            }

            Ok(())
        }).unwrap();

        // Connect references
        for ( _id, model ) in &runtime.model_list {
            model.borrow_mut().resolve_references( &runtime.model_list )
        }

        // Initialize models
        for ( _id, model ) in &runtime.model_list {
            model.borrow_mut().initialize();
        }

        return runtime
    }
}