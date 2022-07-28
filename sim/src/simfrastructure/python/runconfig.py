"""This module specifies the interface for a run configuration to take"""

from abc import ABC
from dataclasses import dataclass
from typing import List
import sys

sim_args = sys.argv[1:]

class IndexService:
    """
    The IndexService is how we assign a unique ID to every entity
    in the simulation. This service makes it all automatic in the
    background.
    """

    current_id = 0

    @classmethod
    def take_id( cls ) -> int:
        id = cls.current_id
        cls.current_id += 1
        return id

@dataclass
class ReferenceList:
    """
    An object that helps connect references in the simulation
    """

    references: List[ "Model" ] = None

    def __post_init__( self ):
        if self.references == None:
            self.references = []
        self.reference_ids = []

        if type( self.references ) == list:
            for model in self.references:
                self.reference_ids.append( model.base.id )
        else:
            self.reference_ids.append( self.references.model_details.id )

@dataclass
class ModelBase:
    """
    Contains metadata that the simulation uses to process the model.
    Every model must contain this.
    """

    order: int
    rate: int
    local_refs: "ReferenceList" = ReferenceList()

    def __post_init__( self ):
        self.id = IndexService.take_id()

class Model( ABC ):
    "The 'Adam' of all models"

    base = ModelBase( 0, 0 )

# class Reference( Model ):
#     "A dumb reference that will later be replaced"

#     def __init__( self, model: Model ):
#         "Creates a reference to an already-existing model."

#         self.target_id = model.model_details.id

@dataclass
class RunConfig( ABC ):
    """The runconfig is what initializes the Rust simulation. 
    It defines the models to use, and how to set them up!"""
    
    model_list: List[ Model ]
    description: str

# def ModelGroup( *args ) -> 