from pathlib import Path
from typing import Any, Callable, Literal, Protocol, Self, overload
from abc import ABCMeta, abstractmethod

from iminuit import Minuit
from numpy.typing import ArrayLike
from scipy.optimize import OptimizeResult

__version__: str

class Parameter_64:
    amplitude: str
    name: str
    index: int | None
    fixed_index: int | None
    free: bool
    fixed: bool
    initial: float
    bounds: tuple[float, float]

    def __init__(self, amplitude: str, name: str, index: int) -> None: ...

class Parameter_32:
    amplitude: str
    name: str
    index: int | None
    fixed_index: int | None
    free: bool
    fixed: bool
    initial: float
    bounds: tuple[float, float]

    def __init__(self, amplitude: str, name: str, index: int) -> None: ...

Parameter = Parameter_64

class PyNode_64(metaclass=ABCMeta):
    @abstractmethod
    def precalculate(self, dataset: Dataset_64) -> None: ...
    @abstractmethod
    def calculate(self, parameters: list[float], event: Event) -> complex: ...
    @abstractmethod
    def parameters(self) -> list[str]: ...

class PyNode_32(metaclass=ABCMeta):
    @abstractmethod
    def precalculate(self, dataset: Dataset_32) -> None: ...
    @abstractmethod
    def calculate(self, parameters: list[float], event: Event) -> complex: ...
    @abstractmethod
    def parameters(self) -> list[str]: ...

PyNode = PyNode_64

class Node_64:
    def __init__(self, pynode: PyNode_64) -> None: ...
    def precalculate(self, dataset: Dataset_64) -> None: ...
    def calculate(self, parameters: list[float], event: Event) -> complex: ...
    def parameters(self) -> list[str]: ...
    def into_amplitude(self, name: str) -> Amplitude_64: ...

class Node_32:
    def __init__(self, pynode: PyNode_32) -> None: ...
    def precalculate(self, dataset: Dataset_32) -> None: ...
    def calculate(self, parameters: list[float], event: Event) -> complex: ...
    def parameters(self) -> list[str]: ...
    def into_amplitude(self, name: str) -> Amplitude_32: ...

Node = Node_64

class Amplitude_64:
    name: str
    active: bool
    cache_position: int
    parameter_index_start: int

    def __init__(self, name: str, node: Node_64) -> None: ...
    def real(self) -> Real_64: ...
    def imag(self) -> Imag_64: ...
    def __add__(self, other: Self | Real_64 | Imag_64 | Product_64 | Sum_64) -> Sum_64: ...
    @overload
    def __mul__(self, other: Self | Real_64 | Imag_64 | Product_64) -> Product_64: ...
    @overload
    def __mul__(self, other: Sum_64) -> Sum_64: ...

class Amplitude_32:
    name: str
    active: bool
    cache_position: int
    parameter_index_start: int

    def __init__(self, name: str, node: Node_32) -> None: ...
    def real(self) -> Real_32: ...
    def imag(self) -> Imag_32: ...
    def __add__(self, other: Self | Real_32 | Imag_32 | Product_32 | Sum_32) -> Sum_32: ...
    @overload
    def __mul__(self, other: Self | Real_32 | Imag_32 | Product_32) -> Product_32: ...
    @overload
    def __mul__(self, other: Sum_32) -> Sum_32: ...

Amplitude = Amplitude_64

class Real_64:
    def real(self) -> Real_64: ...
    def imag(self) -> Imag_64: ...
    def __add__(self, other: Amplitude_64 | Self | Imag_64 | Product_64 | Sum_64) -> Sum_64: ...
    @overload
    def __mul__(self, other: Amplitude_64 | Self | Imag_64 | Product_64) -> Product_64: ...
    @overload
    def __mul__(self, other: Sum_64) -> Sum_64: ...

class Real_32:
    def real(self) -> Real_32: ...
    def imag(self) -> Imag_32: ...
    def __add__(self, other: Amplitude_32 | Self | Imag_32 | Product_32 | Sum_32) -> Sum_32: ...
    @overload
    def __mul__(self, other: Amplitude_32 | Self | Imag_32 | Product_32) -> Product_32: ...
    @overload
    def __mul__(self, other: Sum_32) -> Sum_32: ...

Real = Real_64

