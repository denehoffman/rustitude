from typing import Literal, Sequence
from . import resonances, sdmes, harmonics, dalitz, polarization

class Wave:
    S: Wave
    S0: Wave
    P: Wave
    Pn1: Wave
    P0: Wave
    P1: Wave
    D: Wave
    Dn2: Wave
    Dn1: Wave
    D0: Wave
    D1: Wave
    D2: Wave
    F: Wave
    Fn3: Wave
    Fn2: Wave
    Fn1: Wave
    F0: Wave
    F1: Wave
    F2: Wave
    F3: Wave

class Sign:
    Positive: Sign
    Negative: Sign

class Frame:
    Helicity: Frame
    GottfriedJackson: Frame

class Decay:
    class TwoBodyDecay:
        def __new__(cls, params: list[int]) -> Decay: ...

    class ThreeBodyDecay:
        def __new__(cls, params: list[int]) -> Decay: ...
