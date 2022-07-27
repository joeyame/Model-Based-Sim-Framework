# Import simulation infrastructure components
from simfrastructure import RunConfig

# Import input models
from models.force_effector import ForceEffector
from simfrastructure.python.runconfig import ModelDetails
# from input import eom, force_effector

config = RunConfig( 
    model_list=[
        # eom.model,
        # force_effector.model,
        ForceEffector( 0, 10, 20, ModelDetails( 100, 100 ) )
    ],
    description="A basic system to simulate!"
)