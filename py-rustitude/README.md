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

### Table of Contents:

- [Introduction](#Introduction)
- [Installation](#Installation)
- [Usage](#Usage)
- [TODOs](#TODOs)

# Introduction

This project began with a desire to make a fast but easy to use interface for fitting amplitudes to particle physics data. That being said, there are performant methods such as [`AmpTools`](https://github.com/mashephe/AmpTools), which is written in C++, but in my personal experience, it can be a bit tricky to use and extend, and it generally requires a lot of boilerplate code to generate new amplitudes or plotting scripts. On the other hand, there are also libraries like [`PyPWA`](https://github.com/JeffersonLab/PyPWA/) (written in Python) which seem like they could be easy to use, but often fail in this aspect due to Python's limiting syntax, speed issues, and a general lack of documentation and ongoing development. There have been attempts to bridge the gap between AmpTools and Python, most recently (and successfully) [`PyAmpTools`](https://github.com/lan13005/PyAmpTools). The difficulty with this method is that it relies on PyROOT, which also means you need ROOT installed (and built with your version of Python). For now, I'll spare you the anti-ROOT rant and just say that ROOT should be an opt-in, not a requirement. So where does that leave `rustitude`?

As the name suggests, `rustitude` was written in Rust, so let's get the obvious downside out of the way: not many particle physicists know how to write Rust code. Hopefully, this will change over the next decade (and there has already been some [support](https://www.whitehouse.gov/oncd/briefing-room/2024/02/26/memory-safety-statements-of-support/) from the US government, of all places). While Rust carries the disadvantage of relative obscurity compared to C++, it also has many benefits. No `null` means no null references (Tony Hoare's ["billion dollar mistake"](https://web.archive.org/web/20090628071208/http://qconlondon.com/london-2009/speaker/Tony+Hoare)). In Rust, references are always valid, a guarantee made by a very helpful and only occasionally frustrating borrow checker. Rust "crates" are set up in a way which encourages documentation (see [`rustitude-core`'s documentation](https://docs.rs/rustitude-core/)), and the basic syntax is fairly easy to learn for people who have been using optional type checking in Python. Perhaps one of the biggest benefits of Rust is how easy it is to employ [parallelization](https://crates.io/crates/rayon), but the two reasons I like it most are that it's incredibly easy to write Python bindings (that's what this library is after all) and it has a package manager. This second point is important -- unlike C/C++, where a developer is swamped with some menagerie `Makefile`, `CMakeLists.txt`, or some `scons` monstrosity which may only work on "X" system and only if you install and use `make`, `cmake`, `g++`, or whatever (oh yeah, and you made sure all your external dependencies are linked correctly, right? Right?), Rust supports adding a package by simply adding a line to `Cargo.toml` (or using the `cargo add` command). In many ways, package management in Rust is actually simpler than Python, since there's only one prefered method of creating and managing projects, formatting, linting, and compiling.

Now I've covered why I don't like some of the existing solutions, and why I chose to use Rust, but what does this project have that makes it stand out? Here are some reasons to entice you:

- `rustitude` will automatically parallelize amplitudes over the events in a dataset. There's no reason for a developer to ever write parallelized code themselves.
- Implementing [`Node`](https://docs.rs/rustitude-core/latest/rustitude_core/amplitude/trait.Node.html) on a struct is all that is needed to use it as an amplitude. This means developers need only write two to three total methods to describe the entire functionality of their amplitude, and one of these just gives the names and order of the amplitude's input parameters.
- A major goal of `rustitude` was to increase processing speed by sacrificing memory. This is done by precalculating parts of amplitudes which don't change when the free parameter inputs change. `AmpTools` supports a version of this, but only on the level of each general amplitude rather than on an individual basis. The simplest example of this is the `Ylm` amplitude (spherical harmonic), which can be entirely precalculated given the value of `l` and `m`. In `AmpTools`, different instances of `Ylm` with different `l`s and `m`s share precalculated data, whereas in `rustitude`, they don't. The `AmpTools` implementation of `Ylm` needs to calculate a spherical harmonic for every event on every function call, while `rustitude` just needs to look up a value in an array!
- The majority of the library (the public interface) has Python bindings, so if there is no need for custom amplitudes, a developer never actually has to write any Rust code, and the resulting calculations will be as performant as if they were written in Rust.

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

See the [`rustitude-core`](https://github.com/denehoffman/rustitude/tree/main/crates/rustitude-core) crate for a more in-depth tutorial on writing custom amplitudes in Rust. This package is mostly focused on the Python side of things. Here is the setup for an example analysis:

```python
import rustitude as rt
from rustitude import gluex
import numpy as np

# Start by creating some amplitudes:
f0p = gluex.resonances.KMatrixF0('f0+', channel=2)
f0n = gluex.resonances.KMatrixF0('f0-', channel=2)
f2 = gluex.resonances.KMatrixF2('f2', channel=2)
a0p = gluex.resonances.KMatrixA0('a0+', channel=1)
a0n = gluex.resonances.KMatrixA0('a0-', channel=1)
a2 = gluex.resonances.KMatrixA2('a2', channel=1)
s0p = gluex.harmonics.Zlm('Z00+', 0, 0, reflectivity='+')
s0n = gluex.harmonics.Zlm('Z00-', 0, 0, reflectivity='-')
d2p = gluex.harmonics.Zlm('Z22+', 2, 2, reflectivity='+')

# Next, let's put them together into a model
# The API supports addition and multiplication and has additional methods for the real part (`real`) and imaginary part (`imag`).
pos_re_sum = (f0p + a0p) * s0p.real() + (f2 + a2) * d2p.real()
pos_im_sum = (f0p + a0p) * s0p.imag() + (f2 + a2) * d2p.imag()
neg_re_sum = (f0n + a0n) * s0n.real()
neg_im_sum = (f0n + a0n) * s0n.imag()

mod = rt.Model([pos_re_sum, pos_im_sum, neg_re_sum, neg_im_sum])

# There is no need to constrain amplitudes, since each named amplitude is only ever evaluated once and a cached value gets pulled if we run across an amplitude by the same name!
# We should, however, fix some of the values to make the fit less ambiguous. For instance, suppose we are above the threshold for the f_0(500) which is included in the F0 K-Matrix:
mod.fix("f0+", "f0_500 re", 0.0)
mod.fix("f0+", "f0_500 im", 0.0)
mod.fix("f0-", "f0_500 re", 0.0)
mod.fix("f0-", "f0_500 im", 0.0)

# As mentioned, we should also fix at least one complex phase per coherent sum:
mod.fix("f0+", "f0_980 im", 0.0)
mod.fix("f0-", "f0_980 im", 0.0)

# All done! Now let's load our model into a Manager, which helps coordinate the model with datasets.
# First, load up some datasets. rustitude provides an open operation that uses uproot under the hood:
ds = rt.open('data.root')
ds_mc = rt.open('mc.root')

m_data = rt.Manager(mod, ds)
m_mc = rt.Manager(mod, ds_mc)

# We could stop here and evaluate just one dataset at a time:

# res = m_data([1.0, 3.4, 2.8, -3.6, ... ])
# -> [5.3, 0.2, ...], a list of intensities from the amplitude calculation

# Or, what we probably want to do is find the negative log-likelihood for some choice of parameters:

nll = rt.ExtendedLogLikelihood(m_data, m_mc)

res = nll([10.0] * mod.n_free) # automatic CPU parallelism without GIL
print(res) # prints some value for the NLL
```

Automatic parallelism over the CPU can be disabled via function calls which support it (for example, `nll([10.0] * mod.n_free, parallel=False)` would run without parallel processing), and the number of CPUs used can be controlled via the `RAYON_NUM_THREADS` environment variable, which can be set before the code is run or modified inside the code (for example, `os.environ['RAYON_NUM_THREADS] = '5'` would ensure only five threads are used past that point in the code). By default, an unset value or the value of `'0'` will use all available cores.

See the [`rustitude-gluex`](https://github.com/denehoffman/rustitude/tree/main/crates/rustitude-gluex) package for some of the currently implemented amplitudes (derived from GlueX's [halld_sim](https://github.com/JeffersonLab/halld_sim) repo). There are also some helper methods `Scalar`, `CScalar`, and `PCScalar` to create amplitudes which represent a single free parameter, a single complex free parameter, and a single complex free parameter in polar coordinates respectively.

# TODOs

In no particular order, here is a list of what (probably) needs to be done before I will stop making any breaking changes:

- Pure Rust parsing of ROOT files without the `uproot` backend (I have some moderate success with `oxyroot`, but there are still a few issues reading larger files)
- Add plotting methods
- A way to check if the number of parameters matches the input at compile time would be nice, not sure if it's possible though
- Give managers a way to apply amplitudes to new datasets, like using the result from a fit to weight some generated Monte-Carlo for plotting the result. This is possible to do through Python, but a convenience method is probably needed
- Lots of documentation
- Lots of tests
