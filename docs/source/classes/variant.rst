.. _ref::Variant:

Variant
-------

A scalar value that is either numeric (``f64``) or a string, mirroring the AMPL type system.

``Variant::new() -> Self``
    Create an empty (unset) variant.

``Variant::new_from_string(value: &str) -> Self``
    Create a string-valued variant.

``Variant::new_from_double(value: f64) -> Self``
    Create a numeric-valued variant from a ``f64``.

``get_numeric(&self) -> f64``
    The numeric value. The result is unspecified if this variant holds a string.

``get_string(&self) -> String``
    The string value. Returns an empty string if the value is null or numeric.

``format(&self) -> String``
    The AMPL-formatted string representation of this variant (e.g. a string variant ``"hello"`` formats
    as ``'hello'``).

Tuple
-----

A tuple of AMPL variants used to index into multi-dimensional entities.

Tuples are produced by the AMPL engine (e.g. when iterating instances, see
``Variableinstance::key()``/``Setinstance::contains()``) and are passed back to API calls that require an
instance index.

``Tuple::new(raw: *mut ffi::AMPL_TUPLE) -> Self``
    Wrap an existing raw AMPL tuple pointer. The caller retains ownership. This takes a raw FFI pointer
    from ``amplrs::ffi``, so it is mainly useful for advanced/internal use rather than everyday model code
    - most application code only ever receives ``Tuple``\ s back from the API (e.g. via
    ``Setinstance::contains``), rather than constructing them directly.
