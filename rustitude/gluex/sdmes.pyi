from typing import Literal

from rustitude import AmpOp


def TwoPiSDME(  # noqa: N802
    name: str,
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> AmpOp: ...
def ThreePiSDME(  # noqa: N802
    name: str,
    frame: Literal['helicity', 'hx', 'gottfried-jackson', 'gj'] = 'helicity',
) -> AmpOp: ...
