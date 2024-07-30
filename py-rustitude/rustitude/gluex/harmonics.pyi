from typing import Literal, overload

from rustitude import Amplitude, Amplitude_64, Amplitude_32
from rustitude.gluex import Decay, Frame, Reflectivity

@overload
def Ylm(
    name: str,
    l: Literal[0],  # noqa: E741
    m: Literal[0],
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude: ...
@overload
def Ylm(
    name: str,
    l: Literal[1],  # noqa: E741
    m: Literal[-1, 0, 1],
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude: ...
@overload
def Ylm(
    name: str,
    l: Literal[2],  # noqa: E741
    m: Literal[-2, -1, 0, 1, 2],
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude: ...
@overload
def Ylm(
    name: str,
    l: Literal[3],  # noqa: E741
    m: Literal[-3, -2, -1, 0, 1, 2, 3],
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude: ...
@overload
def Ylm_64(
    name: str,
    l: Literal[0],  # noqa: E741
    m: Literal[0],
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude_64: ...
@overload
def Ylm_64(
    name: str,
    l: Literal[1],  # noqa: E741
    m: Literal[-1, 0, 1],
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude_64: ...
@overload
def Ylm_64(
    name: str,
    l: Literal[2],  # noqa: E741
    m: Literal[-2, -1, 0, 1, 2],
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude_64: ...
@overload
def Ylm_64(
    name: str,
    l: Literal[3],  # noqa: E741
    m: Literal[-3, -2, -1, 0, 1, 2, 3],
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude_64: ...
@overload
def Ylm_32(
    name: str,
    l: Literal[0],  # noqa: E741
    m: Literal[0],
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude_32: ...
@overload
def Ylm_32(
    name: str,
    l: Literal[1],  # noqa: E741
    m: Literal[-1, 0, 1],
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude_32: ...
@overload
def Ylm_32(
    name: str,
    l: Literal[2],  # noqa: E741
    m: Literal[-2, -1, 0, 1, 2],
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude_32: ...
@overload
def Ylm_32(
    name: str,
    l: Literal[3],  # noqa: E741
    m: Literal[-3, -2, -1, 0, 1, 2, 3],
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude_32: ...
@overload
def Zlm(
    name: str,
    l: Literal[0],  # noqa: E741
    m: Literal[0],
    reflectivity: Reflectivity = Reflectivity.Positive,
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude: ...
@overload
def Zlm(
    name: str,
    l: Literal[1],  # noqa: E741
    m: Literal[-1, 0, 1],
    reflectivity: Reflectivity = Reflectivity.Positive,
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude: ...
@overload
def Zlm(
    name: str,
    l: Literal[2],  # noqa: E741
    m: Literal[-2, -1, 0, 1, 2],
    reflectivity: Reflectivity = Reflectivity.Positive,
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude: ...
@overload
def Zlm(
    name: str,
    l: Literal[3],  # noqa: E741
    m: Literal[-3, -2, -1, 0, 1, 2, 3],
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude: ...
@overload
def Zlm_64(
    name: str,
    l: Literal[0],  # noqa: E741
    m: Literal[0],
    reflectivity: Reflectivity = Reflectivity.Positive,
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude_64: ...
@overload
def Zlm_64(
    name: str,
    l: Literal[1],  # noqa: E741
    m: Literal[-1, 0, 1],
    reflectivity: Reflectivity = Reflectivity.Positive,
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude_64: ...
@overload
def Zlm_64(
    name: str,
    l: Literal[2],  # noqa: E741
    m: Literal[-2, -1, 0, 1, 2],
    reflectivity: Reflectivity = Reflectivity.Positive,
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude_64: ...
@overload
def Zlm_64(
    name: str,
    l: Literal[3],  # noqa: E741
    m: Literal[-3, -2, -1, 0, 1, 2, 3],
    reflectivity: Reflectivity = Reflectivity.Positive,
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude_64: ...
@overload
def Zlm_32(
    name: str,
    l: Literal[0],  # noqa: E741
    m: Literal[0],
    reflectivity: Reflectivity = Reflectivity.Positive,
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude_32: ...
@overload
def Zlm_32(
    name: str,
    l: Literal[1],  # noqa: E741
    m: Literal[-1, 0, 1],
    reflectivity: Reflectivity = Reflectivity.Positive,
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude_32: ...
@overload
def Zlm_32(
    name: str,
    l: Literal[2],  # noqa: E741
    m: Literal[-2, -1, 0, 1, 2],
    reflectivity: Reflectivity = Reflectivity.Positive,
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude_32: ...
@overload
def Zlm_32(
    name: str,
    l: Literal[3],  # noqa: E741
    m: Literal[-3, -2, -1, 0, 1, 2, 3],
    reflectivity: Reflectivity = Reflectivity.Positive,
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude_32: ...
def OnePS(  # noqa: N802
    name: str,
    reflectivity: Reflectivity = Reflectivity.Positive,
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude: ...
def OnePS_64(  # noqa: N802
    name: str,
    reflectivity: Reflectivity = Reflectivity.Positive,
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude_64: ...
def OnePS_32(  # noqa: N802
    name: str,
    reflectivity: Reflectivity = Reflectivity.Positive,
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude_32: ...
@overload
def TwoPS(
    name: str,
    l: Literal[0],  # noqa: E741
    m: Literal[0],
    reflectivity: Reflectivity = Reflectivity.Positive,
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude: ...
@overload
def TwoPS(
    name: str,
    l: Literal[1],  # noqa: E741
    m: Literal[-1, 0, 1],
    reflectivity: Reflectivity = Reflectivity.Positive,
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude: ...
@overload
def TwoPS(
    name: str,
    l: Literal[2],  # noqa: E741
    m: Literal[-2, -1, 0, 1, 2],
    reflectivity: Reflectivity = Reflectivity.Positive,
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude: ...
@overload
def TwoPS(
    name: str,
    l: Literal[3],  # noqa: E741
    m: Literal[-3, -2, -1, 0, 1, 2, 3],
    reflectivity: Reflectivity = Reflectivity.Positive,
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude: ...
@overload
def TwoPS_64(
    name: str,
    l: Literal[0],  # noqa: E741
    m: Literal[0],
    reflectivity: Reflectivity = Reflectivity.Positive,
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude_64: ...
@overload
def TwoPS_64(
    name: str,
    l: Literal[1],  # noqa: E741
    m: Literal[-1, 0, 1],
    reflectivity: Reflectivity = Reflectivity.Positive,
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude_64: ...
@overload
def TwoPS_64(
    name: str,
    l: Literal[2],  # noqa: E741
    m: Literal[-2, -1, 0, 1, 2],
    reflectivity: Reflectivity = Reflectivity.Positive,
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude_64: ...
@overload
def TwoPS_64(
    name: str,
    l: Literal[3],  # noqa: E741
    m: Literal[-3, -2, -1, 0, 1, 2, 3],
    reflectivity: Reflectivity = Reflectivity.Positive,
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude_64: ...
@overload
def TwoPS_32(
    name: str,
    l: Literal[0],  # noqa: E741
    m: Literal[0],
    reflectivity: Reflectivity = Reflectivity.Positive,
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude_32: ...
@overload
def TwoPS_32(
    name: str,
    l: Literal[1],  # noqa: E741
    m: Literal[-1, 0, 1],
    reflectivity: Reflectivity = Reflectivity.Positive,
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude_32: ...
@overload
def TwoPS_32(
    name: str,
    l: Literal[2],  # noqa: E741
    m: Literal[-2, -1, 0, 1, 2],
    reflectivity: Reflectivity = Reflectivity.Positive,
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude_32: ...
@overload
def TwoPS_32(
    name: str,
    l: Literal[3],  # noqa: E741
    m: Literal[-3, -2, -1, 0, 1, 2, 3],
    reflectivity: Reflectivity = Reflectivity.Positive,
    decay: Decay = Decay.TwoBodyDecay([0, 1]),
    frame: Frame = Frame.Helicity,
) -> Amplitude_32: ...
