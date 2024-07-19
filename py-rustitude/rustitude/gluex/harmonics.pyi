from typing import Literal, overload

from rustitude import Amplitude, Amplitude64, Amplitude32

@overload
def Ylm(
    name: str,
    l: Literal[0],  # noqa: E741
    m: Literal[0],
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude: ...
@overload
def Ylm(
    name: str,
    l: Literal[1],  # noqa: E741
    m: Literal[-1, 0, 1],
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude: ...
@overload
def Ylm(
    name: str,
    l: Literal[2],  # noqa: E741
    m: Literal[-2, -1, 0, 1, 2],
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude: ...
@overload
def Ylm(
    name: str,
    l: Literal[3],  # noqa: E741
    m: Literal[-3, -2, -1, 0, 1, 2, 3],
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude: ...
@overload
def Ylm64(
    name: str,
    l: Literal[0],  # noqa: E741
    m: Literal[0],
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude64: ...
@overload
def Ylm64(
    name: str,
    l: Literal[1],  # noqa: E741
    m: Literal[-1, 0, 1],
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude64: ...
@overload
def Ylm64(
    name: str,
    l: Literal[2],  # noqa: E741
    m: Literal[-2, -1, 0, 1, 2],
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude64: ...
@overload
def Ylm64(
    name: str,
    l: Literal[3],  # noqa: E741
    m: Literal[-3, -2, -1, 0, 1, 2, 3],
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude64: ...
@overload
def Ylm32(
    name: str,
    l: Literal[0],  # noqa: E741
    m: Literal[0],
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude32: ...
@overload
def Ylm32(
    name: str,
    l: Literal[1],  # noqa: E741
    m: Literal[-1, 0, 1],
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude32: ...
@overload
def Ylm32(
    name: str,
    l: Literal[2],  # noqa: E741
    m: Literal[-2, -1, 0, 1, 2],
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude32: ...
@overload
def Ylm32(
    name: str,
    l: Literal[3],  # noqa: E741
    m: Literal[-3, -2, -1, 0, 1, 2, 3],
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude32: ...
@overload
def Zlm(
    name: str,
    l: Literal[0],  # noqa: E741
    m: Literal[0],
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude: ...
@overload
def Zlm(
    name: str,
    l: Literal[1],  # noqa: E741
    m: Literal[-1, 0, 1],
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude: ...
@overload
def Zlm(
    name: str,
    l: Literal[2],  # noqa: E741
    m: Literal[-2, -1, 0, 1, 2],
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
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
def Zlm64(
    name: str,
    l: Literal[0],  # noqa: E741
    m: Literal[0],
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude64: ...
@overload
def Zlm64(
    name: str,
    l: Literal[1],  # noqa: E741
    m: Literal[-1, 0, 1],
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude64: ...
@overload
def Zlm64(
    name: str,
    l: Literal[2],  # noqa: E741
    m: Literal[-2, -1, 0, 1, 2],
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude64: ...
@overload
def Zlm64(
    name: str,
    l: Literal[3],  # noqa: E741
    m: Literal[-3, -2, -1, 0, 1, 2, 3],
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude64: ...
@overload
def Zlm32(
    name: str,
    l: Literal[0],  # noqa: E741
    m: Literal[0],
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude32: ...
@overload
def Zlm32(
    name: str,
    l: Literal[1],  # noqa: E741
    m: Literal[-1, 0, 1],
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude32: ...
@overload
def Zlm32(
    name: str,
    l: Literal[2],  # noqa: E741
    m: Literal[-2, -1, 0, 1, 2],
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude32: ...
@overload
def Zlm32(
    name: str,
    l: Literal[3],  # noqa: E741
    m: Literal[-3, -2, -1, 0, 1, 2, 3],
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude32: ...
def OnePS(  # noqa: N802
    name: str,
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude: ...
def OnePS64(  # noqa: N802
    name: str,
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude64: ...
def OnePS32(  # noqa: N802
    name: str,
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude32: ...
@overload
def TwoPS(
    name: str,
    l: Literal[0],  # noqa: E741
    m: Literal[0],
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude: ...
@overload
def TwoPS(
    name: str,
    l: Literal[1],  # noqa: E741
    m: Literal[-1, 0, 1],
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    part: Literal['real', 're', 'imaginary', 'imag', 'im', 'both'] = 'real',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude: ...
@overload
def TwoPS(
    name: str,
    l: Literal[2],  # noqa: E741
    m: Literal[-2, -1, 0, 1, 2],
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude: ...
@overload
def TwoPS(
    name: str,
    l: Literal[3],  # noqa: E741
    m: Literal[-3, -2, -1, 0, 1, 2, 3],
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude: ...
@overload
def TwoPS64(
    name: str,
    l: Literal[0],  # noqa: E741
    m: Literal[0],
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude64: ...
@overload
def TwoPS64(
    name: str,
    l: Literal[1],  # noqa: E741
    m: Literal[-1, 0, 1],
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    part: Literal['real', 're', 'imaginary', 'imag', 'im', 'both'] = 'real',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude64: ...
@overload
def TwoPS64(
    name: str,
    l: Literal[2],  # noqa: E741
    m: Literal[-2, -1, 0, 1, 2],
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude64: ...
@overload
def TwoPS64(
    name: str,
    l: Literal[3],  # noqa: E741
    m: Literal[-3, -2, -1, 0, 1, 2, 3],
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude64: ...
@overload
def TwoPS32(
    name: str,
    l: Literal[0],  # noqa: E741
    m: Literal[0],
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude32: ...
@overload
def TwoPS32(
    name: str,
    l: Literal[1],  # noqa: E741
    m: Literal[-1, 0, 1],
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    part: Literal['real', 're', 'imaginary', 'imag', 'im', 'both'] = 'real',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude32: ...
@overload
def TwoPS32(
    name: str,
    l: Literal[2],  # noqa: E741
    m: Literal[-2, -1, 0, 1, 2],
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude32: ...
@overload
def TwoPS32(
    name: str,
    l: Literal[3],  # noqa: E741
    m: Literal[-3, -2, -1, 0, 1, 2, 3],
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude32: ...
