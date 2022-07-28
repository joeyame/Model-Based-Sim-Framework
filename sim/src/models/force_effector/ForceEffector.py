"""This module contains the python bindings to help you generate
a ForceEffector object in the simulation configuration."""

from dataclasses import dataclass
from simfrastructure import Model, ModelBase

@dataclass
class ForceEffector( Model ):
    """Generates an input class for the ForceEffector model"""

    fx: int
    fy: int
    fz: int

    model_details: ModelBase