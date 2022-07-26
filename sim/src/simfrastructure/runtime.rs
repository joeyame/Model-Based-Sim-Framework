// External imports
use pyo3::{prelude::*, types::PyList};

// Local crate imports
use crate::simfrastructure::{ModelCreatorMap, ModelPtr};

pub struct Runtime {
    pub run: i8,
    pub model_list: Vec<ModelPtr>
}

impl Runtime {
    pub fn from_config( path: &String, model_registry: &ModelCreatorMap ) -> Self {
        let config_path = path.clone()+"/runconfig.py";
        println!( "Generating runtime from {}!", config_path );

        // Create runtime
        let mut runtime = Runtime {
            run: 0,
            model_list: vec![],
        };

        // Start python interpreter
        Python::with_gil(|py| -> PyResult<()> {

            // Add current scenario folder to path
            let syspath: &PyList = py.import( "sys" )?.getattr( "path" )?.extract()?;
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

                // Generate model
                runtime.model_list.push( model_registry.get( &model_type ).unwrap()( py_model )? );
            }

            Ok(())
        }).unwrap();

        runtime
    }
}