class Imag_64:
    def real(self) -> Real_64: ...
    def imag(self) -> Imag_64: ...
    def __add__(self, other: Amplitude_64 | Real_64 | Self | Product_64 | Sum_64) -> Sum_64: ...
    @overload
    def __mul__(self, other: Amplitude_64 | Real_64 | Self | Product_64) -> Product_64: ...
    @overload
    def __mul__(self, other: Sum_64) -> Sum_64: ...

class Imag_32:
    def real(self) -> Real_32: ...
    def imag(self) -> Imag_32: ...
    def __add__(self, other: Amplitude_32 | Real_32 | Self | Product_32 | Sum_32) -> Sum_32: ...
    @overload
    def __mul__(self, other: Amplitude_32 | Real_32 | Self | Product_32) -> Product_32: ...
    @overload
    def __mul__(self, other: Sum_32) -> Sum_32: ...

Imag = Imag_64

class Product_64:
    def real(self) -> Real_64: ...
    def imag(self) -> Imag_64: ...
    def __add__(self, other: Amplitude_64 | Real_64 | Imag_64 | Self | Sum_64) -> Sum_64: ...
    @overload
    def __mul__(self, other: Amplitude_64 | Real_64 | Imag_64) -> Self: ...
    @overload
    def __mul__(self, other: Sum_64) -> Sum_64: ...

class Product_32:
    def as_cohsum(self) -> Sum_32: ...
    def real(self) -> Real_32: ...
    def imag(self) -> Imag_32: ...
    def __add__(self, other: Amplitude_32 | Real_32 | Imag_32 | Self | Sum_32) -> Sum_32: ...
    @overload
    def __mul__(self, other: Amplitude_32 | Real_32 | Imag_32) -> Self: ...
    @overload
    def __mul__(self, other: Sum_32) -> Sum_32: ...

Product = Product_64

class Sum_64:
    def __init__(
        self, terms: list[Amplitude_64 | Real_64 | Imag_64 | Product_64 | Self]
    ) -> None: ...
    def real(self) -> Real_64: ...
    def imag(self) -> Imag_64: ...
    def __add__(self, other: Self | Amplitude_64 | Real_64 | Imag_64 | Product_64) -> Self: ...
    def __mul__(self, other: Amplitude_64 | Real_64 | Imag_64 | Product_64) -> Self: ...

class Sum_32:
    def __init__(
        self, terms: list[Amplitude_32 | Real_32 | Imag_32 | Product_32 | Sum_32]
    ) -> None: ...
    def real(self) -> Real_32: ...
    def imag(self) -> Imag_32: ...
    def __add__(self, other: Self | Amplitude_32 | Real_32 | Imag_32 | Product_32) -> Self: ...
    def __mul__(self, other: Amplitude_32 | Real_32 | Imag_32 | Product_32) -> Self: ...

Sum = Sum_64

def Scalar_64(name: str) -> Amplitude_64: ...
def Scalar_32(name: str) -> Amplitude_32: ...

Scalar = Scalar_64

def CScalar_64(name: str) -> Amplitude_64: ...
def CScalar_32(name: str) -> Amplitude_32: ...

CScalar = CScalar_64

def PCScalar_64(name: str) -> Amplitude_64: ...
def PCScalar_32(name: str) -> Amplitude_32: ...

PCScalar = PCScalar_64

def PiecewiseM_64(name: str, bins: int, range: tuple[float, float]) -> Amplitude_64: ...
def PiecewiseM_32(name: str, bins: int, range: tuple[float, float]) -> Amplitude_32: ...

PiecewiseM = PiecewiseM_64

class NormSqr_64:
    pass

class NormSqr_32:
    pass

NormSqr = NormSqr_64

class Model_64:
    cohsums: list[NormSqr_64]
    amplitudes: list[Amplitude_64]
    parameters: list[Parameter_64]
    free_parameters: list[Parameter_64]
    fixed_parameters: list[Parameter_64]
    bounds: list[tuple[float, float]]
    initial: list[float]
    n_free: int

    def __init__(
        self, terms: list[Amplitude_64 | Real_64 | Imag_64 | Product_64 | Sum_64]
    ) -> None: ...
    def get_parameter(self, amplitude_name: str, parameter_name: str) -> Parameter_64 | None: ...
    def print_parameters(self) -> None: ...
    def constrain(
        self, amplitude_1: str, parameter_1: str, amplitude_2: str, parameter_2: str
    ) -> None: ...
    def fix(self, amplitude_1: str, parameter_1: str, value: float) -> None: ...
    def free(self, amplitude_1: str, parameter_1: str) -> None: ...
    def set_bounds(
        self, amplitude_1: str, parameter_1: str, bounds: tuple[float, float]
    ) -> None: ...
    def set_initial(self, amplitude_1: str, parameter_1: str, initial: float) -> None: ...
    def activate(self, amplitude: str) -> None: ...
    def activate_all(self) -> None: ...
    def isolate(self, amplitudes: list[str]) -> None: ...
    def deactivate(self, amplitude: str) -> None: ...
    def deactivate_all(self) -> None: ...

