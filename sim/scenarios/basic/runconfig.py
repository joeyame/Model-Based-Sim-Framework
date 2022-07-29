# Import simulation infrastructure components
from simfrastructure import RunConfig, sim_args

# Import input models
from models.force_effector import ForceEffector
from models.eom import EOM
from simfrastructure.python.runconfig import *

# Create models here
force = ForceEffector( 0, 10, 20, ModelBase( 100, 200 ) )
eom = EOM(
    x=0,
    y=0,
    z=0,
    force_effectors=ReferenceList( force ),
    base=ModelBase( 100, 200 )
)

# Create the actual configuration
config = RunConfig( 
    # Description
    "A basic system to simulate!",

    # Define models:
    ModelGroup( eom, force )
)