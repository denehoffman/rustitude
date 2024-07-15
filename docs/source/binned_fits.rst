Binned Fits
===========

It is often useful to bin particle data by mass or Mandelstam-t to fit model-independent amplitudes separately in each bin. rustitude offers a mechanism to do this using event indices.

Example
-------

Suppose we have some data, and we want to fit polarized spherical harmonics to these data without providing a model for the mass. We can do this by binning by mass and fitting the amplitudes separately in each bin.

.. code-block:: python

   import rustitude as rt
   import numpy as np
   import matplotlib.pyplot as plt

First, we collect the data. Let's split these data into bins of mass, let's say 40 bins in a range of 1-2 GeV.

.. code-block:: python

   ds_data = rt.open("data.root")
   ds_accmc = rt.open("accmc.root")

   ds_data_split, _, _ = ds_data.split_m(bins=40, range=(1, 2))
   ds_accmc_split, _, _ = ds_accmc.split_m(bins=40, range=(1, 2))

These are just lists of lists of indices! We ignore the last two outputs, as they represent the under/overflow bins.

Let's fit an S0+ wave, a P-1+ wave, and a D2- wave. We also need to set up some free parameters, since the Zlm amplitudes are fixed with respect to the data. It's important to make two of them scalars rather than complex scalars, as coherent sums are invariant up to an overall phase.

.. code-block:: python


   z00p = rt.gluex.harmonics.Zlm("Z00+", l=0, m=0, reflectivity="+")
   z1m1p = rt.gluex.harmonics.Zlm("Z1-1+", l=1, m=-1, reflectivity="+")
   z22m = rt.gluex.harmonics.Zlm("Z22-", l=2, m=2, reflectivity="-")

   s0p = rt.Scalar("S0+")
   pm1p = rt.CScalar("P-1+")
   d2m = rt.Scalar("D2-")


Next, we set up our model:

.. code-block:: python

   positive_real_sum = s0p * z00p.real() + pm1p * z1m1p.real()
   positive_imag_sum = s0p * z00p.imag() + pm1p * z1m1p.imag()
   # as_cohsum() because it's a single term:
   negative_real_sum = (d2m * z22m.real()).as_cohsum()
   negative_imag_sum = (d2m * z22m.imag()).as_cohsum()

   model = rt.Model([positive_real_sum, positive_imag_sum, negative_real_sum, negative_imag_sum])
   nll = rt.ExtendedLogLikelihood(rt.Manager(model, ds_data), rt.Manager(model, ds_accmc))

Let's give our parameters some reasonable bounds (this is just a guess).

.. code-block:: python

   for parameter in nll.parameters:
       nll.set_bounds(parameter.amplitude, parameter.name, (-1000, 1000))

We can now use the minimizer interface to create some fit objects. Let's use Minuit via the iminuit package!

.. code-block:: python

   ms = [rt.minimizer(nll,
                      method="Minuit",
                      indices_data=indices_data,
                      indices_mc=indices_mc)
         for indices_data, indices_mc in zip(ds_data_split, ds_accmc_split)]
   
Run the fits (with the Migrad algorithm)!

.. code-block:: python

   for ibin, m in enumerate(ms):
       print(f"Fitting bin {i}")
       m.migrad()

Now we collect the results. We can sum the intensities for each event in each bin separately, and use the "isolate" method to select a subset of waves to evaluate.

.. code-block:: python

   intensity_tot = [sum(nll.intensity(list(m.values),
                                      ds_accmc,
                                      indices_data=indices_data,
                                      indices_mc=indices_mc))
                    for m, indices_data, indices_mc in zip(ms, ds_data_split, ds_accmc_split)]

   nll.isolate(["Z00+", "S0+"])
   intensity_s0p = [sum(nll.intensity(list(m.values),
                                      ds_accmc,
                                      indices_data=indices_data,
                                      indices_mc=indices_mc))
                    for m, indices_data, indices_mc in zip(ms, ds_data_split, ds_accmc_split)]

   nll.isolate(["Z1-1+", "P-1+"])
   intensity_pm1p = [sum(nll.intensity(list(m.values),
                                       ds_accmc,
                                       indices_data=indices_data,
                                       indices_mc=indices_mc))
                     for m, indices_data, indices_mc in zip(ms, ds_data_split, ds_accmc_split)]

   nll.isolate(["Z22-", "D2-"])
   intensity_d2m = [sum(nll.intensity(list(m.values),
                                      ds_accmc,
                                      indices_data=indices_data,
                                      indices_mc=indices_mc))
                    for m, indices_data, indices_mc in zip(ms, ds_data_split, ds_accmc_split)]

Finally, we can plot the results:

.. code-block:: python

   m_data = [(e.daughter_p4s[0] + e.daughter_p4s[1]).m for e in ds_data.events]
   bin_edges = np.histogram_bin_edges([], bins=40, range=(1, 2))
   plt.hist(m_data, weights=ds_data.weights, bins=40, range=(1, 2), histtype='step', label="Data")
   plt.stairs(intensity_tot, bin_edges, label="Fit Total")
   plt.stairs(intensity_s0p, bin_edges, label="$S_{0}^{(+)}")
   plt.stairs(intensity_pm1p, bin_edges, label="$P_{-1}^{(+)}")
   plt.stairs(intensity_d2m, bin_edges, label="$D_{2}^{(-)}")
   plt.legend()
   plt.show()

From here, we'd usually go through the process of modifying the model to produce the most sensible fit, and to really make a complete study, we might generate some sets of indices that can be thought of as bootstrapped data. If we were fitting all of the data at once, we could use the :code:`Dataset.get_bootstrap_indices(self, seed: int)` method to generate such a set, but here we might favor using native Python methods to resample the binned indices we generated above. Note that sorting said indices will probably lead to better performance. Bootstrapped indices can be used the exact same way as the indices were used above, and after collecting intensities from each bootstrap fit, we just need to find the standard deviation to put error bars on the plot we just made, but for now that will be an exercise for the user.
