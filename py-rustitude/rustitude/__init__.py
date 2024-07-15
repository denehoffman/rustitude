from __future__ import annotations

from pathlib import Path

from typing import Callable, Literal, Any, Protocol
import numpy as np
from numpy.typing import ArrayLike
import uproot
from iminuit import Minuit
from scipy.optimize import OptimizeResult
import scipy.optimize as opt

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
    CScalar,
    PCScalar,
    Scalar,
    PiecewiseM,
    Parameter,
    Model,
    Amplitude,
    Real,
    Imag,
    Product,
    CohSum,
    Node,
)
from .dataset import Event, Dataset
from .manager import ExtendedLogLikelihood, Manager

from abc import ABCMeta, abstractmethod

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
    'Dataset',
    'Manager',
    'ExtendedLogLikelihood',
    'Amplitude',
    'Real',
    'Imag',
    'Product',
    'CohSum',
    'Scalar',
    'CScalar',
    'PCScalar',
    'PiecewiseM',
    'Parameter',
    'Model',
    'gluex',
    'Node',
    'PyNode',
    'open',
    'minimizer',
]


def __dir__():
    return __all__


# TODO: add a method to calculate EPS from a given polarization angle and amount
def open(
    file_name: str | Path, tree_name: str | None = None, *, pol_in_beam: bool = False
) -> Dataset:  # noqa: A001
    filepath = (file_name if isinstance(file_name, Path) else Path(file_name)).resolve()
    tfile = uproot.open(filepath)
    ttree = tfile[tree_name] if tree_name else tfile.get(tfile.keys()[0])
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
    return Dataset.from_dict(tree_arrays)


class PyNode(metaclass=ABCMeta):
    @abstractmethod
    def precalculate(self, dataset: Dataset) -> None:
        pass

    @abstractmethod
    def calculate(self, parameters: list[float], event: Event) -> complex:
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
    ell: ExtendedLogLikelihood,
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
