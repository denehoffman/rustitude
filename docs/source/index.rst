.. rustitude documentation master file, created by
   sphinx-quickstart on Fri Jun 21 13:00:59 2024.
   You can adapt this file completely to your liking, but it should at least
   contain the root `toctree` directive.

Welcome to rustitude's documentation!
=====================================

**rustitude** (rust + amplitude) is a Python library for performing amplitude analyses, particularly for particle physics research. As the name suggests, it is mostly built in Rust, a compiled language like C, and contains bindings which are accessible through a Python API.

.. note::
   This project is under active development, and **BREAKING CHANGES** might still happen. However, the core functionality of the project is stable and is unlikely to change significantly. Use with caution!

Installation
------------

Installing rustitude is as simple as

.. code-block:: bash

   pip install rustitude

This should actually install fairly quickly for most OSes, since rustitude builds wheels for most major distributions.

Building from Source
--------------------

The Rust library with Python bindings can also be built manually from source.

First, you'll need Rust. For most Unix systems (macOS, Linux), you can use rustup:

.. code-block:: bash

   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

See `the Rust documentation <https://www.rust-lang.org/tools/install>`_ for alternative installation methods.

Next, clone the repo:

.. code-block:: bash

   git clone git@github.com:denehoffman/rustitude.git
   cd rustitude

Next, optionally create a virtual environment:

.. code-block:: bash

   python -m venv .venv
   source .venv/bin/activate

(the activate script has versions for cshell and fish as well)

Install maturin:

.. code-block:: bash

   pip install maturin

To wrap up, you can either use

.. code-block:: bash

   cd py-rustitude
   maturin develop --release

to install rustitude as an editable package in your active venv, or you can use

.. code-block:: bash

   cd py-rustitude
   maturin build --release
   pip install ../target/wheels/rustitude*

to build the wheel and install it wherever you want.

Usage
-----

To get started with rustitude, you can import it as follows:

.. code-block:: python

   import rustitude as rt # preferred shorthand import

Contents
--------

.. toctree::
   :maxdepth: 2
   :caption: Contents:

   general_usage
   custom_nodes

General Usage
^^^^^^^^^^^^^

Learn about the basic concepts and how to use rustitude for amplitude analysis.

Writing Custom Nodes
^^^^^^^^^^^^^^^^^^^^

Learn how to extend rustitude's functionality by creating your own custom Python nodes using the rustitude.PyNode abstract base class.

Indices and tables
==================

* :ref:`genindex`
* :ref:`modindex`
* :ref:`search`
