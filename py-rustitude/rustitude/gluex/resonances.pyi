from rustitude import AmpOp


def BreitWigner(name: str, p1_indices: list[int], p2_indices: list[int]) -> AmpOp: ...  # noqa: N802
def KMatrixA0(name: str, channel: int) -> AmpOp: ...  # noqa: N802
def KMatrixA2(name: str, channel: int) -> AmpOp: ...  # noqa: N802
def KMatrixF0(name: str, channel: int) -> AmpOp: ...  # noqa: N802
def KMatrixF2(name: str, channel: int) -> AmpOp: ...  # noqa: N802
def KMatrixPi1(name: str, channel: int) -> AmpOp: ...  # noqa: N802
def KMatrixRho(name: str, channel: int) -> AmpOp: ...  # noqa: N802