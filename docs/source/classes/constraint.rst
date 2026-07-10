.. _ref::Constraint:

Constraint
----------

An AMPL constraint entity. Obtained via ``Ampl::get_constraint`` or ``Ampl::get_constraints``.

Entity-level methods
~~~~~~~~~~~~~~~~~~~~~~

``is_logical(&self) -> bool``
    Return true if this is a logical constraint (as opposed to an algebraic one).

``set_dual(&self, dual: f64)``
    Set the dual value for a scalar constraint instance.

``indexarity(&self) -> usize``
    Number of indices of this constraint.

``num_instances(&self) -> usize``
    Total number of instances (rows) of this constraint.

``get_values(&self) -> DataFrame`` / ``get_values_with(&self, suffixes: &[&str]) -> DataFrame``
    All values (or a selected set of suffixes) of this constraint as a :ref:`DataFrame <ref::DataFrame>`.

``declaration(&mut self) -> String``
    The AMPL declaration string for this constraint.

``drop(&self)`` / ``restore(&self)``
    Drop this constraint from the current model / restore a previously dropped constraint (equivalent to
    the AMPL ``drop``/``restore`` commands).

Getting instances
~~~~~~~~~~~~~~~~~~~

``get_scalar(&self) -> Constraintinstance``
    The instance of this scalar (non-indexed) constraint.

``get(&self, key: &str) -> Constraintinstance``
    The instance indexed by the given string key.

``get_int(&self, key: i64) -> Constraintinstance``
    The instance indexed by the given integer key.

``instances(&self) -> Vec<Constraintinstance>``
    All instances of this constraint as a list.

Constraintinstance
-------------------

A single instance of an AMPL constraint, identified by its indexing tuple. Obtained from ``Constraint``
via ``get_scalar``, ``get``, ``get_int``, or ``instances``.

``name(&self) -> String``
    Fully-qualified AMPL name of this instance (e.g. ``"c['a']"``).

``to_string(&self) -> String``
    Human-readable string representation of this constraint instance.

Numeric suffixes (all ``-> f64`` unless noted)
    ``body``, ``dual``, ``lb``, ``ub``, ``lbs``, ``ubs``, ``ldual``, ``udual``, ``lslack``, ``uslack``,
    ``slack``, ``defvar``, ``dinit``, ``dinit0``, ``val`` (value of a logical constraint).

String suffixes (all ``-> String``)
    ``astatus``, ``sstatus``, ``status``.

``set_dual(&self, value: f64)``
    Set the dual variable value for this constraint instance.

``drop(&self)`` / ``restore(&self)``
    Drop / restore this constraint instance from the active model.

``dbl_suffix(&self, suffix: Numericsuffix) -> f64`` / ``int_suffix(&self, suffix: Numericsuffix) -> i32`` / ``string_suffix(&self, suffix: Stringsuffix) -> String``
    Read an arbitrary numeric/integer/string suffix by :ref:`Numericsuffix or Stringsuffix <ref::suffix>`
    value, for suffixes not covered by a dedicated named method above.
