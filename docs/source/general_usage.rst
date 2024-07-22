General Usage
=============

rustitude is a powerful library for performing amplitude analyses, particularly in particle physics. This guide will walk you through the basic concepts and usage patterns, culminating in a complex example that demonstrates many of rustitude's features.

Basic Concepts
--------------

Nodes and Amplitudes
^^^^^^^^^^^^^^^^^^^^

The core building blocks of rustitude are Nodes and Amplitudes:

- **Nodes** are the fundamental calculation units, typically implemented in Rust for efficiency.
- **Amplitudes** are created from Nodes by giving them a name. They are the primary objects you'll work with in your analyses.

rustitude provides several utility amplitudes:

- ``rt.Scalar``: Represents a single free parameter.
- ``rt.CScalar``: Represents two free parameters as a complex number in Cartesian form.
- ``rt.PCScalar``: Represents two free parameters as a complex number in polar form.
- ``rt.PiecewiseM``: Describes a piecewise amplitude over the combined invariant mass of all daughter particles.

Building Models
^^^^^^^^^^^^^^^

Models in rustitude are built using operations on Amplitudes:

- Use ``.real()`` and ``.imag()`` methods to get the real or imaginary parts of an Amplitude.
- Multiply Amplitudes using the ``*`` operator.
- Add Amplitudes using the ``+`` operator (creates coherent sums).
- Use ``.as_cohsum()`` to turn a single Amplitude into a coherent sum.
- Create incoherent sums by listing Amplitudes as separate entries in the Model's cohsum list.

Working with Datasets
^^^^^^^^^^^^^^^^^^^^^

Use the ``rt.open()`` method to create datasets from .root files. This method uses uproot to load ROOT files and convert them to Dataset objects. The ROOT files should have the following branches:

- ``E_Beam``, ``Px_Beam``, ``Py_Beam``, ``Pz_Beam``: Beam four-momentum components (floats)
- ``Weight``: Event weight (float)
- ``E_FinalState``, ``Px_FinalState``, ``Py_FinalState``, ``Pz_FinalState``: Arrays of final state particle four-momenta (floats)
- ``EPS``: Beam polarization vector (optional, array of three floats)

Managers and Models
^^^^^^^^^^^^^^^^^^^

Managers combine datasets and models, providing the main interface for interactions:

.. code-block:: python

    class Manager:
        # ... (attributes omitted for brevity)

        def __init__(self, model: Model, dataset: Dataset) -> None: ...
        def __call__(self, parameters: list[float]) -> list[float]: ...
        def evaluate(self, parameters: list[float]) -> list[float]: ...
        def par_evaluate(self, parameters: list[float]) -> list[float]: ...
        def fix(self, amplitude_1: str, parameter_1: str, value: float) -> None: ...
        def free(self, amplitude_1: str, parameter_1: str) -> None: ...
        def set_bounds(self, amplitude_1: str, parameter_1: str, bounds: tuple[float, float]) -> None: ...
        def set_initial(self, amplitude_1: str, parameter_1: str, initial: float) -> None: ...
        def activate(self, amplitude: str) -> None: ...
        def activate_all(self) -> None: ...
        def deactivate(self, amplitude: str) -> None: ...
        def deactivate_all(self) -> None: ...

Models have similar methods for managing parameters and amplitudes:

.. code-block:: python

    class Model:
        cohsums: list[NormSqr]
        amplitudes: list[Amplitude]
        parameters: list[Parameter]
        fixed_parameters: list[Parameter]
        free_parameters: list[Parameter]
        bounds: list[tuple[float, float]]
        initial: list[float]
        n_free: int

        def __init__(self, cohsums: list[Amplitude | Real | Imag | Product | Sum]) -> None: ...
        def get_parameter(self, amplitude_name: str, parameter_name: str) -> Parameter | None: ...
        def print_parameters(self) -> None: ...
        def constrain(self, amplitude_1: str, parameter_1: str, amplitude_2: str, parameter_2: str) -> None: ...
        # ... (other methods similar to Manager)

Extended Log-Likelihood
^^^^^^^^^^^^^^^^^^^^^^^

The ``ExtendedLogLikelihood`` class manages two Managers, simplifying parameter management across data and Monte Carlo samples. It computes results based on the following formula:

The extended log-likelihood function is given by:

.. math::

   -2 \left( \sum_{e \in D_\text{data}} e_w \ln(\mathcal{L}(\vec{p}, e)) - \frac{N_\text{data}}{N_\text{MC}} \sum_{e \in D_\text{MC}} e_w \mathcal{L}(\vec{p}, e) \right)

