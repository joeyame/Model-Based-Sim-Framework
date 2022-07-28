"""This module contains the python bindings to help you generate
an EOM object in the simulation configuration."""

from dataclasses import dataclass
from simfrastructure.python.runconfig import Model, ModelBase, ReferenceList

@dataclass
class EOM( Model ):
    """Generates an input class for the EOM model"""

    x: int
    y: int
    z: int

    force_effectors: ReferenceList

    base: ModelBase