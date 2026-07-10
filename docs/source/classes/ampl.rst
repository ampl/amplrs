.. _ref::Ampl:

Ampl
----

A running AMPL interpreter instance.

All interaction with the AMPL engine goes through this type: loading models and data, solving, reading
results, and setting data via ``DataFrame``\ s. The underlying AMPL process is closed automatically when
the value is dropped.

Construction
~~~~~~~~~~~~~

``Ampl::new()``
    Create a new AMPL interpreter instance using the system ``PATH`` to locate the binary.

``Ampl::new_with_env(env: &Environment)``
    Create a new AMPL interpreter instance using the given :ref:`Environment <ref::Environment>` to
    locate the binary.

``Ampl::clone(&self) -> Self``
    Return a shallow copy sharing the same underlying AMPL pointer.

Direct interaction
~~~~~~~~~~~~~~~~~~~~

``eval(&mut self, statement: &str)``
    Evaluate an arbitrary AMPL statement or expression.

``read(&mut self, filename: &str)``
    Read and execute an AMPL model file at ``filename``.

``read_data(&mut self, filename: &str)``
    Read an AMPL data file at ``filename``.

``read_table(&mut self, tablename: &str)``
    Read a table named ``tablename`` into AMPL (equivalent to ``read table tablename;``).

``write_table(&mut self, tablename: &str)``
    Write a table named ``tablename`` from AMPL (equivalent to ``write table tablename;``).

``write(&mut self, filename: &str, auxfiles: &str)``
    Write the model to ``filename`` with auxiliary files listed in ``auxfiles``.

Solving and process control
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

``solve(&self, problem: &str, solver: &str)``
    Solve the current problem, optionally specifying a sub-problem name and a solver. Pass empty strings
    to use the defaults already set in the model/options.

``reset(&mut self)``
    Reset the AMPL interpreter to a clean state, discarding model and data.

``close(&mut self)``
    Close the underlying AMPL process. The instance should not be used afterward.

``is_running(&mut self) -> bool``
    Return ``true`` if the underlying AMPL process is running.

``is_busy(&mut self) -> bool``
    Return ``true`` if AMPL is currently busy (e.g. solving asynchronously).

``interrupt(&mut self)``
    Send an interrupt signal to the running AMPL process.

Exporting model / data / snapshots
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

``snapshot(&mut self, filename: &str, model: bool, data: bool, options: bool) -> String``
    Write a snapshot of the current session to ``filename``. ``model``, ``data``, and ``options`` control
    which parts of the session are included. Returns the snapshot as a string.

``export_model(&mut self, filename: &str) -> String``
    Export the current model to ``filename`` and return it as a string.

``export_data(&mut self, filename: &str) -> String``
    Export the current data to ``filename`` and return it as a string.

Options
~~~~~~~~

``set_option``/``get_option`` (``String``), ``set_bool_option``/``get_bool_option`` (``bool``), ``set_int_option``/``get_int_option`` (``i32``), ``set_dbl_option``/``get_dbl_option`` (``f64``)
    Set or get AMPL options of the corresponding type. The getters return the type's default (empty
    string, ``false``, ``0``, ``0.0``) if the option does not exist.

Data
~~~~~

``set_data(&mut self, df: &DataFrame, set_name: Option<&str>)``
    Assign data from ``df`` to the AMPL entities whose names match the DataFrame's column headers. If
    ``set_name`` is ``Some("S")``, the index column values are also assigned to set ``S``.

``get_data(&mut self, statements: &[&str]) -> DataFrame``
    Retrieve data from AMPL for the given display ``statements`` and return it as a
    :ref:`DataFrame <ref::DataFrame>`. Statements may be parameter/variable names or arbitrary AMPL
    expressions; all statements must be indexable over the same set.

Entity access
~~~~~~~~~~~~~~

``get_current_objective(&mut self) -> String``
    Return the name of the currently active objective, or an empty string if none is set.

``get_variable(&mut self, name: &str) -> Variable`` / ``get_variables(&mut self) -> Vec<Variable>``
    Get the variable with the given AMPL name, or all declared variables.

``get_constraint(&mut self, name: &str) -> Constraint`` / ``get_constraints(&mut self) -> Vec<Constraint>``
    Get the constraint with the given AMPL name, or all declared constraints.

``get_objective(&mut self, name: &str) -> Objective`` / ``get_objectives(&mut self) -> Vec<Objective>``
    Get the objective with the given AMPL name, or all declared objectives.

``get_parameter(&mut self, name: &str) -> Parameter`` / ``get_parameters(&mut self) -> Vec<Parameter>``
    Get the parameter with the given AMPL name, or all declared parameters.

``get_set(&mut self, name: &str) -> Set`` / ``get_sets(&mut self) -> Vec<Set>``
    Get the set with the given AMPL name, or all declared sets.
