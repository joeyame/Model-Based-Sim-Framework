# Import simulation infrastructure components
import simfrastructure

# Import input models
from input import eom, force_effector

config = simfrastructure.RunConfig( 
    model_list=[
        eom.model,
        force_effector.model,
    ],
    description="A basic system to simulate!"
)