Where :math:`D_\text{data}` is the data sample, :math:`D_\text{MC}` is the Monte Carlo sample, :math:`e_w` is the event weight, :math:`\mathcal{L}(\vec{p}, e)` is the likelihood for a given set of parameters :math:`\vec{p}` and event :math:`e`, and :math:`N_\text{data}` and :math:`N_\text{MC}` are the number of events in the data and Monte Carlo samples respectively.

Complex Example
---------------

Here's a complex example that demonstrates many of rustitude's features:

.. code-block:: python

    import rustitude as rt
    from rustitude import gluex
    import numpy as np
    import scipy
    import matplotlib.pyplot as plt

    # Define resonances and harmonics
    f0p = gluex.resonances.KMatrixF0("f0+", channel=2)
    f0n = gluex.resonances.KMatrixF0("f0-", channel=2)
    f2 = gluex.resonances.KMatrixF2("f2", channel=2)
    a0p = gluex.resonances.KMatrixA0("a0+", channel=1)
    a0n = gluex.resonances.KMatrixA0("a0-", channel=1)
    a2 = gluex.resonances.KMatrixA2("a2", channel=1)
    s0p = gluex.harmonics.Zlm("s0+", 0, 0, "+")
    s0n = gluex.harmonics.Zlm("s0-", 0, 0, "-")
    d2p = gluex.harmonics.Zlm("d2+", 2, 2, "+")

    # Build the model
    pos_re = (f0p + a0p) * s0p.real() + (f2 + a2) * d2p.real()
    pos_im = (f0p + a0p) * s0p.imag() + (f2 + a2) * d2p.imag()
    neg_re = (f0n + a0n) * s0n.real()
    neg_im = (f0n + a0n) * s0n.imag()
    model = rt.Model([pos_re, pos_im, neg_re, neg_im])

    # Load data files
    ds = rt.open("data_pol.root")
    ds_mc = rt.open("accmc_pol.root")

    # Create managers
    data_manager = rt.Manager(model, ds)
    mc_manager = rt.Manager(model, ds_mc)

    # Set up negative log-likelihood
    nll = rt.ExtendedLogLikelihood(data_manager, mc_manager)

    # Set bounds and initial values
    for parameter in nll.parameters:
        # for demonstration only, in the fit we start at a random position:
        nll.set_initial(parameter.amplitude, parameter.name, 100.0) 
        # these bounds, however, are used by the fit!
        nll.set_bounds(parameter.amplitude, parameter.name, (-1000.0, 1000.0))

    # Fix some parameters
    # Note that the fix method sets a flag which fixes a paramater and any parameters
    # parameters which might be constrained to be equal to it. It overrides the "initial"
    # value, so setting the initial value of a fixed parameter will change the value it is
    # fixed to!
    nll.fix("f0+", "f0_500 re", 0.0)
    nll.fix("f0+", "f0_500 im", 0.0)
    nll.fix("f0+", "f0_980 im", 0.0)
    nll.fix("f0-", "f0_500 re", 0.0)
    nll.fix("f0-", "f0_500 im", 0.0)
    nll.fix("f0-", "f0_980 im", 0.0)


    # Perform optimization
    
    rng = np.random.default_rng()

    for parameter in nll.parameters:
        if parameter.free:
            nll.set_initial(parameter.amplitude, parameter.name, rng.random() * 100)

    # With the default 'method=None' argument, this will use scipy.optimize.minimize's default algorithm:
    m = rt.minimizer(nll)
    res = m()

    # Process results
    print(f"Fit Result:\n{res}")
    fit_pars = res.x
    masses = [(event.daughter_p4s[0] + event.daughter_p4s[1]).m for event in ds.events]
    fit_weights_mc = nll.intensity(fit_pars, ds_mc)
    masses_mc = [(event.daughter_p4s[0] + event.daughter_p4s[1]).m for event in ds_mc.events]

    # Plot results
    plt.hist(masses, bins=40, range=(1.0, 2.0), weights=ds.weights, label="data", histtype='step')
    plt.hist(masses_mc, bins=40, range=(1.0, 2.0), weights=np.array(fit_weights_mc), label="fit", histtype='step')
    plt.legend()
    plt.savefig("result.png")

Automatic parallelism over the CPU can be disabled via function calls which support it (for example, ``nll([10.0] * mod.n_free, parallel=False)`` would run without parallel processing), and the number of CPUs used can be controlled via the ``RAYON_NUM_THREADS`` environment variable, which can be set before the code is run or modified inside the code (for example, ``os.environ['RAYON_NUM_THREADS] = '5'`` would ensure only five threads are used past that point in the code). By default, an unset value or the value of ``'0'`` will use all available cores.
