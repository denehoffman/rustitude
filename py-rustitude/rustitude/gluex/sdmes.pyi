from typing import Literal

from rustitude import Amplitude

def TwoPiSDME(  # noqa: N802
    name: str,
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude: ...
def ThreePiSDME(  # noqa: N802
    name: str,
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> Amplitude: ...
