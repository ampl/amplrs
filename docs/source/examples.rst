.. _secExamplesRust:

Examples
========

This section lists examples and shows the basic usage of ``amplrs``. Each takes an optional solver name as its first command-line
argument, e.g. ``cargo run --example firstexample -- highs``.

Example 1: First steps
------------------------

:download:`examples/firstexample.rs <../../examples/firstexample.rs>`

This example shows how to:

* read an AMPL model and data file
* reassign values to a parameter, both for specific instances and all at once
* solve the model
* display the objective function value
* read a variable's values into a :ref:`DataFrame <ref::DataFrame>`, and read an arbitrary AMPL
  expression into a DataFrame via ``Ampl::get_data``

Example 2: Get and set AMPL options
--------------------------------------

:download:`examples/optionsexample.rs <../../examples/optionsexample.rs>`

This example shows how to:

* get and set AMPL options (``presolve``, ``solver``)
* check whether an option with a given name exists

Example 3: Assign all data to a model and solve it
-------------------------------------------------------

:download:`examples/dietexample.rs <../../examples/dietexample.rs>`

This example shows how to:

* assign all the data necessary to generate a model instance programmatically, using ``DataFrame``\ s for
  1-index columns (``set_column_doubles``/``set_column_strings``) and a 2-index matrix
  (``set_matrix_doubles``)
* solve the resulting model without reading any ``.dat`` file

Example 4: Multi-dimensional data
------------------------------------

:download:`examples/multidimensionalexample.rs <../../examples/multidimensionalexample.rs>`

This example shows how to:

* build a DataFrame with two index columns (a network's ``LINKSFrom``/``LINKSTo`` pair) and two data
  columns (``cost``, ``capacity``)
* assign it to a set declared with ``within (CITIES cross CITIES)``

Example 5: Write a model and handle AMPL errors
-----------------------------------------------------

:download:`examples/writemodel.rs <../../examples/writemodel.rs>`

This example shows how to:

* build a small LP/model programmatically across several variants (infeasible, unbounded, trivial)
* use ``amplrs::error::catch_ampl_error`` to catch an AMPL-side error (e.g. presolve detecting
  infeasibility while writing a model) without the whole program panicking, and inspect its message

Example 6: Simple heuristic
------------------------------

:download:`examples/trackingmodel.rs <../../examples/trackingmodel.rs>`

This example shows how to:

* read a larger model plus table-driven data (``read_table``) from a directory of files
* solve a relaxed (QP) version of a mixed-integer tracking-portfolio model
* use the relaxed solution to fix some binary decisions heuristically, via
  ``Parameter::set_all_double_values``
* solve the resulting smaller mixed-integer (QMIP) problem
