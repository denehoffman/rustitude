Writing Custom Nodes in Python
==============================

rustitude allows you to extend its functionality by creating custom nodes using the ``rustitude.PyNode`` abstract base class. This feature enables you to implement your own amplitude calculations that seamlessly integrate with the rest of the rustitude framework.

Creating a Custom Node
----------------------

To create a custom node, you need to subclass ``rustitude.PyNode`` and implement three methods:

1. ``calculate``: Performs the actual calculation for each event.
2. ``precalculate``: (Optional) Performs any necessary preprocessing on the dataset.
3. ``parameters``: Returns a list of parameter names used by the node.

Here's an example of a custom node implementing a nonrelativistic Breit-Wigner function:

.. code-block:: python

    import rustitude as rt
    import numpy as np

    class PyBreitWigner(rt.PyNode):
        def calculate(self, parameters: list[float], event: rt.Event) -> complex:
            res_m = (event.daughter_p4s[0] + event.daughter_p4s[1]).m
            return np.sqrt(parameters[0] * parameters[2] / (np.power(res_m - parameters[1], 2) + np.power(parameters[2], 2)) / np.pi)

        def precalculate(self, dataset: rt.Dataset) -> None:
            pass

        def parameters(self) -> list[str]:
            return ["magnitude", "mass", "width"]

Let's break down each method:

- ``calculate``: This method takes a list of parameters and an ``Event`` object, and returns a complex number. In this case, it calculates the Breit-Wigner function using the invariant mass of the first two daughter particles. Note that the output of this should be a complex value, since we then automatically take the norm-squared to come up with the final result.

- ``precalculate``: This method is called once for the entire dataset before any calculations. In this example, no preprocessing is used, so it's left empty. The ``precalculate`` method is called when a ``Manager`` is created, and just like in a Rust struct, the values should be stored as vectors over the ``Dataset`` somewhere in the class itself.

- ``parameters``: This method returns a list of parameter names used by the node. Here, we have "magnitude", "mass", and "width".

These methods are decorated with ``@abstractmethod`` in the definition of ``rt.PyNode``, which is just an abstract base class that stipulates the template for child classes. This is useful because Rust will not be happy if any of the required functions don't exist!

Using the Custom Node
---------------------

Once you've defined your custom node, you can use it in your rustitude analysis:

.. code-block:: python

    mynode = rt.Amplitude("bw", rt.Node(PyBreitWigner())) # or rt.Node(PyBreitWigner()).into_amplitude("bw")
    mod = rt.Model([mynode.as_cohsum()])
    ds = rt.open("data_file.root")
    m = rt.Manager(mod, ds)
    res = m.evaluate([1.0, 1.300, 0.200])

In this example:

1. We create an ``Amplitude`` object using our custom ``PyBreitWigner`` node.
2. We create a ``Model`` with just this amplitude in a coherent sum.
3. We open a dataset using ``rt.open``.
4. We create a ``Manager`` object with our model and dataset.
5. Finally, we evaluate the model with specific parameter values.

Tips for Custom Nodes
---------------------

- Ensure your ``calculate`` method is as efficient as possible, as it will be called for each event in your dataset. Since this method is called by Rust through the GIL, models which use Python-side nodes cannot use the automatic parallelism of Rust. There may be workarounds to this, and for very complicated functions, it should be possible to use JIT compilation with tools like JAX or Numba natively on the function.
- Use the ``precalculate`` method for any computations that can be done once for the entire dataset, rather than repeating them for each event. Any calculation that doesn't depend on the free parameters of your fit should be done here. This particular amplitude has no such optimizations (except we could maybe calculate the resonance mass ahead of time), and users should be cautious about caching too much here, since these values get stored in memory and can accumulate quicky for large datasets.
- Make sure the number and order of parameters in your ``calculate`` method match the list returned by your `parameters` method.
- When using ``numpy`` or other Python libraries, be aware of any performance implications compared to native Rust implementations. As stated, it will almost always be faster to write a Rust amplitude, but unless such an amplitude is directly added to the ``rustitude`` crate, it cannot be used with the Python API. This is an issue with PyO3, but I'm working on some way to get around it. Suggestions are appreciated.

By creating custom nodes, you can extend rustitude's capabilities to fit your specific analysis needs while still benefiting from some of the efficiency of the Rust core.
