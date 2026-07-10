.. _secClassStructure:

Class structure
================

``amplrs`` consists of a collection of types to interact with AMPL and to
access its inputs and outputs. The structure of these types is explained in this section. For a quick
introduction to AMPL entities see `Quick Introduction to AMPL <https://dev.ampl.com/ampl/introduction.html>`_.

The main type used to interact with AMPL, instantiate and interrogate models is ``Ampl``. One value of
this type represents an execution of the AMPL translator, and is the first thing that has to be created
when developing a solution based on ``amplrs``. It allows interaction with the underlying AMPL translator:
issuing commands, getting diagnostics, and controlling the process.

.. _secAMPLClass:

Ampl type
---------

For all calculations, ``amplrs`` uses an underlying AMPL execution engine, wrapped by ``Ampl``. One value
of this type is the first thing to be created when writing a program that uses ``amplrs``. The underlying
AMPL process is closed automatically when the ``Ampl`` value is dropped (its ``Drop`` implementation calls
the equivalent of ``close()``); there is no separate method to close it early.

All model creation and structural alteration operations are expressed in AMPL language through the
``Ampl`` value; the type also provides access to the current state via the entity types described in
:ref:`secRustAlgebraicEntitiesReference`, and several other functionalities (see :ref:`secReferenceRust`).

The methods can be split into three groups: direct AMPL interaction, model interrogation, and commands.

Direct interaction with AMPL
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

The methods available to send AMPL commands are ``Ampl::eval``, ``Ampl::read``, and ``Ampl::read_data``;
they send the strings specified (or the specified files) to the AMPL engine for interpretation.

Model interrogation
~~~~~~~~~~~~~~~~~~~~

Evaluating AMPL files or statements creates various kinds of entities in the underlying AMPL process. To
get a Rust representation of such entities, there are two main options.

* Get a ``Vec`` of all available entities of one kind, to iterate through them:

  * ``Ampl::get_variables()`` gets all defined variables
  * ``Ampl::get_constraints()`` gets all defined constraints
  * ``Ampl::get_objectives()`` gets all defined objectives
  * ``Ampl::get_sets()`` gets all defined sets
  * ``Ampl::get_parameters()`` gets all defined parameters

* Knowing the AMPL name of an entity, get it directly:

  * ``Ampl::get_variable(name)`` returns the ``Variable`` with the given name
  * ``Ampl::get_constraint(name)`` returns the ``Constraint`` with the given name
  * ``Ampl::get_objective(name)`` returns the ``Objective`` with the given name
  * ``Ampl::get_parameter(name)`` returns the ``Parameter`` with the given name
  * ``Ampl::get_set(name)`` returns the ``Set`` with the given name

Once the desired entities have been obtained, their methods read and manipulate the model, and extract or
assign data. Each call goes through the underlying AMPL process directly (there is no local caching): an
entity value is a lightweight handle (an AMPL entity name plus a pointer back to the ``Ampl`` process), not
a snapshot, so reading e.g. ``variable.value()`` after a ``solve()`` always returns the current AMPL state.

.. _secModellingClasses:

Modelling entity types
------------------------

This group of types represents the basic entities of an AMPL optimization model: variables, constraints,
objectives, parameters, and sets. They are used to access the current state of the AMPL translator (e.g.
to find the values of a variable), and to some extent for data input (e.g. assigning values to a
parameter, fixing a variable).

Values of these types cannot be created directly by the user: model creation and structural modification
is handled in AMPL (see :ref:`secAMPLClass`), through ``Ampl::eval`` and ``Ampl::read``. There is no
common base type/trait; each entity type (``Variable``, ``Constraint``, ``Objective``, ``Parameter``,
``Set``) is a distinct struct obtained from ``Ampl``.

Indexed entities and instances
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

For indexed entities (e.g. a variable indexed over a set in AMPL), a single instance is represented by a
separate "instance" type - the value for one specific index of the indexing set(s). Two entity types
currently support full instance access:

* ``Variable`` / ``Variableinstance``: ``Variable::get_scalar()`` (for a non-indexed ``var x;``),
  ``Variable::instances()`` (all instances, as a ``Vec<Variableinstance>``).
* ``Constraint`` / ``Constraintinstance``: ``Constraint::get_scalar()``, ``Constraint::get(key)`` (string
  index), ``Constraint::get_int(key)`` (integer index), ``Constraint::instances()``.

``Objective`` and ``Set`` currently only expose scalar-style access directly on the entity type itself
(e.g. ``Objective::value()``, ``Objective::astatus()``) - there is presently no public way to obtain a
single instance of an indexed objective or set (the ``Objectiveinstance``/``Setinstance`` types exist, but
their constructors are crate-private and unused). For indexed objectives and sets, use
``Ampl::get_data(&[...])`` or ``Entity::get_values()`` to read all instances at once via a
:ref:`DataFrame <ref::DataFrame>` instead.

``Parameter`` has no separate instance type: its values (typically numbers or strings) are read/written in
bulk through ``Parameter::set_all_double_values``/``set_some_double_values`` or via
``Ampl::get_data``/``Ampl::set_data``, not through per-instance objects.

As an example, ``Variable`` has entity-level functionality like ``Variable::fix()`` (which fixes every
instance) alongside scalar-convenience methods like ``Variable::value()`` for the case where the variable
is not indexed; for indexed variables, per-instance access goes through ``Variableinstance::value()`` and
``Variableinstance::dual()`` (obtained via ``instances()``). ``Constraint`` similarly has entity-level
``drop()``/``restore()`` alongside per-instance ``body()``/``dual()`` on ``Constraintinstance``.

