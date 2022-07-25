from simfrastructure.python.utilities.unit_conversion import SI, PrefixSI

from models import EOM

model = EOM( 
    name = "EOM_0", 
    order = 100, 
    x = 0, 
    y = 0, 
    z = int( SI( 100 ).convert_to( PrefixSI.femto ).value ),
)