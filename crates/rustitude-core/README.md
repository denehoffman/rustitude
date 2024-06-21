<p align="center">
  <img
    width="800"
    src="https://github.com/denehoffman/rustitude/blob/main/media/logo.png"
  />
</p>
<p align="center">
    <h1 align="center">Demystifying Amplitude Analysis</h1>
</p>

<p align="center">
  <a href="https://github.com/denehoffman/rustitude/releases" alt="Releases">
    <img alt="GitHub Release" src="https://img.shields.io/github/v/release/denehoffman/rustitude?style=for-the-badge&logo=github"></a>
  <a href="https://github.com/denehoffman/rustitude/commits/main/" alt="Lastest Commits">
    <img alt="GitHub last commit" src="https://img.shields.io/github/last-commit/denehoffman/rustitude?style=for-the-badge&logo=github"></a>
  <a href="https://github.com/denehoffman/rustitude/actions" alt="Build Status">
    <img alt="GitHub Actions Workflow Status" src="https://img.shields.io/github/actions/workflow/status/denehoffman/rustitude/rust.yml?style=for-the-badge&logo=github"></a>
  <a href="LICENSE" alt="License">
    <img alt="GitHub License" src="https://img.shields.io/github/license/denehoffman/rustitude?style=for-the-badge"></a>
  <a href="https://crates.io/crates/rustitude" alt="Rustitude on crates.io">
    <img alt="Crates.io Version" src="https://img.shields.io/crates/v/rustitude?style=for-the-badge&logo=rust&logoColor=red&color=red"></a>
  <a href="https://docs.rs/rustitude" alt="Rustitude documentation on docs.rs">
    <img alt="docs.rs" src="https://img.shields.io/docsrs/rustitude?style=for-the-badge&logo=rust&logoColor=red"></a>
  <a href="https://app.codecov.io/github/denehoffman/rustitude/tree/main/" alt="Codecov coverage report">
    <img alt="Codecov" src="https://img.shields.io/codecov/c/github/denehoffman/rustitude?style=for-the-badge&logo=codecov"></a>
  <a href="https://pypi.org/project/rustitude/" alt="View project on PyPI">
  <img alt="PyPI - Version" src="https://img.shields.io/pypi/v/rustitude?style=for-the-badge&logo=python&logoColor=yellow&labelColor=blue"></a>
  <a href="https://rustitude.readthedocs.io/en/latest/", alt="Rustitude documentation on readthedocs.io">
    <img alt="Read the Docs" src="https://img.shields.io/readthedocs/rustitude?style=for-the-badge&logo=python&logoColor=yellow&labelColor=blue"></a>
</p>

### Table of Contents

