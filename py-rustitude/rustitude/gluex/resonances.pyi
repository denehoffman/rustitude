from rustitude import Amplitude, Amplitude64, Amplitude32

def BreitWigner(name: str, p1_indices: list[int], p2_indices: list[int], l: int) -> Amplitude: ...  # noqa: N802
def BreitWigner64(
    name: str, p1_indices: list[int], p2_indices: list[int], l: int
) -> Amplitude64: ...  # noqa: N802
def BreitWigner32(
    name: str, p1_indices: list[int], p2_indices: list[int], l: int
) -> Amplitude32: ...  # noqa: N802
def KMatrixA0(name: str, channel: int) -> Amplitude: ...  # noqa: N802
def KMatrixA064(name: str, channel: int) -> Amplitude64: ...  # noqa: N802
def KMatrixA032(name: str, channel: int) -> Amplitude32: ...  # noqa: N802
def KMatrixA2(name: str, channel: int) -> Amplitude: ...  # noqa: N802
def KMatrixA264(name: str, channel: int) -> Amplitude64: ...  # noqa: N802
def KMatrixA232(name: str, channel: int) -> Amplitude32: ...  # noqa: N802
def KMatrixF0(name: str, channel: int) -> Amplitude: ...  # noqa: N802
def KMatrixF064(name: str, channel: int) -> Amplitude64: ...  # noqa: N802
def KMatrixF032(name: str, channel: int) -> Amplitude32: ...  # noqa: N802
def KMatrixF2(name: str, channel: int) -> Amplitude: ...  # noqa: N802
def KMatrixF264(name: str, channel: int) -> Amplitude64: ...  # noqa: N802
def KMatrixF232(name: str, channel: int) -> Amplitude32: ...  # noqa: N802
def KMatrixPi1(name: str, channel: int) -> Amplitude: ...  # noqa: N802
def KMatrixPi164(name: str, channel: int) -> Amplitude64: ...  # noqa: N802
def KMatrixPi132(name: str, channel: int) -> Amplitude32: ...  # noqa: N802
def KMatrixRho(name: str, channel: int) -> Amplitude: ...  # noqa: N802
def KMatrixRho64(name: str, channel: int) -> Amplitude64: ...  # noqa: N802
def KMatrixRho32(name: str, channel: int) -> Amplitude32: ...  # noqa: N802
