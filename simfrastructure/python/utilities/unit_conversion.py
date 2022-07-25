from dataclasses import dataclass
from enum import Enum

from attr import attr

# class SI:
#     @staticmethod
#     def m2fm( meters: float ) -> float:
#         return meters * 1000000000000000000

class PrefixSI( Enum ):
    """An enum that contains conversion factors for all the different SI unit
    prefixes. Should be used alongside the SI number class."""

    peta  = 1e15
    tera  = 1e12
    giga  = 1e9
    mega  = 1e6
    kilo  = 1e3
    hecto = 1e2
    deca  = 1e1

    none  = 1

    deci  = 1e-1
    centi = 1e-2
    milli = 1e-3
    micro = 1e-6
    nano  = 1e-9
    pico  = 1e-12
    femto = 1e-15


@dataclass
class SI:
    """Allows a number to be tied to an SI prefix, and converted between
    a list of available prefixes!"""

    value: float
    prefix: PrefixSI = PrefixSI.none

    def convert_to( self, new_prefix: PrefixSI ) -> "SI":
        """Converts the value to a new SI unit"""

        self.value = ( self.value * self.prefix.value ) / new_prefix.value
        return self



import unittest
from termcolor import colored

class TestPrefixConversion(unittest.TestCase):

    def test_m_to_km( self ):
        "Tests that low-to-high conversions are calculated correctly"

        print( colored( "TestPrefixConversion:test_m_to_km: ", "white" ), end="" )
        try:
            m = SI( 500 )
            km = m.convert_to( PrefixSI.kilo )
            self.assertEqual( km.value, 0.5 )
            print( colored( "OK", "green", attrs=["bold"] ) )
        except Exception as e:
            print( colored( "Failed!", "red", attrs=['bold'] ) )
            raise e

    def test_m_to_mm( self ):
        "Tests that high-to-low conversions are calculated correctly"

        print( colored( "TestPrefixConversion:test_m_to_mm: ", "white" ), end="" )
        try:
            m = SI( 500 )
            mm = m.convert_to( PrefixSI.milli )
            self.assertEqual( mm.value, 500000 )
            print( colored( "OK", "green", attrs=["bold"] ) )
        except Exception as e:
            print( colored( "Failed!", "red", attrs=['bold'] ) )
            raise e

    def test_mm_to_km( self ):
        "Tests that a very low to very high conversion is calculated correctly"

        print( colored( "TestPrefixConversion:test_mm_to_km: ", "white" ), end="" )
        try:
            mm = SI( 5000000, PrefixSI.milli )
            km = mm.convert_to( PrefixSI.kilo ).value
            self.assertEqual( km, 5 )
            print( colored( "OK", "green", attrs=["bold"] ) )
        except Exception as e:
            print( colored( "Failed!", "red", attrs=['bold'] ) )
            raise e