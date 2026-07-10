.. _amplapi:



amplrs
======

``amplrs`` is a safe Rust interface to `AMPL <https://ampl.com>`_, the algebraic modeling language for
mathematical optimization. It lets you build, solve, and inspect AMPL models from Rust code.
For a quick introduction to AMPL itself see `Quick Introduction to AMPL <https://dev.ampl.com/ampl/introduction.html>`_.

Model generation and solver interaction are handled entirely by the AMPL interpreter, which is spawned
as a subprocess and driven over the AMPL C API; ``amplrs`` is a thin, safe wrapper around that API. This
gives great stability and speed - the added overhead depends mostly on how much data is sent and read
back from AMPL, not on the size of the expanded model.

Installation & minimal example
-------------------------------

.. code-block:: bash

    # Add amplrs to your crate
    $ cargo add amplrs

.. note::
    You can use a free `Community Edition license <https://ampl.com/ce>`_, which allows **free
    use of AMPL with Open-Source solvers**.

.. code-block:: rust

    use amplrs::Ampl;

    fn main() {
        let mut ampl = Ampl::new();
        ampl.eval(r#"
            set A ordered;
            param S{A, A};
            param lb default 0;
            param ub default 1;
            var w{A} >= lb <= ub;
            minimize portfolio_variance:
                sum {i in A, j in A} w[i] * S[i, j] * w[j];
            s.t. portfolio_weights:
                sum {i in A} w[i] = 1;
        "#);
        // ... assign data to A and S, e.g. via a DataFrame or ampl.set_data() ...
        ampl.set_option("solver", "gurobi");
        ampl.solve("", "");
        let sigma = ampl.get_objective("portfolio_variance").value().sqrt();
        println!("Volatility: {:.1}%", sigma * 100.0);
    }


Contents
--------

.. toctree::
   :maxdepth: 2

   intro
   getting-started
   quick-start
   class-structure
   reference
   examples