class Model_32:
    cohsums: list[NormSqr_32]
    amplitudes: list[Amplitude_32]
    parameters: list[Parameter_32]
    free_parameters: list[Parameter_32]
    fixed_parameters: list[Parameter_32]
    bounds: list[tuple[float, float]]
    initial: list[float]
    n_free: int

    def __init__(
        self, terms: list[Amplitude_32 | Real_32 | Imag_32 | Product_32 | Sum_32]
    ) -> None: ...
    def get_parameter(self, amplitude_name: str, parameter_name: str) -> Parameter_32 | None: ...
    def print_parameters(self) -> None: ...
    def constrain(
        self, amplitude_1: str, parameter_1: str, amplitude_2: str, parameter_2: str
    ) -> None: ...
    def fix(self, amplitude_1: str, parameter_1: str, value: float) -> None: ...
    def free(self, amplitude_1: str, parameter_1: str) -> None: ...
    def set_bounds(
        self, amplitude_1: str, parameter_1: str, bounds: tuple[float, float]
    ) -> None: ...
    def set_initial(self, amplitude_1: str, parameter_1: str, initial: float) -> None: ...
    def activate(self, amplitude: str) -> None: ...
    def activate_all(self) -> None: ...
    def isolate(self, amplitudes: list[str]) -> None: ...
    def deactivate(self, amplitude: str) -> None: ...
    def deactivate_all(self) -> None: ...

Model = Model_64

class FourMomentum_64:
    e: float
    px: float
    py: float
    pz: float
    m: float
    m2: float

    def __init__(self, e: float, px: float, py: float, pz: float) -> None: ...
    def set_e(self, value: float) -> None: ...
    def set_px(self, value: float) -> None: ...
    def set_py(self, value: float) -> None: ...
    def set_pz(self, value: float) -> None: ...
    def boost_along(self, other: FourMomentum_64) -> float: ...
    def __add__(self, other: FourMomentum_64) -> FourMomentum_64: ...
    def __sub__(self, other: FourMomentum_64) -> FourMomentum_64: ...

class FourMomentum_32:
    e: float
    px: float
    py: float
    pz: float
    m: float
    m2: float

    def __init__(self, e: float, px: float, py: float, pz: float) -> None: ...
    def set_e(self, value: float) -> None: ...
    def set_px(self, value: float) -> None: ...
    def set_py(self, value: float) -> None: ...
    def set_pz(self, value: float) -> None: ...
    def boost_along(self, other: FourMomentum_32) -> float: ...
    def __add__(self, other: FourMomentum_32) -> FourMomentum_32: ...
    def __sub__(self, other: FourMomentum_32) -> FourMomentum_32: ...

FourMomentum = FourMomentum_64

class Event_64:
    index: int
    weight: float
    beam_p4: FourMomentum_64
    recoil_p4: FourMomentum_64
    daughter_p4s: list[FourMomentum_64]
    eps: list[float]

class Event_32:
    index: int
    weight: float
    beam_p4: FourMomentum_32
    recoil_p4: FourMomentum_32
    daughter_p4s: list[FourMomentum_32]
    eps: list[float]

Event = Event_64