- [Overview](#Overview)
- [Installation](#Installation)
- [Usage](#Usage)
- [Implementing an Amplitude](#Implementing-an-Amplitude)
  - [Python Bindings](#Python-Bindings)
- [TODOs](#TODOs)

# Overview

This crate contains all of the core mechanics

- `rustitude` will automatically parallelize amplitudes over the events in a dataset. There's no reason for a developer to ever write parallelized code themselves.
- Implementing [`Node`](https://docs.rs/rustitude-core/latest/rustitude_core/amplitude/trait.Node.html) on a struct is all that is needed to use it as an amplitude. This means developers need only write two to three total methods to describe the entire functionality of their amplitude, and one of these just gives the names and order of the amplitude's input parameters.
- A major goal of `rustitude` was to increase processing speed by sacrificing memory. This is done by precalculating parts of amplitudes which don't change when the free parameter inputs change. The simplest example of this is the `Ylm` amplitude (spherical harmonic), which can be entirely precalculated given the value of `l` and `m`. To calculate this amplitude at evaluation time, `rustitude` just needs to look up a value in an array!

# Installation

Cargo provides the usual command for including this crate in a project:

```shell
cargo add rustitude-core
```

However, this project is best-used with some pre-made amplitudes (see [`rustitude-gluex`](https://github.com/denehoffman/rustitude/tree/main/crates/rustitude-gluex/), and these come bundled with the [`rustitude`](https://github.com/denehoffman/rustitude/) meta-crate, which also re-exports this crate's prelude. This can be installed via the same command:

```shell
cargo add rustitude
```

# Implementing an Amplitude

While typical usage might be to use pre-made amplitudes in various combinations, it is important to know how to design an amplitude which will work seamlessly with this crate. Let's write down the `rustitude` version of the [OmegaDalitz](https://github.com/JeffersonLab/halld_sim/blob/6544f01ac1514b0b9a53ad241cf2e8a63e1d3dfa/src/libraries/AMPTOOLS_AMPS/OmegaDalitz.cc) amplitude:

```rust
use rayon::prelude::*;
use rustitude_core::prelude::*;

#[derive(Default)]
pub struct OmegaDalitz {
    dalitz_z: Vec<f64>,
    dalitz_sin3theta: Vec<f64>,
    lambda: Vec<f64>,
}

impl Node for OmegaDalitz {
    fn precalculate(&mut self, dataset: &Dataset) -> Result<(), RustitudeError> {
        (self.dalitz_z, (self.dalitz_sin3theta, self.lambda)) = dataset
            .events
            .read()
            .par_iter()
            .map(|event| {
                let pi0 = event.daughter_p4s[0];
                let pip = event.daughter_p4s[1];
                let pim = event.daughter_p4s[2];
                let omega = pi0 + pip + pim;

                let dalitz_s = (pip + pim).m2();
                let dalitz_t = (pip + pi0).m2();
                let dalitz_u = (pim + pi0).m2();

                let m3pi = (2.0 * pip.m()) + pi0.m();
                let dalitz_d = 2.0 * omega.m() * (omega.m() - m3pi);
                let dalitz_sc = (1.0 / 3.0) * (omega.m2() + pip.m2() + pim.m2() + pi0.m2());
                let dalitz_x = f64::sqrt(3.0) * (dalitz_t - dalitz_u) / dalitz_d;
                let dalitz_y = 3.0 * (dalitz_sc - dalitz_s) / dalitz_d;

                let dalitz_z = dalitz_x * dalitz_x + dalitz_y * dalitz_y;
                let dalitz_sin3theta = f64::sin(3.0 * f64::asin(dalitz_y / f64::sqrt(dalitz_z)));

                let pip_omega = pip.boost_along(&omega);
                let pim_omega = pim.boost_along(&omega);
                let pi_cross = pip_omega.momentum().cross(&pim_omega.momentum());

                let lambda = (4.0 / 3.0) * f64::abs(pi_cross.dot(&pi_cross))
                    / ((1.0 / 9.0) * (omega.m2() - (2.0 * pip.m() + pi0.m()).powi(2)).powi(2));

                (dalitz_z, (dalitz_sin3theta, lambda))
            })
            .unzip();
        Ok(())
    }

    fn calculate(&self, parameters: &[f64], event: &Event) -> Result<Complex64, RustitudeError> {
        let dalitz_z = self.dalitz_z[event.index];
        let dalitz_sin3theta = self.dalitz_sin3theta[event.index];
        let lambda = self.lambda[event.index];
        let alpha = parameters[0];
        let beta = parameters[1];
        let gamma = parameters[2];
        let delta = parameters[3];
        Ok(f64::sqrt(f64::abs(
            lambda
                * (1.0
                    + 2.0 * alpha * dalitz_z
                    + 2.0 * beta * dalitz_z.powf(3.0 / 2.0) * dalitz_sin3theta
                    + 2.0 * gamma * dalitz_z.powi(2)
                    + 2.0 * delta * dalitz_z.powf(5.0 / 2.0) * dalitz_sin3theta),
        ))
        .into())
    }

    fn parameters(&self) -> Vec<String> {
        vec![
            "alpha".to_string(),
            "beta".to_string(),
            "gamma".to_string(),
            "delta".to_string(),
        ]
    }
}
```

Let's walk through this code. First, we need to define a `struct` which has all of the general information about the amplitude, and in this case some kind of `Vec` for storing precalculated data. We consider this precalculated data to correspond to a single dataset, and each dataset gets its own copy of the amplitude `struct`. Because this particular amplitude doesn't have any input parameters, we can `#[derive(Default)]` on it to make a default constructor, which allows the amplitude to be initialized with something like `let amp = OmegaDalitz::default();`. If we wanted a parameterized constructor, we have to define our own, and while Rust has no default name for constructors, `pub fn new(...) -> rustitude_core::AmpOp` is preferred.

Next, we implement the `Node` trait for the `struct`. Traits in Rust are kind of like abstract classes or interfaces in object-oriented languages, they provide some set of methods which a `struct` must implement. The first of these methods is `fn precalculate(&mut self, dataset: &Dataset) -> Result<(), RustitudeError>`. As the signature suggests, it takes a `Dataset` and mutates the `struct` in some way. It should raise a `RustitudeError` if anything goes wrong in the evaluation. The intended usage of this function is to precalculate some terms in the amplitude's mathematical expression, things which don't change when you update the free parameter inputs to the amplitude. In this case, the four input parameters, $`\alpha`$, $`\beta`$, $`\gamma`$, and $`\delta`$, are independent from `dalitz_z`, `dalitz_sin3theta`, and `lambda`, so we can safely calculate those ahead of time and just pull from their respective `Vec`s when needed later. I won't go too far into Rust's syntax here, but typical precalculation functions will start by iterating over the dataset's events in parallel (the line `use rayon::prelude::*;` is needed to use `par_iter` here) and collecting or unzipping that iterator into a `Vec` or group of `Vec`s.

The calculate step has the signature `fn calculate(&self, parameters: &[f64], event: &Event) -> Result<Complex64, RustitudeError>`. This means we need to take a list of parameters and a single event and turn them into a complex value. The `Event` struct contains an `index` field which can be used to access the precalculated storage arrays made in the previous step.

Finally, the `parameters` function just returns a list of the parameter names in the order they are expected to be input into `calculate`. In the event that an amplitude doesn't have any free parameters (like [my implementation of the `Ylm` and `Zlm` amplitudes](https://github.com/denehoffman/rustitude/blob/main/crates/rustitude-gluex/src/harmonics.rs)), we can omit this function entirely, as the default implementation returns `vec![]`.

And that's it! However, it is important to be aware of the steps which need to be taken to allow this amplitude to be used through the Python interface.

## Python Bindings

To make Python bindings, `pyo3` needs to be included as a dependency:

```rust
use pyo3::prelude::*;

// OmegaDalitz code here

#[pyfunction(name = "OmegaDalitz")]
fn omega_dalitz(name: &str) -> AmpOp {
    Amplitude::new(name, OmegaDalitz::default()).into()
}

pub fn pyo3_module(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(omega_dalitz, m)?)?;
    Ok(())
}
```

Rather than bind the `struct` directly, we prefer to bind a function which returns a `PyAmpOp`, a wrapper struct that implements `#[pyclass]` and can be used in the Python interface. The `pyo3_module` function will then need to be added to the `py-rustitude` crate's [`lib.rs`](https://github.com/denehoffman/rustitude/blob/main/py-rustitude/src/lib.rs) file. This step is a bit problematic, since it means new amplitudes cannot be added without modifying `py-rustitude` itself. For this reason, developers may want to work with their own fork of the repository rather than using the one installed by `cargo` if they wish to use Python with custom amplitudes. This is a limitation of `pyo3`, which doesn't recognize class bindings across crates. There is also a mechanism to implement `Amplitude`s via pure Python classes, but it is currently incompatible with multithreading due to the GIL (see the [Python docs](https://rustitude.readthedocs.io/en/latest/custom_nodes.html)).

# TODOs

In no particular order, here is a list of what (probably) needs to be done before I will stop making any breaking changes:

- Pure Rust parsing of ROOT files without the `uproot` backend (I have some moderate success with `oxyroot`, but there are still a few issues reading larger files)
- Add plotting methods
- A way to check if the number of parameters matches the input at compile time would be nice, not sure if it's possible though
- Amplitude serialization (I want to be able to send and share an amplitude configuration just like an `AmpTools` config file, but with the amplitude itself included)
- Lots of documentation
- Lots of tests
