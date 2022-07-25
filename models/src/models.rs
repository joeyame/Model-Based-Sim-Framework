use simfrastructure::{ModelCreatorMap, models::ModelFromInput};

pub fn register_models( map: &mut ModelCreatorMap ) {
    map.insert( "EOM".to_owned(), eom::EOM::new );
    map.insert("ForceEffector".to_owned(), force_effector::ForceEffector::new);
}