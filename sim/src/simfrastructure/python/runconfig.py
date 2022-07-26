"""This module specifies the interface for a run configuration to take"""

from abc import ABC
from dataclasses import dataclass
from typing import List

@dataclass
class RunModel:
    name: str
    order: int

@dataclass
class RunConfig( ABC ):
    """The runconfig is what initializes the Rust simulation. 
    It defines the models to use, and how to set them up!"""
    
    model_list: List[ RunModel ]
    description: str