.. note::

    AMPL's ``drop``/``restore`` statements are only meaningful for constraints and objectives. Calling
    ``drop()``/``restore()`` on a ``Variable``, ``Parameter``, or ``Set`` (all of which expose these
    methods too, for API uniformity) will panic with a syntax error from the AMPL interpreter.

.. _secAccessInstancesAndValues:

Access to instances and values
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. code-block:: rust

   use amplrs::Ampl;

   let mut ampl = Ampl::new();
   ampl.eval("var x;");
   let x = ampl.get_variable("x");
   let value = x.value();               // Compact access to a scalar entity
   let value = x.get_scalar().value();  // Access through an explicit instance

Indexed entities are central to modelling in AMPL. ``Variable::instances()`` and ``Constraint::instances()``
return every instance as a ``Vec``; ``Constraint::get(key)``/``get_int(key)`` fetch one instance directly
by its (string or integer) index:

.. code-block:: rust

   use amplrs::Ampl;

   let mut ampl = Ampl::new();
   ampl.eval("var x{1..2, 4..5, 7..8};");
   let x = ampl.get_variable("x");

   // Enumerate all instances
   for inst in x.instances() {
       println!("{} -> {}", inst.name(), inst.value());
   }

The currently defined entities are obtained from the various ``get_*`` methods on ``Ampl`` (see
:ref:`secAMPLClass`). Because an entity value is just a handle back into the running AMPL process, it
always reflects the live state - if a variable is solved after the handle was obtained, reading its value
afterward returns the new, solved value automatically:

.. code-block:: rust

   use amplrs::Ampl;

   let mut ampl = Ampl::new();
   ampl.eval("var x;");
   ampl.eval("maximize z: x;");
   ampl.eval("subject to c: x<=10;");
   let x = ampl.get_variable("x");

   println!("{}", x.value()); // prints 0

   ampl.solve("", "");

   println!("{}", x.value()); // prints 10

Relation between entities and data
------------------------------------

The entities and instances in AMPL store data (numbers or strings) and can be indexed, so the instances
available depend on the values in the indexing set(s). The order in which AMPL enumerates these instances
is not always consistent with the order the data was defined in, so it is often useful, even when
interested only in data decoupled from the AMPL entities, to keep track of which indexing values
correspond to each value.

Every read through an entity handle goes to the live AMPL process, so values are always consistent with
the current AMPL state - at the cost of one round-trip per access. For example, reading the value of 1000
instances one by one:

.. code-block:: rust

  use amplrs::Ampl;

  let mut ampl = Ampl::new();
  ampl.eval("set A := 1..1000; param c{i in A} default 0; var x{i in 1..1000} := c[i];");

  let x = ampl.get_variable("x");
  for inst in x.instances() {
      println!("{}", inst.value());
  }

incurs a round-trip per instance. To avoid this, use a :ref:`DataFrame <ref::DataFrame>`, whose usage is
two-fold:

* It allows definition of data for multiple parameters in one single call to the underlying interpreter.
* It decouples data from entities, avoiding per-instance round-trips.

``DataFrame`` should be used together with ``Ampl::set_data()`` and ``Entity::get_values()``:

.. code-block:: rust

  use amplrs::{Ampl, DataFrame};

  // Create a new DataFrame with one index column ("A") and one data column ("c")
  let df = DataFrame::new(1, 1, &["A", "c"]);
  let indices: Vec<String> = (1..=1000).map(|i| i.to_string()).collect();
  let index_refs: Vec<&str> = indices.iter().map(String::as_str).collect();
  let values: Vec<f64> = (1..=1000).map(|i| i as f64 * 1.1).collect();
  df.set_array(&index_refs, &values);

  let mut ampl = Ampl::new();
  ampl.eval("set A; param c{i in A} default 0; var x{i in A} := c[i];");
  // Assign data to the set A and the parameter c in one call
  ampl.set_data(&df, Some("A"));

  let x = ampl.get_variable("x");
  // df is decoupled from the AMPL process from this point on
  let df = x.get_values();

  for row in df.rows() {
      // row[0] is the "A" index (a string here - set_array always writes
      // string-typed set members), row[1] is "x.val" (numeric)
      println!("{} -> {}", row[0].as_str().unwrap(), row[1].as_f64().unwrap());
  }

The underlying AMPL interpreter does not need to be running while a ``DataFrame`` value is used on its
own, but it maintains all the correspondence between indexing set and actual value of the instances.

.. _secAccessToScalars:

Access to scalar values
~~~~~~~~~~~~~~~~~~~~~~~~~

Simplified access to scalar values - the value of a scalar variable or parameter, or in general any AMPL
expression that evaluates to a single number - is possible using ``Ampl::get_data`` with a single-cell
result, or the more targeted per-entity accessors (e.g. ``Variable::value()``). ``amplrs`` does not
currently expose a dedicated "evaluate to a single scalar" convenience method like some other AMPL APIs
do; for one-off scalar AMPL expressions, wrap them in ``get_data`` and read the single resulting cell:

.. code-block:: rust

  use amplrs::Ampl;

  let mut ampl = Ampl::new();
  ampl.eval("var x{i in 1..3} := i;");
  let df = ampl.get_data(&["x[2]"]);
  println!("{}", df.get_row_by_index(0)[0].as_f64().unwrap()); // prints 2
