.. _ref::Variable:

Variable
--------

An AMPL variable entity. Obtained via ``Ampl::get_variable`` or ``Ampl::get_variables``.

Entity-level methods
~~~~~~~~~~~~~~~~~~~~~~

``value(&self) -> f64``
    Current value for a scalar variable.

``fix(&self)``
    Fix a scalar variable at its current value, removing it from the optimisation.

``fix_with_value(&self, value: f64)``
    Fix a scalar variable at the specified ``value``.

``unfix(&self)``
    Unfix a previously fixed variable, allowing it to be optimised again.

``set_value(&self, value: f64)``
    Set the value of a scalar variable.

``indexarity(&self) -> usize``
    Number of indices of this variable.

``num_instances(&self) -> usize``
    Total number of instances of this variable.

``declaration(&mut self) -> String``
    The AMPL declaration string for this variable.

``get_values(&self) -> DataFrame``
    All values and suffixes of this variable as a :ref:`DataFrame <ref::DataFrame>`.

``drop(&self)`` / ``restore(&self)``
    Drop this variable from the current model / restore a previously dropped variable.

Getting instances
~~~~~~~~~~~~~~~~~~~

``get_scalar(&self) -> Variableinstance``
    The instance of this scalar (non-indexed) variable.

``instances(&self) -> Vec<Variableinstance>``
    All instances of this variable as a list.

Variableinstance
------------------

A single instance of an AMPL variable, identified by its indexing tuple. Obtained from ``Variable`` via
``get_scalar`` or ``instances``.

``name(&self) -> String``
    Fully-qualified AMPL name of this instance (e.g. ``"Buy['BEEF']"``).

``to_string(&self) -> String``
    Human-readable string representation of this variable instance.

``drop(&self)`` / ``restore(&self)``
    Drop / restore this variable instance from the active model.

``dbl_suffix(&self, suffix: Numericsuffix) -> f64`` / ``int_suffix(&self, suffix: Numericsuffix) -> i32`` / ``string_suffix(&self, suffix: Stringsuffix) -> String``
    Read an arbitrary numeric/integer/string suffix by :ref:`Numericsuffix or Stringsuffix <ref::suffix>`
    value.

``value(&self) -> f64``
    Current value of this variable instance (``dbl_suffix(Numericsuffix::Value)``).

``dual(&self) -> f64``
    Dual value of this variable instance (``dbl_suffix(Numericsuffix::Dual)``).

``key(&self) -> Vec<Value>``
    The index tuple of this instance as a ``Vec<Value>``.

``fix(&self)`` / ``fix_to_value(&self, value: f64)`` / ``unfix(&self)``
    Fix this variable instance at its current value / at the specified ``value`` / unfix it.

``set_value(&self, value: f64)``
    Set the value of this variable instance.
