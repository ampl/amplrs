.. _ref::Parameter:

Parameter
---------

An AMPL parameter entity. Obtained via ``Ampl::get_parameter`` or ``Ampl::get_parameters``.

``Parameter`` has no separate instance type - its values are read/written in bulk (see below) or via
``Ampl::get_data``/``Ampl::set_data``, not through per-instance objects.

``indexarity(&self) -> usize``
    Number of indices of this parameter.

``num_instances(&self) -> usize``
    Total number of instances of this parameter.

``declaration(&mut self) -> String``
    The AMPL declaration string for this parameter.

``set_all_double_values(&self, values: &[f64])``
    Assign ``values`` to all instances of this parameter, in the order they appear in the model. The
    length of ``values`` must equal the number of instances.

``set_some_double_values(&self, indices: &[&str], values: &[f64])``
    Assign ``values`` to the specific instances identified by the string ``indices``. Both slices must
    have the same length; each index is treated as a 1-element string tuple.

``drop(&self)`` / ``restore(&self)``
    Drop this parameter from the current model / restore a previously dropped parameter.
