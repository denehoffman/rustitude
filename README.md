<p align="center">
  <img
    width="800"
    src="https://raw.githubusercontent.com/denehoffman/rustitude/main/media/logo.png"
  />
</p>
<p align="center">
    <h1 align="center">Demystifying Amplitude Analysis</h1>
</p>

<p align="center">
  <a href="https://github.com/denehoffman/rustitude/commits/main/" alt="Lastest Commits">
    <img src="https://img.shields.io/github/last-commit/denehoffman/rustitude/main" /></a>
  <a href="https://github.com/denehoffman/rustitude/actions" alt="Build Status">
    <img src="https://img.shields.io/github/actions/workflow/status/denehoffman/rustitude/rust.yml" /></a>
  <a href="LICENSE" alt="License">
    <img src="https://img.shields.io/github/license/denehoffman/rustitude" /></a>
  <a href="https://crates.io/crates/rustitude" alt="Rustitude on crates.io">
    <img src="https://img.shields.io/crates/v/rustitude" /></a>
  <a href="https://docs.rs/rustitude/latest/rustitude/" alt="Rustitude documentation on docs.rs">
    <img src="https://img.shields.io/docsrs/rustitude" /></a>
</p>


### Note: This project is still very much under development and not recommended for use in actual research projects (yet)

