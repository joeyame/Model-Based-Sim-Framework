// Global imports
use std::boxed::Box;

// Imports from the simulation infrastructure
use simfrastructure::models::SimModelTrait;

fn main() {
    // Create the list of models
    let mut model_list: Vec<Box<dyn SimModelTrait>> = Vec::new();

    // Add a model to the list
    model_list.push( Box::new( eom::new() ) );

    for model in model_list.iter_mut() {
        model.initialize();
    }

    for _ in 1..10 as i8 {
        for model in model_list.iter_mut() {
            model.update();
        }
    }

    for model in model_list.iter_mut() {
        model.finalize();
    }

    println!( "Simulation complete!" );
}