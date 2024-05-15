from typing import Literal, overload

from rustitude import AmpOp

@overload
def Ylm(
    name: str,
    l: Literal[0],  # noqa: E741
    m: Literal[0],
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> AmpOp: ...
@overload
def Ylm(
    name: str,
    l: Literal[1],  # noqa: E741
    m: Literal[-1, 0, 1],
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> AmpOp: ...
@overload
def Ylm(
    name: str,
    l: Literal[2],  # noqa: E741
    m: Literal[-2, -1, 0, 1, 2],
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> AmpOp: ...
@overload
def Ylm(
    name: str,
    l: Literal[3],  # noqa: E741
    m: Literal[-3, -2, -1, 0, 1, 2, 3],
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> AmpOp: ...
@overload
def Zlm(
    name: str,
    l: Literal[0],  # noqa: E741
    m: Literal[0],
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> AmpOp: ...
@overload
def Zlm(
    name: str,
    l: Literal[1],  # noqa: E741
    m: Literal[-1, 0, 1],
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> AmpOp: ...
@overload
def Zlm(
    name: str,
    l: Literal[2],  # noqa: E741
    m: Literal[-2, -1, 0, 1, 2],
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> AmpOp: ...
@overload
def Zlm(
    name: str,
    l: Literal[3],  # noqa: E741
    m: Literal[-3, -2, -1, 0, 1, 2, 3],
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> AmpOp: ...
def OnePS(  # noqa: N802
    name: str,
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> AmpOp: ...
@overload
def TwoPS(
    name: str,
    l: Literal[0],  # noqa: E741
    m: Literal[0],
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> AmpOp: ...
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
) -> AmpOp: ...
@overload
def TwoPS(
    name: str,
    l: Literal[2],  # noqa: E741
    m: Literal[-2, -1, 0, 1, 2],
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> AmpOp: ...
@overload
def TwoPS(
    name: str,
    l: Literal[3],  # noqa: E741
    m: Literal[-3, -2, -1, 0, 1, 2, 3],
    reflectivity: Literal[
        'positive', 'pos', 'p', '+', 'plus', 'negative', 'neg', 'n', '-', 'minus', 'm'
    ] = 'positive',
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> AmpOp: ...
