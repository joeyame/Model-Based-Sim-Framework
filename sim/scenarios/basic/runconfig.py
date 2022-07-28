# Import simulation infrastructure components
from simfrastructure import RunConfig, sim_args

# print("Sim args:", sim_args)

# Import input models
from models.force_effector import ForceEffector
from models.eom import EOM
from simfrastructure.python.runconfig import ModelBase, ReferenceList
# from input import eom, force_effector

# print( ForceEffector( 0, 10, 20, ModelDetails( 100, 100 ) ).model_details.id )
# print( ForceEffector( 10, 10, 10, ModelDetails( 0, 0 ) ).model_details.id )

force = ForceEffector( 0, 10, 20, ModelBase( 100, 100 ) )
eom = EOM(
    x=0,
    y=0,
    z=0,
    force_effectors=ReferenceList( force ),
    base=ModelBase( 100, 100, ReferenceList( force ) )
)

# print(force)
print(eom.base)
# print(eom.force_effectors.reference_ids)

config = RunConfig( 
    model_list=[
        force,
        eom
    ],
    description="A basic system to simulate!"
)

# print( config.model_list[0].model_details.id )