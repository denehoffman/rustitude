from rustitude import Amplitude, Amplitude_64, Amplitude_32
from rustitude.gluex import Decay, Sign

def ThreePiPolFrac(
    name: str,
    beam_pol: Sign,
    j_resonance: int,
    p_resonance: int,
    i_resonance: int,
    l_resonance: int,
    j_isobar: int,
    i_isobar: int,
    iz_daughters: tuple[int, int, int],
    decay_resonance=Decay.ThreeBodyDecay([0, 1, 2]),
    decay_isobar=Decay.TwoBodyDecay([0, 1]),
) -> Amplitude: ...
def ThreePiPolFrac_64(
    name: str,
    beam_pol: Sign,
    j_resonance: int,
    p_resonance: int,
    i_resonance: int,
    l_resonance: int,
    j_isobar: int,
    i_isobar: int,
    iz_daughters: tuple[int, int, int],
    decay_resonance=Decay.ThreeBodyDecay([0, 1, 2]),
    decay_isobar=Decay.TwoBodyDecay([0, 1]),
) -> Amplitude_64: ...
def ThreePiPolFrac_32(
    name: str,
    beam_pol: Sign,
    j_resonance: int,
    p_resonance: int,
    i_resonance: int,
    l_resonance: int,
    j_isobar: int,
    i_isobar: int,
    iz_daughters: tuple[int, int, int],
    decay_resonance=Decay.ThreeBodyDecay([0, 1, 2]),
    decay_isobar=Decay.TwoBodyDecay([0, 1]),
) -> Amplitude_32: ...
