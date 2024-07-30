from rustitude import Amplitude, Amplitude_64, Amplitude_32
from rustitude.gluex import Decay, Frame

def TwoPiSDME(  # noqa: N802
    name: str, decay: Decay = Decay.TwoBodyDecay([0, 1]), frame: Frame = Frame.Helicity
) -> Amplitude: ...
def TwoPiSDME_64(  # noqa: N802
    name: str, decay: Decay = Decay.TwoBodyDecay([0, 1]), frame: Frame = Frame.Helicity
) -> Amplitude_64: ...
def TwoPiSDME_32(  # noqa: N802
    name: str, decay: Decay = Decay.TwoBodyDecay([0, 1]), frame: Frame = Frame.Helicity
) -> Amplitude_32: ...
def ThreePiSDME(  # noqa: N802
    name: str, decay: Decay = Decay.TwoBodyDecay([0, 1, 2]), frame: Frame = Frame.Helicity
) -> Amplitude: ...
def ThreePiSDME_64(  # noqa: N802
    name: str, decay: Decay = Decay.TwoBodyDecay([0, 1, 2]), frame: Frame = Frame.Helicity
) -> Amplitude_64: ...
def ThreePiSDME_32(  # noqa: N802
    name: str, decay: Decay = Decay.TwoBodyDecay([0, 1, 2]), frame: Frame = Frame.Helicity
) -> Amplitude_32: ...
