.. _ref::Objective:

Objective
---------

An AMPL objective entity. Obtained via ``Ampl::get_objective`` or ``Ampl::get_objectives``.

Unlike :ref:`Variable <ref::Variable>` and :ref:`Constraint <ref::Constraint>`, ``Objective`` currently
has no ``instances()``/``get()``/``get_scalar()`` methods - see the note in :ref:`secClassStructure`. All
value-access methods below apply directly to the (scalar) entity.

``indexarity(&self) -> usize``
    Number of indices of this objective.

``num_instances(&self) -> usize``
    Total number of instances of this objective.

``declaration(&mut self) -> String``
    The AMPL declaration string for this objective.

``drop(&self)`` / ``restore(&self)``
    Drop this objective from the current model / restore a previously dropped objective.

``value(&self) -> f64``
    Current objective value (``.val`` suffix) for a scalar objective.

``astatus(&self) -> String``
    Algebraic status (``.astatus``) of this objective.

``sstatus(&self) -> String``
    Solver status (``.sstatus``) of this objective.

``exitcode(&self) -> i32``
    Solver exit code (``.exitcode``) for this objective.

``message(&self) -> String``
    Solver message (``.message``) for this objective.

``result(&self) -> String``
    Solve result string (``.result``) for this objective.

``is_minimization(&self) -> bool``
    Return ``true`` if this objective is declared as a minimization.

Objectiveinstance
-------------------

A single instance of an AMPL objective, identified by its indexing tuple. The type exists in the crate's
public API, but there is currently no way to obtain one (its constructor is crate-private) - see the note
in :ref:`secClassStructure`. For reference, once obtained it exposes:

``name(&self) -> String``
    Fully-qualified AMPL name of this instance.

``to_string(&self) -> String``
    Human-readable string representation of this objective instance.

``drop(&self)`` / ``restore(&self)``
    Drop / restore this objective instance from the active model.

``dbl_suffix(&self, suffix: Numericsuffix) -> f64`` / ``int_suffix(&self, suffix: Numericsuffix) -> i32`` / ``string_suffix(&self, suffix: Stringsuffix) -> String``
    Read an arbitrary numeric/integer/string suffix by :ref:`Numericsuffix or Stringsuffix <ref::suffix>`
    value.
