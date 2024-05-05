from __future__ import annotations

from pathlib import Path

import numpy as np
import uproot

from ._rustitude import amplitude, dataset, four_momentum, gluex, manager
from .amplitude import cscalar, pcscalar, scalar
from .dataset import Dataset
from .manager import ExtendedLogLikelihood, Manager, MultiManager

__all__ = [
    'dataset',
    'manager',
    'amplitude',
    'four_momentum',
    'Dataset',
    'Manager',
    'MultiManager',
    'ExtendedLogLikelihood',
    'scalar',
    'cscalar',
    'pcscalar',
    'gluex',
    'open',
]


def __dir__():
    return __all__


# TODO: add a method to calculate EPS from a given polarization angle and amount
def open(file_name: str | Path, tree_name: str | None = None, *, pol_in_beam: bool = False) -> Dataset:  # noqa: A001
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
        tree_arrays['Px_Beam'] = np.zeros_like(tree_arrays['Px_Beam'])
        tree_arrays['Py_Beam'] = np.zeros_like(tree_arrays['Py_Beam'])
        tree_arrays['EPS'] = [np.array([ex, ey, 0] for ex, ey in zip(eps_x, eps_y))]
    return Dataset.from_dict(tree_arrays)
