from pathlib import Path

from rustitude.dataset import Event

class Amplitude: ...

class Dataset:
    def __getitem__(self, index: int) -> Event: ...
    def __len__(self) -> int: ...
    def split_m(
        self,
        range: tuple[float, float],  # noqa: A002
        bins: int,
        p1_indices: list[float] | None = None,
        p2_indices: list[float] | None = None,
    ) -> tuple[list[Dataset], Dataset, Dataset]: ...
    @staticmethod
    def from_dict(data: dict[str, list[float | list[float]]]) -> Dataset: ...
    @staticmethod
    def from_parquet(path: str) -> Dataset: ...
    @staticmethod
    def from_parquet_eps_in_beam(path: str) -> Dataset: ...
    @staticmethod
    def from_parquet_with_eps(path: str, eps: list[float]) -> Dataset: ...
    @staticmethod
    def from_parquet_unpolarized(path: str) -> Dataset: ...
    @staticmethod
    def from_root(path: str) -> Dataset: ...

def open(file_name: str | Path, tree_name: str | None = None, *, pol_in_beam: bool = False) -> Dataset: ...  # noqa: A001

class Manager:
    def __init__(self, dataset: Dataset) -> None: ...
    def __call__(self, parameters: list[float]) -> list[float]: ...
    def register(self, sum_name: str, group_name: str, amplitude: Amplitude) -> None: ...
    def constrain(self, parameter_1: tuple[str, str, str, str], parameter_2: tuple[str, str, str, str]) -> None: ...
    def constrain_amplitude(self, amplitude_1: tuple[str, str, str], amplitude_2: tuple[str, str, str]) -> None: ...
    def activate(self, amplitude: tuple[str, str, str]) -> None: ...
    def deactivate(self, amplitude: tuple[str, str, str]) -> None: ...
    def fix(self, parameter: tuple[str, str, str, str], value: float) -> None: ...
    def free(self, parameter: tuple[str, str, str, str], initial_value: float) -> None: ...
    def set_initial(self, parameter: tuple[str, str, str, str], initial_value: float) -> None: ...
    def set_bounds(self, parameter: tuple[str, str, str, str], lower_bound: float, upper_bound: float) -> None: ...
    def get_initial(self, *, fixed: bool = False, constrained: bool = False) -> list[float]: ...
    def get_bounds(self, *, fixed: bool = False, constrained: bool = False) -> list[tuple[float, float]]: ...
    def get_lower_bounds(self, *, fixed: bool = False, constrained: bool = False) -> list[float]: ...
    def get_upper_bounds(self, *, fixed: bool = False, constrained: bool = False) -> list[float]: ...
    def parameters(self, *, fixed: bool = False, constrained: bool = False) -> list[tuple[str, str, str, str]]: ...

class MultiManager:
    def __init__(self, datasets: list[Dataset]) -> None: ...
    def register(self, sum_name: str, group_name: str, amplitude: Amplitude) -> None: ...
    def constrain(self, parameter_1: tuple[str, str, str, str], parameter_2: tuple[str, str, str, str]) -> None: ...
    def constrain_amplitude(self, amplitude_1: tuple[str, str, str], amplitude_2: tuple[str, str, str]) -> None: ...
    def activate(self, amplitude: tuple[str, str, str]) -> None: ...
    def deactivate(self, amplitude: tuple[str, str, str]) -> None: ...
    def fix(self, parameter: tuple[str, str, str, str], value: float) -> None: ...
    def free(self, parameter: tuple[str, str, str, str], initial_value: float) -> None: ...
    def set_initial(self, parameter: tuple[str, str, str, str], initial_value: float) -> None: ...
    def set_bounds(self, parameter: tuple[str, str, str, str], lower_bound: float, upper_bound: float) -> None: ...
    def get_initial(self, *, fixed: bool = False, constrained: bool = False) -> list[float]: ...
    def get_bounds(self, *, fixed: bool = False, constrained: bool = False) -> list[tuple[float, float]]: ...
    def get_lower_bounds(self, *, fixed: bool = False, constrained: bool = False) -> list[float]: ...
    def get_upper_bounds(self, *, fixed: bool = False, constrained: bool = False) -> list[float]: ...
    def parameters(self, *, fixed: bool = False, constrained: bool = False) -> list[tuple[str, str, str, str]]: ...

class ExtendedLogLikelihood:
    def __init__(self, dataset: Dataset, montecarlo: Dataset) -> None: ...
    def __call__(self, parameters: list[float]) -> float: ...
    def register(self, sum_name: str, group_name: str, amplitude: Amplitude) -> None: ...
    def constrain(self, parameter_1: tuple[str, str, str, str], parameter_2: tuple[str, str, str, str]) -> None: ...
    def constrain_amplitude(self, amplitude_1: tuple[str, str, str], amplitude_2: tuple[str, str, str]) -> None: ...
    def activate(self, amplitude: tuple[str, str, str]) -> None: ...
    def deactivate(self, amplitude: tuple[str, str, str]) -> None: ...
    def fix(self, parameter: tuple[str, str, str, str], value: float) -> None: ...
    def free(self, parameter: tuple[str, str, str, str], initial_value: float) -> None: ...
    def set_initial(self, parameter: tuple[str, str, str, str], initial_value: float) -> None: ...
    def set_bounds(self, parameter: tuple[str, str, str, str], lower_bound: float, upper_bound: float) -> None: ...
    def get_initial(self, *, fixed: bool = False, constrained: bool = False) -> list[float]: ...
    def get_bounds(self, *, fixed: bool = False, constrained: bool = False) -> list[tuple[float, float]]: ...
    def get_lower_bounds(self, *, fixed: bool = False, constrained: bool = False) -> list[float]: ...
    def get_upper_bounds(self, *, fixed: bool = False, constrained: bool = False) -> list[float]: ...
    def parameters(self, *, fixed: bool = False, constrained: bool = False) -> list[tuple[str, str, str, str]]: ...
