from rustitude import Amplitude, Amplitude_64, Amplitude_32
from rustitude.gluex import Decay

def BreitWigner(
    name: str,
    l: int,  # noqa: E741
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
) -> Amplitude: ...  # noqa: N802
def BreitWigner_64(
    name: str,
    l: int,  # noqa: E741
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
) -> Amplitude_64: ...  # noqa: N802
def BreitWigner_32(
    name: str,
    l: int,  # noqa: E741
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
) -> Amplitude_32: ...  # noqa: N802
def KMatrixA0(name: str, channel: int, decay: Decay = Decay.TwoBodyDecay([0, 1])) -> Amplitude: ...  # noqa: N802
def KMatrixA0_64(
    name: str, channel: int, decay: Decay = Decay.TwoBodyDecay([0, 1])
) -> Amplitude_64: ...  # noqa: N802
def KMatrixA0_32(
    name: str, channel: int, decay: Decay = Decay.TwoBodyDecay([0, 1])
) -> Amplitude_32: ...  # noqa: N802
def KMatrixA2(name: str, channel: int, decay: Decay = Decay.TwoBodyDecay([0, 1])) -> Amplitude: ...  # noqa: N802
def KMatrixA2_64(
    name: str, channel: int, decay: Decay = Decay.TwoBodyDecay([0, 1])
) -> Amplitude_64: ...  # noqa: N802
def KMatrixA2_32(
    name: str, channel: int, decay: Decay = Decay.TwoBodyDecay([0, 1])
) -> Amplitude_32: ...  # noqa: N802
def KMatrixF0(name: str, channel: int, decay: Decay = Decay.TwoBodyDecay([0, 1])) -> Amplitude: ...  # noqa: N802
def KMatrixF0_64(
    name: str, channel: int, decay: Decay = Decay.TwoBodyDecay([0, 1])
) -> Amplitude_64: ...  # noqa: N802
def KMatrixF0_32(
    name: str, channel: int, decay: Decay = Decay.TwoBodyDecay([0, 1])
) -> Amplitude_32: ...  # noqa: N802
def KMatrixF2(name: str, channel: int, decay: Decay = Decay.TwoBodyDecay([0, 1])) -> Amplitude: ...  # noqa: N802
def KMatrixF2_64(
    name: str, channel: int, decay: Decay = Decay.TwoBodyDecay([0, 1])
) -> Amplitude_64: ...  # noqa: N802
def KMatrixF2_32(
    name: str, channel: int, decay: Decay = Decay.TwoBodyDecay([0, 1])
) -> Amplitude_32: ...  # noqa: N802
def KMatrixPi1(name: str, channel: int, decay: Decay = Decay.TwoBodyDecay([0, 1])) -> Amplitude: ...  # noqa: N802
def KMatrixPi1_64(
    name: str, channel: int, decay: Decay = Decay.TwoBodyDecay([0, 1])
) -> Amplitude_64: ...  # noqa: N802
def KMatrixPi1_32(
    name: str, channel: int, decay: Decay = Decay.TwoBodyDecay([0, 1])
) -> Amplitude_32: ...  # noqa: N802
def KMatrixRho(name: str, channel: int, decay: Decay = Decay.TwoBodyDecay([0, 1])) -> Amplitude: ...  # noqa: N802
def KMatrixRho_64(
    name: str, channel: int, decay: Decay = Decay.TwoBodyDecay([0, 1])
) -> Amplitude_64: ...  # noqa: N802
def KMatrixRho_32(
    name: str, channel: int, decay: Decay = Decay.TwoBodyDecay([0, 1])
) -> Amplitude_32: ...  # noqa: N802
