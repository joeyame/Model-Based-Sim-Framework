use std::{env, collections::HashMap};

use sim::simfrastructure::{Runtime, ModelCreatorMap};
use sim::models::register_models;

fn main() {
    let args: Vec<_> = env::args().collect();
    println!( "Starting simulation..." );
    for arg in args.iter() {
        println!( "Argument: {}", arg );
    }

    // First, build registry of available model types
    let mut model_registry: ModelCreatorMap = HashMap::new();
    register_models( &mut model_registry );

    // Now generate the runtime with the given list of models
    let runtime = Runtime::from_config( &args[ 1 ], &model_registry );

    for model in runtime.model_list.iter() {
        println!("Model {:?}", model);
    }
}