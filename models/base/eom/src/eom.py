"""This module contains the python bindings to help you generate
an EOM object in the simulation configuration."""

from dataclasses import dataclass
from simfrastructure.python.runconfig import RunModel

@dataclass
class EOM( RunModel ):
    """Generates an input class for the EOM model"""
    x: int
    y: int
    z: int