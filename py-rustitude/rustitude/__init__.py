from __future__ import annotations

from pathlib import Path

from typing import Callable, Literal, Any, Protocol
import numpy as np
from numpy.typing import ArrayLike
import uproot
from iminuit import Minuit
from scipy.optimize import OptimizeResult
import scipy.optimize as opt
from uproot.behaviors.TBranch import HasBranches

from ._rustitude import (
    amplitude,
    dataset,
    four_momentum,
    gluex,
    manager,
    __version__,
    __rustitude_precision__,
)
from .amplitude import (
    Scalar64,
    Scalar32,
    CScalar64,
    CScalar32,
    PCScalar64,
    PCScalar32,
    PiecewiseM64,
    PiecewiseM32,
    Parameter64,
    Parameter32,
    Model64,
    Model32,
    Amplitude64,
    Amplitude32,
    Real64,
    Real32,
    Imag64,
    Imag32,
    Product64,
    Product32,
    CohSum64,
    CohSum32,
    Node64,
    Node32,
)
from .dataset import Event64, Event32, Dataset64, Dataset32
from .manager import (
    ExtendedLogLikelihood64,
    ExtendedLogLikelihood32,
    Manager64,
    Manager32,
    NelderMead64,
    NelderMead32,
)

from abc import ABCMeta, abstractmethod

Scalar = Scalar64
CScalar = CScalar64
PCScalar = PCScalar64
PiecewiseM = PiecewiseM64
Parameter = Parameter64
Model = Model64
Amplitude = Amplitude64
Real = Real64
Imag = Imag64
Product = Product64
CohSum = CohSum64
Node = Node64
Event = Event64
Dataset = Dataset64
ExtendedLogLikelihood = ExtendedLogLikelihood64
Manager = Manager64
NelderMead = NelderMead64

__version__: str = __version__
__rustitude_precision__: str = __rustitude_precision__

__all__ = [
    '__version__',
    '__rustitude_precision__',
    'dataset',
    'manager',
    'amplitude',
    'four_momentum',
    'Event',
    'Event64',
    'Event32',
    'Dataset',
    'Dataset64',
    'Dataset32',
    'Manager',
    'Manager64',
    'Manager32',
    'ExtendedLogLikelihood',
    'ExtendedLogLikelihood64',
    'ExtendedLogLikelihood32',
    'Amplitude',
    'Amplitude64',
    'Amplitude32',
    'Real',
    'Real64',
    'Real32',
    'Imag',
    'Imag64',
    'Imag32',
    'Product',
    'Product64',
    'Product32',
    'CohSum',
    'CohSum64',
    'CohSum32',
    'Scalar',
    'Scalar64',
    'Scalar32',
    'CScalar',
    'CScalar64',
    'CScalar32',
    'PCScalar',
    'PCScalar64',
    'PCScalar32',
    'PiecewiseM',
    'PiecewiseM64',
    'PiecewiseM32',
    'Parameter',
    'Parameter64',
    'Parameter32',
    'Model',
    'Model64',
    'Model32',
    'NelderMead',
    'NelderMead64',
    'NelderMead32',
    'Node',
    'Node64',
    'Node32',
    'PyNode',
    'PyNode64',
    'PyNode32',
    'gluex',
    'open',
    'minimizer',
]


def __dir__():
    return __all__


# TODO: add a method to calculate EPS from a given polarization angle and amount
def open(
    file_name: str | Path,
    tree_name: str | None = None,
    *,
    pol_in_beam: bool = False,
    f32: bool = False,
) -> Dataset64 | Dataset32:  # noqa: A001
    filepath = (file_name if isinstance(file_name, Path) else Path(file_name)).resolve()
    tfile = uproot.open(filepath)
    ttree = tfile[tree_name] if tree_name else tfile.get(tfile.keys()[0])
    if not isinstance(ttree, HasBranches):
        raise Exception('TTree has no branches!')
    requested_branches = [
        'E_Beam',
        'Px_Beam',
        'Py_Beam',
        'Pz_Beam',
        'Weight',
        'EPS',
        'E_FinalState',
        'Px_FinalState',
        'Py_FinalState',
        'Pz_FinalState',
    ]
    available_branches = [b for b in requested_branches if b in ttree]
    tree_arrays = ttree.arrays(available_branches, library='np')
    if pol_in_beam:
        eps_x = tree_arrays['Px_Beam']
        eps_y = tree_arrays['Py_Beam']
        eps_z = tree_arrays['Pz_Beam']
        tree_arrays['Px_Beam'] = np.zeros_like(tree_arrays['Px_Beam'])
        tree_arrays['Py_Beam'] = np.zeros_like(tree_arrays['Py_Beam'])
        tree_arrays['Pz_Beam'] = tree_arrays['E_Beam']
        tree_arrays['EPS'] = [np.array([ex, ey, ez]) for ex, ey, ez in zip(eps_x, eps_y, eps_z)]
    if f32:
        return Dataset32.from_dict(tree_arrays)
    else:
        return Dataset64.from_dict(tree_arrays)


class PyNode64(metaclass=ABCMeta):
    @abstractmethod
    def precalculate(self, dataset: Dataset) -> None:
        pass

    @abstractmethod
    def calculate(self, parameters: list[float], event: Event) -> complex:
        pass

    @abstractmethod
    def parameters(self) -> list[str]:
        pass


PyNode = PyNode64


class PyNode32(metaclass=ABCMeta):
    @abstractmethod
    def precalculate(self, dataset: Dataset32) -> None:
        pass

    @abstractmethod
    def calculate(self, parameters: list[float], event: Event32) -> complex:
        pass

    @abstractmethod
    def parameters(self) -> list[str]:
        pass


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


def minimizer(
    ell: ExtendedLogLikelihood64 | ExtendedLogLikelihood32,
    method: Literal['Minuit'] | ScipyOptMethods | ScipyMinCallable | None = None,
    *args: Any,
    indices_data: list[int] | None = None,
    indices_mc: list[int] | None = None,
    parallel: bool = True,
    minimizer_kwargs: dict[str, Any] | None = None,
) -> Minuit | Callable[[], OptimizeResult]:
    bounds = []
    unbounded = True
    for bound in ell.bounds:
        lb = None if bound[0] == -np.inf else bound[0]
        ub = None if bound[1] == np.inf else bound[1]
        if lb or ub:
            unbounded = False
        bounds.append((lb, ub))
    if callable(method) or method != 'Minuit' or method is None:
        if minimizer_kwargs is None:
            minimizer_kwargs = {}
        if unbounded:
            bounds = None

        def fcn_scipy(x: ArrayLike, *args: Any):
            return ell(x, indices_data=indices_data, indices_mc=indices_mc, parallel=parallel)

        def fit() -> OptimizeResult:
            return opt.minimize(
                fcn_scipy, ell.initial, args, method, bounds=bounds, **minimizer_kwargs
            )

        return fit

    else:

        def fcn_minuit(*args: float):
            # error def is correct because of implicit multiplication by 2 in ELL
            return ell(
                list(args),
                indices_data=indices_data,
                indices_mc=indices_mc,
                parallel=parallel,
            )

        minuit_par_names = [f'{p.amplitude} - {p.name}' for p in ell.parameters if p.free]
        m = Minuit(fcn_minuit, *ell.initial, name=minuit_par_names)
        for par_name, bound in zip(minuit_par_names, bounds):
            m.limits[par_name] = bound
        return m
