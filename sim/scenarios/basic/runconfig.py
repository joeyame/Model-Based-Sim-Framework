# Import simulation infrastructure components
from simfrastructure import RunConfig

# Import input models
from input import eom, force_effector

config = RunConfig( 
    model_list=[
        eom.model,
        force_effector.model,
    ],
    description="A basic system to simulate!"
)