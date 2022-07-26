use crate::simfrastructure::{ModelCreatorMap, models::ModelFromInput};

pub mod eom;
pub mod force_effector;

pub fn register_models( map: &mut ModelCreatorMap ) {
    map.insert( "EOM".to_owned(), eom::EOM::new );
    map.insert("ForceEffector".to_owned(), force_effector::ForceEffector::new);
}