class Dataset_64:
    events: list[Event_64]
    weights: list[float]

    def __getitem__(self, index: int) -> Event_64: ...
    def __len__(self) -> int: ...
    def split_m(
        self,
        range: tuple[float, float],  # noqa: A002
        bins: int,
        p1_indices: list[float] | None = None,
        p2_indices: list[float] | None = None,
    ) -> tuple[list[list[int]], list[int], list[int]]: ...
    def get_bootstrap_indices(self, seed: int) -> list[int]: ...
    @staticmethod
    def from_events(events: list[Event_64]) -> Dataset_64: ...
    @staticmethod
    def from_dict(data: dict[str, list[float | list[float]]]) -> Dataset_64: ...
    @staticmethod
    def from_parquet(path: str) -> Dataset_64: ...
    @staticmethod
    def from_parquet_eps_in_beam(path: str) -> Dataset_64: ...
    @staticmethod
    def from_parquet_with_eps(path: str, eps: list[float]) -> Dataset_64: ...
    @staticmethod
    def from_parquet_unpolarized(path: str) -> Dataset_64: ...
    @staticmethod
    def from_root(path: str) -> Dataset_64: ...

class Dataset_32:
    events: list[Event_32]
    weights: list[float]

    def __getitem__(self, index: int) -> Event_32: ...
    def __len__(self) -> int: ...
    def split_m(
        self,
        range: tuple[float, float],  # noqa: A002
        bins: int,
        p1_indices: list[float] | None = None,
        p2_indices: list[float] | None = None,
    ) -> tuple[list[list[int]], list[int], list[int]]: ...
    def get_bootstrap_indices(self, seed: int) -> list[int]: ...
    @staticmethod
    def from_events(events: list[Event_32]) -> Dataset_32: ...
    @staticmethod
    def from_dict(data: dict[str, list[float | list[float]]]) -> Dataset_32: ...
    @staticmethod
    def from_parquet(path: str) -> Dataset_32: ...
    @staticmethod
    def from_parquet_eps_in_beam(path: str) -> Dataset_32: ...
    @staticmethod
    def from_parquet_with_eps(path: str, eps: list[float]) -> Dataset_32: ...
    @staticmethod
    def from_parquet_unpolarized(path: str) -> Dataset_32: ...
    @staticmethod
    def from_root(path: str) -> Dataset_32: ...

Dataset = Dataset_64

@overload
def open(
    file_name: str | Path,
    tree_name: str | None = None,
    *,
    pol_in_beam: bool = False,
    f32: Literal[False] = False,
) -> Dataset_64: ...  # noqa: A001
@overload
def open(
    file_name: str | Path,
    tree_name: str | None = None,
    *,
    pol_in_beam: bool = False,
    f32: Literal[True],
) -> Dataset_32: ...  # noqa: A001
def open(
    file_name: str | Path,
    tree_name: str | None = None,
    *,
    pol_in_beam: bool = False,
    f32: bool = False,
) -> Dataset_64 | Dataset_32: ...  # noqa: A001
@overload
def minimizer(
    ell: ExtendedLogLikelihood_64 | ExtendedLogLikelihood_32,
    method: Literal['Minuit'],
    *args: Any,
    indices_data: list[int] | None = None,
    indices_mc: list[int] | None = None,
    num_threads: int = 1,
    minimizer_kwargs: dict[str, Any] | None = None,
) -> Minuit: ...

ScipyOptMethods = Literal[
    'Nelder-Mead',
    'Powell',
    'CG',
    'BFGS',
    'Newton-CG',
    'L-BFGS-B',
    'TNC',
    'COBYLA',
    'COBYQA',
    'SLSQP',
    'trust-constr',
    'dogleg',
    'trust-ncg',
    'trust-exact',
    'trust-krylov',
]

class ScipyCallable(Protocol):
    def __call__(self, x: ArrayLike, *args: Any) -> float: ...

class ScipyMinCallable(Protocol):
    def __call__(
        self, fun: ScipyCallable, x0: ArrayLike, args: tuple[Any], **kwargs_and_options: Any
    ) -> OptimizeResult: ...

@overload
def minimizer(
    ell: ExtendedLogLikelihood_64 | ExtendedLogLikelihood_32,
    method: ScipyOptMethods,
    *args: Any,
    indices_data: list[int] | None = None,
    indices_mc: list[int] | None = None,
    num_threads: int = 1,
    minimizer_kwargs: dict[str, Any] | None = None,
) -> Callable[[], OptimizeResult]: ...
@overload
def minimizer(
    ell: ExtendedLogLikelihood_64 | ExtendedLogLikelihood_32,
    method: ScipyMinCallable,
    *args: Any,
    indices_data: list[int] | None = None,
    indices_mc: list[int] | None = None,
    num_threads: int = 1,
    minimizer_kwargs: dict[str, Any] | None = None,
) -> Callable[[], OptimizeResult]: ...
def minimizer(
    ell: ExtendedLogLikelihood_64 | ExtendedLogLikelihood_32,
    method: Literal['Minuit'] | ScipyOptMethods | ScipyMinCallable | None = None,
    *args: Any,
    indices_data: list[int] | None = None,
    indices_mc: list[int] | None = None,
    num_threads: int = 1,
    minimizer_kwargs: dict[str, Any] | None = None,
) -> Minuit | Callable[[], OptimizeResult]: ...

