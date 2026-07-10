Introduction
============

What are AMPL APIs?
--------------------

`AMPL APIs <https://ampl.com/apis/>`_ are interfaces that allow developers to access the features of AMPL
from within a programming language. All model generation and solver interaction is handled
directly by AMPL, which leads to great stability and speed. Functions for directly assigning data to AMPL parameters and
sets are provided, which can be used instead of the normal AMPL data reading procedures. AMPL APIs are
available for
`Python <https://ampl.com/api/latest/python>`_,
`R <https://ampl.com/api/latest/R>`_,
`C <https://ampl.com/api/latest/c>`_,
`C++ <https://ampl.com/api/latest/cpp>`_,
`C#/.NET <https://ampl.com/api/latest/dotnet>`_,
`Java <https://ampl.com/api/latest/java>`_,
`MATLAB <https://ampl.com/api/latest/matlab>`_, and, with ``amplrs``, Rust.

``amplrs`` wraps the AMPL C API directly: it is a thin, safe Rust layer with no runtime dependency on
any other language's AMPL binding.

Who can use amplrs
-------------------

The intended user is a Rust developer who needs to connect an application to optimization models and
solvers, or anyone comfortable with Rust who wants to build a proof-of-concept optimization application
without leaving the Rust ecosystem.

System requirements
--------------------

``amplrs`` requires:

* A Rust toolchain new enough for the 2024 edition (see ``Cargo.toml``).
* A local AMPL installation (+ license) to actually *run* models - see :ref:`lblGettingStarted`.
* ``libclang``, used by `bindgen <https://rust-lang.github.io/rust-bindgen/>`_ to generate the FFI
  bindings at build time. This is preinstalled on most Linux and macOS toolchains; on Windows it may
  need to be pointed to explicitly via the ``LIBCLANG_PATH`` environment variable.

The officially supported platforms are Windows, Linux, and macOS.

About this manual
------------------

This document guides a developer through implementing an AMPL-based application in Rust. The section
:ref:`secClassStructure` presents the main logic of the API, which mirrors the same concepts (entities,
instances, DataFrames) shared by every AMPL API regardless of language. Further sections walk through
the implementation of common tasks; finally the sections :ref:`secReferenceRust` and :ref:`secExamplesRust`
contain the API reference documentation and a collection of examples.
