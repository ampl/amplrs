.. _lblGettingStarted:

Initial Setup
=============

Installation
------------

``amplrs`` needs two separate things to work: a Rust crate dependency, and a local AMPL installation
(+ license) for it to talk to at runtime.

AMPL installation
~~~~~~~~~~~~~~~~~~

If you do not yet have an AMPL installation on the machine where you will run your program, see the
`AMPL Community Edition page <https://ampl.com/ce/>`_ to download a working version that can be
installed quickly and gives free use of AMPL with open-source solvers (e.g. HiGHS, CBC).

``amplrs`` looks for the ``ampl`` executable on the system ``PATH`` by default. If your AMPL installation
directory is not on ``PATH``, use :ref:`Environment <ref::Environment>` to point at it explicitly:

.. code-block:: rust

    use amplrs::{Ampl, Environment};

    let env = Environment::new("/full/path/to/the/ampl/installation/directory", "ampl");
    let mut ampl = Ampl::new_with_env(&env);

Adding the crate
~~~~~~~~~~~~~~~~~

.. code-block:: bash

    cargo add amplrs

FFI build requirements
~~~~~~~~~~~~~~~~~~~~~~~

``amplrs`` links against the AMPL C API's shared library and generates its FFI bindings at build time
with `bindgen <https://rust-lang.github.io/rust-bindgen/>`_, which needs ``libclang`` available.

For Windows only, `bindgen` needs to find ``libclang.dll``; if it is not auto-detected, set
``LIBCLANG_PATH`` to the directory containing it (e.g. an installed LLVM's ``bin`` directory).

Initial test
------------

To begin, clone the repository:

.. code-block:: bash

    git clone https://github.com/ampl/amplrs.git
    cd amplrs

To complete an initial test, run ``firstexample`` with

.. code-block:: bash

   cargo run --example firstexample -- <solver>

where optionally ``<solver>`` is the name of a solver that has been installed with AMPL (e.g. ``highs``
or ``gurobi``). If a solver is not specified, AMPL's default choice will be used, which will fail with
"No solver specified" unless one has already been configured via ``option solver``. This will solve
several small diet problems and then display the optimal amounts of the foods from the last solution.
The other example programs are in `examples/ <https://github.com/ampl/amplrs/tree/master/examples>`_.

If AMPL could not be started, "Can't find" / "invalid syntax" style errors will come from AMPL
itself, and an ``AMPL_ERRORINFO`` panic from ``amplrs``. If the AMPL installation directory
is not in the system search path, use ``Environment`` as shown above:

.. code-block:: rust

    use amplrs::{Ampl, Environment};

    let env = Environment::new(r"C:\ampl\ampl.mswin64", "ampl.exe");
    let mut ampl = Ampl::new_with_env(&env);

Development
-----------

If you have an existing AMPL installation in the system search path:

.. code-block:: rust

   use amplrs::Ampl;

   let mut ampl = Ampl::new();
   println!("{}", ampl.get_option("version"));

If you have an existing AMPL installation, but not in the system search path:

.. code-block:: rust

    use amplrs::{Ampl, Environment};

    let env = Environment::new("/full/path/to/the/ampl/installation/directory", "ampl");
    let mut ampl = Ampl::new_with_env(&env);
    println!("{}", ampl.get_option("version"));
