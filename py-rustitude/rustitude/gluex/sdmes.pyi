from typing import Literal

from rustitude import Amplitude, Amplitude64, Amplitude32

def TwoPiSDME(  # noqa: N802
    name: str,
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude: ...
def TwoPiSDME64(  # noqa: N802
    name: str,
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude64: ...
def TwoPiSDME32(  # noqa: N802
    name: str,
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude32: ...
def ThreePiSDME(  # noqa: N802
    name: str,
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude: ...
def ThreePiSDME64(  # noqa: N802
    name: str,
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude64: ...
def ThreePiSDME32(  # noqa: N802
    name: str,
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude32: ...