class Manager_64:
    model: Model_64
    dataset: Dataset_64
    root: Amplitude_64
    amplitudes: list[Amplitude_64]
    parameters: list[Parameter_64]
    free_parameters: list[Parameter_64]
    fixed_parameters: list[Parameter_64]
    bounds: list[tuple[float, float]]
    initial: list[float]
    n_free: int

    def __init__(self, model: Model_64, dataset: Dataset_64) -> None: ...
    def __call__(
        self, parameters: list[float], *, indices: list[int] | None = None, parallel: bool = True
    ) -> list[float]: ...
    def evaluate(
        self, parameters: list[float], *, indices: list[int] | None = None, parallel: bool = True
    ) -> list[float]: ...
    def fix(self, amplitude_1: str, parameter_1: str, value: float) -> None: ...
    def free(self, amplitude_1: str, parameter_1: str) -> None: ...
    def set_bounds(
        self, amplitude_1: str, parameter_1: str, bounds: tuple[float, float]
    ) -> None: ...
    def set_initial(self, amplitude_1: str, parameter_1: str, initial: float) -> None: ...
    def activate(self, amplitude: str) -> None: ...
    def activate_all(self) -> None: ...
    def isolate(self, amplitudes: list[str]) -> None: ...
    def deactivate(self, amplitude: str) -> None: ...
    def deactivate_all(self) -> None: ...

class Manager_32:
    model: Model_32
    dataset: Dataset_32
    root: Amplitude_32
    amplitudes: list[Amplitude_32]
    parameters: list[Parameter_32]
    free_parameters: list[Parameter_32]
    fixed_parameters: list[Parameter_32]
    bounds: list[tuple[float, float]]
    initial: list[float]
    n_free: int

    def __init__(self, model: Model_32, dataset: Dataset_32) -> None: ...
    def __call__(
        self, parameters: list[float], *, indices: list[int] | None = None, parallel: bool = True
    ) -> list[float]: ...
    def evaluate(
        self, parameters: list[float], *, indices: list[int] | None = None, parallel: bool = True
    ) -> list[float]: ...
    def fix(self, amplitude_1: str, parameter_1: str, value: float) -> None: ...
    def free(self, amplitude_1: str, parameter_1: str) -> None: ...
    def set_bounds(
        self, amplitude_1: str, parameter_1: str, bounds: tuple[float, float]
    ) -> None: ...
    def set_initial(self, amplitude_1: str, parameter_1: str, initial: float) -> None: ...
    def activate(self, amplitude: str) -> None: ...
    def activate_all(self) -> None: ...
    def isolate(self, amplitudes: list[str]) -> None: ...
    def deactivate(self, amplitude: str) -> None: ...
    def deactivate_all(self) -> None: ...

Manager = Manager_64

class ExtendedLogLikelihood_64:
    data_manager: Manager_64
    mc_manager: Manager_64
    root: Amplitude_64
    amplitudes: list[Amplitude_64]
    parameters: list[Parameter_64]
    free_parameters: list[Parameter_64]
    fixed_parameters: list[Parameter_64]
    bounds: list[tuple[float, float]]
    initial: list[float]
    n_free: int

    def __init__(
        self, data_manager: Manager | Manager_64, mc_manager: Manager | Manager_64
    ) -> None: ...
    def __call__(
        self,
        parameters: list[float],
        *,
        indices_data: list[float] | None = None,
        indices_mc: list[float] | None = None,
        parallel: bool = True,
    ) -> float: ...
    def evaluate(
        self,
        parameters: list[float],
        indices_data: list[int] | None = None,
        indices_mc: list[int] | None = None,
        parallel: bool = True,
    ) -> float: ...
    def intensity(
        self,
        parameters: list[float],
        dataset_mc: Dataset_64,
        *,
        indices_data: list[int] | None = None,
        indices_mc: list[int] | None = None,
        parallel: bool = True,
    ) -> list[float]: ...
    def fix(self, amplitude_1: str, parameter_1: str, value: float) -> None: ...
    def free(self, amplitude_1: str, parameter_1: str) -> None: ...
    def set_bounds(
        self, amplitude_1: str, parameter_1: str, bounds: tuple[float, float]
    ) -> None: ...
    def set_initial(self, amplitude_1: str, parameter_1: str, initial: float) -> None: ...
    def activate(self, amplitude: str) -> None: ...
    def activate_all(self) -> None: ...
    def isolate(self, amplitudes: list[str]) -> None: ...
    def deactivate(self, amplitude: str) -> None: ...
    def deactivate_all(self) -> None: ...

