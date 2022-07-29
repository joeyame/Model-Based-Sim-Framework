"""This module specifies the interface for a run configuration to take"""

from abc import ABC
from dataclasses import dataclass
from typing import List
import sys
import unittest

sim_args = sys.argv[1:]

class IndexService:
    """
    The IndexService is how we assign a unique ID to every entity
    in the simulation. This service makes it all automatic in the
    background.
    """

    current_id = 1

    @classmethod
    def take_id( cls ) -> int:
        id = cls.current_id
        cls.current_id = cls.current_id + 1
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

    base: ModelBase

class RunConfig( ABC ):
    """The runconfig is what initializes the Rust simulation. 
    It defines the models to use, and how to set them up!"""
    
    def __init__( self, description: str, *args: Model ) -> None:
        """
        Creates a run configuration given a description and a
        list of models.
        """

        self.description = description

        self.model_list = []
        for item in args:
            if type( item ) == list:
                self.model_list.extend( item )
            elif isinstance( item, Model ):
                self.model_list.append( item )

def ModelGroup( *args: Model ) -> List[ Model ]:
    """
    Models grouped together automatically hold references
    to each other inside of their base.local_refs property.

    This allows for pretty advanced systems without the need
    to manually define many references.
    """

    # Add the ID's to a model's references, skipping over itself.
    for model in args:
        small_list = [ neighbor for neighbor in args if neighbor != model ]
        model.base.local_refs = ReferenceList( small_list )
    
    return list( args )

from termcolor import colored
class tests(unittest.TestCase):
    def test_indexing( self ):
        "Tests that model IDs are handled correctly"

        print( colored( "RunConfig:test_indexing: ", "white" ), end="" )
        try:
            index0 = IndexService.take_id()
            index1 = IndexService.take_id()
            index2 = IndexService.take_id()
            index3 = IndexService.take_id()
            index4 = IndexService.take_id()
            self.assertEqual( index0, 2 )
            self.assertEqual( index1, 3 )
            self.assertEqual( index2, 4 )
            self.assertEqual( index3, 5 )
            self.assertEqual( index4, 6 )
            print( colored( "OK", "green", attrs=["bold"] ) )
        except Exception as e:
            print( colored( "Failed!", "red", attrs=['bold'] ) )
            raise e

    def test_model_id( self ):
        "Test that model IDs are handled correctly"

        print( colored( "RunConfig:test_model_id: ", "white" ), end="" )
        try:
            base0 = ModelBase( 0, 10 )
            self.assertEqual( ModelBase( 0, 10 ).id, 8 )
            base1 = ModelBase( 0, 10 )
            self.assertEqual( ModelBase( 0, 10 ).id, 10 )
            base2 = ModelBase( 0, 10 )
            base3 = ModelBase( 0, 10 )
            base4 = ModelBase( 0, 10 )
            self.assertEqual( base0.id, 7 )
            self.assertEqual( base1.id, 9 )
            self.assertEqual( base2.id, 11 )
            self.assertEqual( base3.id, 12 )
            self.assertEqual( base4.id, 13 )
            print( colored( "OK", "green", attrs=["bold"] ) )
        except Exception as e:
            print( colored( "Failed!", "red", attrs=['bold'] ) )
            raise e