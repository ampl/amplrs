.. _ref::Set:

Set
---

An AMPL set entity. Obtained via ``Ampl::get_set`` or ``Ampl::get_sets``.

Like :ref:`Objective <ref::Objective>`, ``Set`` currently has no ``instances()``/``get()``/``get_scalar()``
methods - see the note in :ref:`secClassStructure`.

``indexarity(&self) -> usize``
    Number of indices of this set.

``num_instances(&self) -> usize``
    Total number of instances of this set.

``declaration(&mut self) -> String``
    The AMPL declaration string for this set.

``drop(&self)`` / ``restore(&self)``
    Drop this set from the current model / restore a previously dropped set.

Setinstance
------------

A single instance of an AMPL set, identified by its indexing tuple. The type exists in the crate's
public API, but there is currently no way to obtain one (its constructor is crate-private) - see the note
in :ref:`secClassStructure`. For reference, once obtained it exposes:

``name(&self) -> String``
    Fully-qualified AMPL name of this instance.

``to_string(&self) -> String``
    Human-readable string representation of this set instance.

``drop(&self)`` / ``restore(&self)``
    Drop / restore this set instance from the active model.

``dbl_suffix(&self, suffix: Numericsuffix) -> f64`` / ``int_suffix(&self, suffix: Numericsuffix) -> i32`` / ``string_suffix(&self, suffix: Stringsuffix) -> String``
    Read an arbitrary numeric/integer/string suffix by :ref:`Numericsuffix or Stringsuffix <ref::suffix>`
    value.

``size(&self) -> usize``
    Number of elements in this set instance.

``contains(&self, contained: Tuple) -> bool``
    Return ``true`` if the set instance contains the element described by ``contained``.