### Table of Contents
- [Overview](#Overview)
- [Installation](#Installation)
- [Usage](#Usage)

## Overview
Amplitude analysis is the scientific process of fitting models (amplitudes) to data in order to extract additional information. In particle physics, this is often synonymous with partial-wave analysis (PWA), a process commonly used to determine angular momentum quantum numbers for decaying particles. The goal of Rustitude is to establish a framework which is generic enough to fit almost any data to any model, but specific enough to be quick and easy to use in typical particle physics studies. There are three core ideas which this crate tries to follow to make fits fast and efficient:

1. Every function is automatically paralellized over the dataset using [rayon](https://github.com/rayon-rs/rayon).
2. Model builders can separate their code efficiently into pieces which only need to be calculated once for the whole dataset and those which depend on parameters, which could change at every evaluation. Amplitudes implement `Node`, which is a trait containing all the required methods for an amplitude to be used in an analysis. By precalculating things which don't change, we trade RAM usage now for evaluation speed later.
3. `Dataset` structs only allow `Event` structs to be stored, so there is a fixed format that implementers must agree on. As it stands, this format closely follows that of [AmpTools](https://github.com/mashephe/AmpTools), but with the addition of a polarization vector. This format can be added to in the future without breaking backwards compatibility, but members should probably not be removed or modified. Unfortunately, ROOT files do not yet have well-documented R/W libraries like Python's [uproot](https://pypi.org/project/uproot/) or Julia's [UpROOT.jl](https://github.com/JuliaHEP/UpROOT.jl), so the current recommendation is to convert files to a more versatile format like parquet. A ROOT-free Python conversion script is included [here](convert).

## Installation

Cargo provides the usual command for including this crate in a project:
```sh
cargo add rustitude
```

## Usage
The basic usage pattern is as follows. We start by importing our data files, we create the amplitudes we want to use, we register them with a `Manager` struct that handles all the internal sums and parameters, and then we evaluate the resulting model. For example, we can generate a model which takes evaluates two spherical harmonics as follows (this uses some code from [another repo of mine](https://github.com/denehoffman/rustitude-gluex/) which contains the spherical harmonic struct):
```rust
use rustitude::prelude::*;
use rustitude_gluex::harmonics::{Wave, Ylm};

fn main() {
    // Load Datasets:
    let data = Dataset::from_parquet("path/to/data.parquet", false);
    let mc = Dataset::from_parquet("path/to/mc.parquet", false);
    // Create Manager:
    let mut m = ExtendedLogLikelihood::new(&data, &mc);
    // Create Amplitudes:
    let s0 = amplitude!("S0", Ylm::new(Wave::S0));
    let d2 = amplitude!("D2", Ylm::new(Wave::D2));
    let s0_amp = scalar!("S0 amp");
    let d2_amp = cscalar!("D2 amp");
    // Register Amplitudes:
    m.register("", "S0", &s0);
    m.register("", "S0", &s0_amp);
    m.register("", "D2", &d2);
    m.register("", "D2", &d2_amp);
    // You can check the names of the free parameters and how many there are:
    dbg!(m.parameters());
    // prints:
    // m.parameters() = [
    // ("", "S0", "S0 amp", "value"),
    // ("", "D2", "D2 amp", "real"),
    // ("", "D2", "D2 amp", "imag"),
    // ]
    // Compute the negative log-likelihood:
    let nll = dbg!(m.compute(&[1.0, 2.0, 3.0]));
    std::hint::black_box(nll);
}
```
Amplitudes are registered into a named `sum` and `group`. The typical calculation for any event $e$ and list of parameters $\overrightarrow{p}$ will then be:
```math
I(\overrightarrow{p}, e) = \sum_{\text{groups} \in \text{sums}}\left|\sum_{\text{amplitudes} \in \text{groups}} \prod_{\text{amp} \in \text{amplitudes}} \text{amp}(\overrightarrow{p}, e)\right|^2
```

This is the behavior of a `Manager` struct, but more complex `Manage`-implementing structs like `ExtendedLogLikelihood` can be used to run analyses over more than one dataset at a time (data and Monte Carlo in this case) and compute other values like a negative log-likelihood.

## Implementing an Amplitude

While typical usage might be to use premade amplitudes in various combinations, it is important to know how to design an amplitude which will work seamlessly with this crate. Let's write down the Rustitude version of the [OmegaDalitz](https://github.com/JeffersonLab/halld_sim/blob/6544f01ac1514b0b9a53ad241cf2e8a63e1d3dfa/src/libraries/AMPTOOLS_AMPS/OmegaDalitz.cc) amplitude:

```rust
use rayon::prelude::*;
use rustitude::prelude::*;

#[derive(Default)]
struct OmegaDalitz {
    dalitz_z: Vec<f64>,
    dalitz_sin3theta: Vec<f64>,
    lambda: Vec<f64>,
}

impl Node for OmegaDalitz {
    fn precalculate(&mut self, dataset: &Dataset) {
        (self.dalitz_z, (self.dalitz_sin3theta, self.lambda)) = dataset
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
    }

    fn calculate(&self, parameters: &[f64], event: &Event) -> Complex64 {
        let dalitz_z = self.dalitz_z[event.index];
        let dalitz_sin3theta = self.dalitz_sin3theta[event.index];
        let lambda = self.lambda[event.index];
        let alpha = parameters[0];
        let beta = parameters[1];
        let gamma = parameters[2];
        let delta = parameters[3];
        f64::sqrt(f64::abs(
            lambda
                * (1.0
                    + 2.0 * alpha * dalitz_z
                    + 2.0 * beta * dalitz_z.powf(3.0 / 2.0) * dalitz_sin3theta
                    + 2.0 * gamma * dalitz_z.powi(2)
                    + 2.0 * delta * dalitz_z.powf(5.0 / 2.0) * dalitz_sin3theta),
        ))
        .into()
    }

    fn parameters(&self) -> Option<Vec<String>> {
        Some(vec![
            "alpha".to_string(),
            "beta".to_string(),
            "gamma".to_string(),
            "delta".to_string(),
        ])
    }
}

fn main() {
    let data = Dataset::from_parquet("data.parquet", false);
    let mut m = Manager::new(&data);
    let dalitz = amplitude!("Omega Dalitz", OmegaDalitz::default())
    m.register("", "", &dalitz);
    let res: Vec<f64> = m.compute(&[1.1, 2.2, 3.3, 4.4]);
    println!("First event result: {}", res[0]);
}
```

# TODOs
In no particular order, here is a list of what (probably) needs to be done before I will stop making any breaking changes:
- [ ] Create convenience methods for binned fits
- [x] Formalize parameters into their own struct for pretty-printing and ease of use
- [x] Create alternate methods of including polarization
- [ ] Read raw ROOT files
- [ ] Add plotting methods
- [ ] A way to check if the number of parameters matches the input at compile time would be nice, not sure if it's possible though
- [ ] If the parameters are formalized, I could include upper and lower bounds somehow as well as preferred initial values or randomization
- [ ] Lots of documentation
- [ ] Lots of tests