class ExtendedLogLikelihood_32:
    data_manager: Manager_32
    mc_manager: Manager_32
    root: Amplitude_32
    amplitudes: list[Amplitude_32]
    parameters: list[Parameter_32]
    free_parameters: list[Parameter_32]
    fixed_parameters: list[Parameter_32]
    bounds: list[tuple[float, float]]
    initial: list[float]
    n_free: int

    def __init__(self, data_manager: Manager_32, mc_manager: Manager_32) -> None: ...
    def __call__(
        self,
        parameters: list[float],
        *,
        indices_data: list[float] | None = None,
        indices_mc: list[float] | None = None,
        parallel: bool = True,
    ) -> float: ...
    def evaluate(
        self,
        parameters: list[float],
        indices_data: list[int] | None = None,
        indices_mc: list[int] | None = None,
        parallel: bool = True,
    ) -> float: ...
    def intensity(
        self,
        parameters: list[float],
        dataset_mc: Dataset_32,
        *,
        indices_data: list[int] | None = None,
        indices_mc: list[int] | None = None,
        parallel: bool = True,
    ) -> list[float]: ...
    def fix(self, amplitude_1: str, parameter_1: str, value: float) -> None: ...
    def free(self, amplitude_1: str, parameter_1: str) -> None: ...
    def set_bounds(
        self, amplitude_1: str, parameter_1: str, bounds: tuple[float, float]
    ) -> None: ...
    def set_initial(self, amplitude_1: str, parameter_1: str, initial: float) -> None: ...
    def activate(self, amplitude: str) -> None: ...
    def activate_all(self) -> None: ...
    def isolate(self, amplitudes: list[str]) -> None: ...
    def deactivate(self, amplitude: str) -> None: ...
    def deactivate_all(self) -> None: ...

ExtendedLogLikelihood = ExtendedLogLikelihood_64

class NelderMead_64:
    def __init__(
        self,
        ell: ExtendedLogLikelihood_64,
        *,
        simplex_size: float = 1.0,
        reflection_coeff: float = 1.0,
        expansion_coeff: float = 2.0,
        outside_contraction_coeff: float = 0.5,
        inside_contraction_coeff: float = 0.5,
        shrink_coeff: float = 0.5,
        min_simplex_standard_deviation: float = 1e-8,
    ) -> None: ...
    @staticmethod
    def adaptive(
        ell, *, simplex_size=1.0, min_simplex_standard_deviation=1e-8
    ) -> NelderMead_64: ...
    def initialize(self) -> None: ...
    def step(self) -> None: ...
    def check_for_termination(self) -> bool: ...
    def minimize(self, steps: int) -> None: ...
    def best(self) -> tuple[list[float], float]: ...

class NelderMead_32:
    def __init__(
        self,
        ell: ExtendedLogLikelihood_32,
        *,
        simplex_size: float = 1.0,
        reflection_coeff: float = 1.0,
        expansion_coeff: float = 2.0,
        outside_contraction_coeff: float = 0.5,
        inside_contraction_coeff: float = 0.5,
        shrink_coeff: float = 0.5,
        min_simplex_standard_deviation: float = 1e-8,
    ) -> None: ...
    @staticmethod
    def adaptive(
        ell, *, simplex_size=1.0, min_simplex_standard_deviation=1e-8
    ) -> NelderMead_32: ...
    def initialize(self) -> None: ...
    def step(self) -> None: ...
    def check_for_termination(self) -> bool: ...
    def minimize(self, steps: int) -> None: ...
    def best(self) -> tuple[list[float], float]: ...

NelderMead = NelderMead_64
