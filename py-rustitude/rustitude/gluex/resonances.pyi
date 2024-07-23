from rustitude import Amplitude, Amplitude_64, Amplitude_32

def BreitWigner(
    name: str,
    p1_indices: list[int],
    p2_indices: list[int],
    l: int,  # noqa: E741
) -> Amplitude: ...  # noqa: N802
def BreitWigner_64(
    name: str,
    p1_indices: list[int],
    p2_indices: list[int],
    l: int,  # noqa: E741
) -> Amplitude_64: ...  # noqa: N802
def BreitWigner_32(
    name: str,
    p1_indices: list[int],
    p2_indices: list[int],
    l: int,  # noqa: E741
) -> Amplitude_32: ...  # noqa: N802
def KMatrixA0(name: str, channel: int) -> Amplitude: ...  # noqa: N802
def KMatrixA0_64(name: str, channel: int) -> Amplitude_64: ...  # noqa: N802
def KMatrixA0_32(name: str, channel: int) -> Amplitude_32: ...  # noqa: N802
def KMatrixA2(name: str, channel: int) -> Amplitude: ...  # noqa: N802
def KMatrixA2_64(name: str, channel: int) -> Amplitude_64: ...  # noqa: N802
def KMatrixA2_32(name: str, channel: int) -> Amplitude_32: ...  # noqa: N802
def KMatrixF0(name: str, channel: int) -> Amplitude: ...  # noqa: N802
def KMatrixF0_64(name: str, channel: int) -> Amplitude_64: ...  # noqa: N802
def KMatrixF0_32(name: str, channel: int) -> Amplitude_32: ...  # noqa: N802
def KMatrixF2(name: str, channel: int) -> Amplitude: ...  # noqa: N802
def KMatrixF2_64(name: str, channel: int) -> Amplitude_64: ...  # noqa: N802
def KMatrixF2_32(name: str, channel: int) -> Amplitude_32: ...  # noqa: N802
def KMatrixPi1(name: str, channel: int) -> Amplitude: ...  # noqa: N802
def KMatrixPi1_64(name: str, channel: int) -> Amplitude_64: ...  # noqa: N802
def KMatrixPi1_32(name: str, channel: int) -> Amplitude_32: ...  # noqa: N802
def KMatrixRho(name: str, channel: int) -> Amplitude: ...  # noqa: N802
def KMatrixRho_64(name: str, channel: int) -> Amplitude_64: ...  # noqa: N802
def KMatrixRho_32(name: str, channel: int) -> Amplitude_32: ...  # noqa: N802
