from typing import Literal

from rustitude import Amplitude, Amplitude_64, Amplitude_32

def TwoPiSDME(  # noqa: N802
    name: str,
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude: ...
def TwoPiSDME_64(  # noqa: N802
    name: str,
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude_64: ...
def TwoPiSDME_32(  # noqa: N802
    name: str,
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude_32: ...
def ThreePiSDME(  # noqa: N802
    name: str,
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude: ...
def ThreePiSDME_64(  # noqa: N802
    name: str,
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude_64: ...
def ThreePiSDME_32(  # noqa: N802
    name: str,
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude_32: ...
