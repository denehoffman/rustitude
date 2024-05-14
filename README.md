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
  <a href="https://docs.rs/rustitude" alt="Rustitude documentation on docs.rs">
    <img src="https://img.shields.io/docsrs/rustitude" /></a>
</p>

### Table of Contents:
- [Introduction](#Introduction)
- [Theory](#Theory)
- [Installation](#Installation)
- [Usage](#Usage)
- [TODOs](#TODOs)

# Introduction
This project began with a desire to make a fast but easy to use interface for fitting amplitudes to particle physics data. That being said, there are performant methods such as [`AmpTools`](https://github.com/mashephe/AmpTools), which is written in C++, but in my personal experience, it can be a bit tricky to use and extend, and it generally requires a lot of boilerplate code to generate new amplitudes or plotting scripts. On the other hand, there are also libraries like [`PyPWA`](https://github.com/JeffersonLab/PyPWA/) (written in Python) which seem like they could be easy to use, but often fail in this aspect due to Python's limiting syntax, speed issues, and a general lack of documentation and ongoing development. There have been attempts to bridge the gap between AmpTools and Python, most recently (and successfully) [`PyAmpTools`](https://github.com/lan13005/PyAmpTools). The difficulty with this method is that it relies on PyROOT, which also means you need ROOT installed (and built with your version of Python). For now, I'll spare you the anti-ROOT rant and just say that ROOT should be an opt-in, not a requirement. So where does that leave `rustitude`?

As the name suggests, `rustitude` was written in Rust, so let's get the obvious downside out of the way: not many particle physicists know how to write Rust code. Hopefully, this will change over the next decade (and there has already been some [support](https://www.whitehouse.gov/oncd/briefing-room/2024/02/26/memory-safety-statements-of-support/) from the US government, of all places). While Rust carries the disadvantage of relative obscurity compared to C++, it also has many benefits. No `null` means no null references (Tony Hoare's ["billion dollar mistake"](https://web.archive.org/web/20090628071208/http://qconlondon.com/london-2009/speaker/Tony+Hoare)). Pointers (called references in Rust) are always valid, a guarantee made by a very helpful and only occasionally frustrating borrow checker. Rust "crates" are set up in a way which encourages documentation (see [`rustitude-core`'s documentation](https://docs.rs/rustitude-core/)), and the basic syntax is fairly easy to learn for people who have been using optional type checking in Python. Perhaps one of the biggest benefits of Rust is how easy it is to employ [parallelization](https://crates.io/crates/rayon), but the two reasons I like it most are that it's incredibly easy to write Python bindings (that's what this library is after all) and it has a package manager. This second point is important -- unlike C/C++, where a developer is swamped with some menagerie `Makefile`, `CMakeLists.txt`, or some `scons` monstrosity which may only work on "X" system and only if you install and use `make`, `cmake`, `g++`, or whatever (oh yeah, and you made sure all your external dependencies are linked correctly, right? Right?), Rust supports adding a package by simply adding a line to `Cargo.toml` (or using the `cargo add` command). In many ways, package management in Rust is actually simpler than Python, since there's only one prefered method of creating and managing projects, formatting, linting, and compiling.

Now I've covered why I don't like some of the existing solutions, and why I chose to use Rust, but what does this project have that makes it stand out? Here are some reasons to entice you:

- `rustitude` will automatically parallelize amplitudes over the events in a dataset. There's no reason for a developer to ever write parallelized code themselves.
- Implementing [`Node`](https://docs.rs/rustitude-core/latest/rustitude_core/amplitude/trait.Node.html) on a struct is all that is needed to use it as an amplitude. This means developers need only write two to three total methods to describe the entire functionality of their amplitude, and one of these just gives the names and order of the amplitude's input parameters.
- A major goal of `rustitude` was to increase processing speed by sacrificing memory. This is done by precalculating parts of amplitudes which don't change when the free parameter inputs change. `AmpTools` supports a version of this, but only on the level of each general amplitude rather than on an individual basis. The simplest example of this is the `Ylm` amplitude (spherical harmonic), which can be entirely precalculated given the value of `l` and `m`. In `AmpTools`, different instances of `Ylm` with different `l`s and `m`s share precalculated data, whereas in `rustitude`, they don't. The `AmpTools` implementation of `Ylm` needs to calculate a spherical harmonic for every event on every function call, while `rustitude` just needs to look up a value in an array!
- The majority of the library (the public interface) has Python bindings, so if there is no need for custom amplitudes, a developer never actually has to write any Rust code, and the resulting calculations will be as performant as if they were written in Rust.

# Theory

Amplitudes are registered into a named `sum` and `group`. Similar to `AmpTools`, the typical calculation for any event $e$ and list of parameters $\overrightarrow{p}$ will then be:
```math
I(\overrightarrow{p}, e) = \sum_{\text{groups} \in \text{sums}}\left|\sum_{\text{amplitudes} \in \text{groups}} \prod_{\text{amp} \in \text{amplitudes}} \text{amp}(\overrightarrow{p}, e)\right|^2
```

# Installation
Adding `rustitude` to an existing Rust project is as simple as
```shell
cargo add rustitude
```

The Python installation is equally straightforward:
```shell
pip install rustitude
```

# Usage
See the [`rustitude-core`](https://github.com/denehoffman/rustitude-core) crate for a more in-depth tutorial on writing custom amplitudes in Rust. This package is mostly focused on the Python side of things. Here is the setup for an example analysis:
```python
import rustitude as rt
from rustitude import gluex
import numpy as np

# Load data files
# This uses uproot under the hood
ds = rt.open('data.root')
ds_mc = rt.open('mc.root')

# Create a new "manager" to handle the interface between data and amplitudes
m = rt.ExtendedLogLikelihood(ds, ds_mc)

# Register some amplitudes
# We provide a sum name, group name, and a named amplitude
# This function also runs the precalculation method over the datasets
m.register('pos re', 'S', gluex.resonances.KMatrixF0('f0', channel=2))
m.register('pos re', 'S', gluex.resonances.KMatrixA0('a0', channel=1))
m.register('pos re', 'S', gluex.harmonics.Zlm('z00', 0, 0, reflectivity='+', part='real'))
m.register('pos re', 'D', gluex.resonances.KMatrixF2('f2', channel=2))
m.register('pos re', 'D', gluex.resonances.KMatrixA2('a2', channel=1))
m.register('pos re', 'D', gluex.harmonics.Zlm('z22', 0, 0, reflectivity='+', part='real'))

m.register('pos im', 'S', gluex.resonances.KMatrixF0('f0', channel=2))
m.register('pos im', 'S', gluex.resonances.KMatrixA0('a0', channel=1))
m.register('pos im', 'S', gluex.harmonics.Zlm('z00', 0, 0, reflectivity='+', part='imag'))
m.register('pos im', 'D', gluex.resonances.KMatrixF2('f2', channel=2))
m.register('pos im', 'D', gluex.resonances.KMatrixA2('a2', channel=1))
m.register('pos im', 'D', gluex.harmonics.Zlm('z22', 0, 0, reflectivity='+', part='imag'))

# We can constrain all the free parameters of one amplitude to be equal to those of another,
# provided they have the same free parameters. To constrain an individual amplitude, we can use
# m.constrain(('pos re', 'S', 'f0', 'f0_500 re'), ('pos re', 'S', 'f0', 'f0_500 im')) for example,
# which would make the real and imaginary part of equal to each other in the calculation.
# This step reduces the number of free parameters in the calculation.
m.constrain_amplitude(('pos re', 'S', 'f0'), ('pos im', 'S', 'f0'))
m.constrain_amplitude(('pos re', 'S', 'a0'), ('pos im', 'S', 'a0'))
m.constrain_amplitude(('pos re', 'D', 'f2'), ('pos im', 'D', 'f2'))
m.constrain_amplitude(('pos re', 'D', 'a2'), ('pos im', 'D', 'a2'))

# Fix some parameters to a given value (zero in this case)
m.fix(('pos re', 'S', 'f0', 'f0_500 re'), 0.0)
m.fix(('pos re', 'S', 'f0', 'f0_500 im'), 0.0)
m.fix(('pos re', 'S', 'f0', 'f0_980 im'), 0.0)

m.register('neg re', 'S', gluex.resonances.KMatrixF0('f0', channel=2))
m.register('neg re', 'S', gluex.resonances.KMatrixA0('a0', channel=1))
m.register('neg re', 'S', gluex.harmonics.Zlm('z00', 0, 0, reflectivity='-', part='real'))

m.register('neg im', 'S', gluex.resonances.KMatrixF0('f0', channel=2))
m.register('neg im', 'S', gluex.resonances.KMatrixA0('a0', channel=1))
m.register('neg im', 'S', gluex.harmonics.Zlm('z00', 0, 0, reflectivity='-', part='imag'))

m.constrain_amplitude(('neg re', 'S', 'f0'), ('neg im', 'S', 'f0'))
m.constrain_amplitude(('neg re', 'S', 'a0'), ('neg im', 'S', 'a0'))

m.fix(('neg re', 'S', 'f0', 'f0_500 re'), 0.0)
m.fix(('neg re', 'S', 'f0', 'f0_500 im'), 0.0)
m.fix(('neg re', 'S', 'f0', 'f0_980 im'), 0.0)

# Calculate the negative log-likelihood given some random input parameters:
rng = np.random.default_rng()
nll = m(rng.random(len(m.parameters())) * 100.0)
```

See the [`rustitude-gluex`](https://github.com/denehoffman/rustitude-gluex) package for some of the currently implemented amplitudes (derived from GlueX's [halld_sim](https://github.com/JeffersonLab/halld_sim) repo). There are also some helper methods `scalar`, `cscalar`, and `pcscalar` to create amplitudes which represent a single free parameter, a single complex free parameter, and a single complex free parameter in polar coordinates respectively.

# TODOs
In no particular order, here is a list of what (probably) needs to be done before I will stop making any breaking changes:
- Pure Rust parsing of ROOT files without the `uproot` backend (I have some moderate success with `oxyroot`, but there are still a few issues reading larger files)
- Add plotting methods
- A way to check if the number of parameters matches the input at compile time would be nice, not sure if it's possible though
- Give managers a way to apply amplitudes to new datasets, like using the result from a fit to weight some generated Monte-Carlo for plotting the result. This is possible to do through Python, but a convenience method is probably needed
- Lots of documentation
- Lots of tests
