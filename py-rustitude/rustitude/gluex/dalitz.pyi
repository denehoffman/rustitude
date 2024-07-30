from rustitude import Amplitude, Amplitude_64, Amplitude_32
from rustitude.gluex import Decay

def OmegaDalitz(name: str, decay: Decay = Decay.ThreeBodyDecay([0, 1, 2])) -> Amplitude: ...  # noqa: N802
def OmegaDalitz_64(name: str, decay: Decay = Decay.ThreeBodyDecay([0, 1, 2])) -> Amplitude_64: ...  # noqa: N802
def OmegaDalitz_32(name: str, decay: Decay = Decay.ThreeBodyDecay([0, 1, 2])) -> Amplitude_32: ...  # noqa: N802
