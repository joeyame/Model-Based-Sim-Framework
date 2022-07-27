"""This module specifies the interface for a run configuration to take"""

from abc import ABC, abstractmethod
from dataclasses import dataclass
from typing import List

@dataclass
class ModelDetails:
    """
    Contains metadata that the simulation uses to process the model.
    Every model must contain this.
    """

    order: int
    rate: int

class Model( ABC ):
    "The 'Adam' of all models"

    # @abstractmethod
    # def getModelDetails( ) -> ModelDetails:
    #     "Returns a set of model-details to the sim"

@dataclass
class RunConfig( ABC ):
    """The runconfig is what initializes the Rust simulation. 
    It defines the models to use, and how to set them up!"""
    
    model_list: List[ Model ]
    description: str