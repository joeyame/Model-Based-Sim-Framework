"""This module contains the python bindings to help you generate
a ForceEffector object in the simulation configuration."""

from dataclasses import dataclass
from simfrastructure.python.runconfig import RunModel

@dataclass
class ForceEffector( RunModel ):
    """Generates an input class for the ForceEffector model"""

    fx: int
    fy: int
    fz: int