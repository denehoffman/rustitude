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
)
from .amplitude import (
    Scalar_64,
    Scalar_32,
    CScalar_64,
    CScalar_32,
    PCScalar_64,
    PCScalar_32,
    PiecewiseM_64,
    PiecewiseM_32,
    Parameter_64,
    Parameter_32,
    Model_64,
    Model_32,
    Amplitude_64,
    Amplitude_32,
    Real_64,
    Real_32,
    Imag_64,
    Imag_32,
    Product_64,
    Product_32,
    Sum_64,
    Sum_32,
    NormSqr_64,
    NormSqr_32,
    Node_64,
    Node_32,
)
from .dataset import Event_64, Event_32, Dataset_64, Dataset_32
from .manager import (
    ExtendedLogLikelihood_64,
    ExtendedLogLikelihood_32,
    Manager_64,
    Manager_32,
    NelderMead_64,
    NelderMead_32,
)

from abc import ABCMeta, abstractmethod

Scalar = Scalar_64
CScalar = CScalar_64
PCScalar = PCScalar_64
PiecewiseM = PiecewiseM_64
Parameter = Parameter_64
Model = Model_64
Amplitude = Amplitude_64
Real = Real_64
Imag = Imag_64
Product = Product_64
Sum = Sum_64
NormSqr = NormSqr_64
Node = Node_64
Event = Event_64
Dataset = Dataset_64
ExtendedLogLikelihood = ExtendedLogLikelihood_64
Manager = Manager_64
NelderMead = NelderMead_64

__version__: str = __version__

__all__ = [
    '__version__',
    'dataset',
    'manager',
    'amplitude',
    'four_momentum',
    'Event',
    'Event_64',
    'Event_32',
    'Dataset',
    'Dataset_64',
    'Dataset_32',
    'Manager',
    'Manager_64',
    'Manager_32',
    'ExtendedLogLikelihood',
    'ExtendedLogLikelihood_64',
    'ExtendedLogLikelihood_32',
    'Amplitude',
    'Amplitude_64',
    'Amplitude_32',
    'Real',
    'Real_64',
    'Real_32',
    'Imag',
    'Imag_64',
    'Imag_32',
    'Product',
    'Product_64',
    'Product_32',
    'Sum',
    'Sum_64',
    'Sum_32',
    'NormSqr',
    'NormSqr_64',
    'NormSqr_32',
    'Scalar',
    'Scalar_64',
    'Scalar_32',
    'CScalar',
    'CScalar_64',
    'CScalar_32',
    'PCScalar',
    'PCScalar_64',
    'PCScalar_32',
    'PiecewiseM',
    'PiecewiseM_64',
    'PiecewiseM_32',
    'Parameter',
    'Parameter_64',
    'Parameter_32',
    'Model',
    'Model_64',
    'Model_32',
    'NelderMead',
    'NelderMead_64',
    'NelderMead_32',
    'Node',
    'Node_64',
    'Node_32',
    'PyNode',
    'PyNode_64',
    'PyNode_32',
    'gluex',
    'open',
    'minimizer',
]


def __dir__():
    return __all__


def open(
    file_name: str | Path,
    tree_name: str | None = None,
    *,
    pol_in_beam: bool = False,
    eps: tuple[float, float, float] | None = None,
    f32: bool = False,
) -> Dataset_64 | Dataset_32:  # noqa: A001
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
        tree_arrays['EPS'] = np.array([
            np.array([ex, ey, ez]) for ex, ey, ez in zip(eps_x, eps_y, eps_z)
        ])
    elif eps is not None:
        tree_arrays['EPS'] = np.array([np.array(eps) for _ in range(len(tree_arrays['Weight']))])
    if f32:
        return Dataset_32.from_dict(tree_arrays)
    else:
        return Dataset_64.from_dict(tree_arrays)


class PyNode_64(metaclass=ABCMeta):
    @abstractmethod
    def precalculate(self, dataset: Dataset_64) -> None:
        pass

    @abstractmethod
    def calculate(self, parameters: list[float], event: Event_64) -> complex:
        pass

    @abstractmethod
    def parameters(self) -> list[str]:
        pass


PyNode = PyNode_64


class PyNode_32(metaclass=ABCMeta):
    @abstractmethod
    def precalculate(self, dataset: Dataset_32) -> None:
        pass

    @abstractmethod
    def calculate(self, parameters: list[float], event: Event_32) -> complex:
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
    ell: ExtendedLogLikelihood_64 | ExtendedLogLikelihood_32,
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

        def fcn_scipy(x: ArrayLike, *_args: Any):
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
