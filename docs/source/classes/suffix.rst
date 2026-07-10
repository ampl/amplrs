.. _ref::suffix:

Numericsuffix / Stringsuffix
------------------------------

Unlike the other types in this reference, these two enums are **not** re-exported at the crate root - they
are accessed as ``amplrs::suffix::Numericsuffix`` and ``amplrs::suffix::Stringsuffix``.

They are used with the ``dbl_suffix``/``int_suffix``/``string_suffix`` methods on the instance types
(``Variableinstance``, ``Constraintinstance``, ``Objectiveinstance``, ``Setinstance``) to read a suffix
not already covered by a dedicated named method (e.g. ``Constraintinstance::dual()``).

Numericsuffix
~~~~~~~~~~~~~~~

Numeric suffixes that AMPL attaches to variables, constraints, and objectives:

``Value`` (``.val``), ``Defeqn`` (``.defeqn``), ``Dual`` (``.dual``), ``Init`` (``.init``),
``Init0`` (``.init0``), ``Lb`` (``.lb``), ``Ub`` (``.ub``), ``Lb0`` (``.lb0``), ``Ub0`` (``.ub0``),
``Lb1`` (``.lb1``), ``Ub1`` (``.ub1``), ``Lb2`` (``.lb2``), ``Ub2`` (``.ub2``), ``Lrc`` (``.lrc``),
``Urc`` (``.urc``), ``Lslack`` (``.lslack``), ``Uslack`` (``.uslack``), ``Rc`` (``.rc``),
``Slack`` (``.slack``) - general suffixes; ``Body`` (``.body``), ``Defvar`` (``.defvar``),
``Dinit`` (``.dinit``), ``Dinit0`` (``.dinit0``), ``Lbs`` (``.lbs``), ``Ubs`` (``.ubs``),
``Ldual`` (``.ldual``), ``Udual`` (``.udual``), ``Val`` (logical constraint value, ``.val``) -
constraint-specific; ``Exitcode`` (solver exit code, ``.exitcode``) - objective-specific;
``Unknown(ffi::AMPL_NUMERICSUFFIX)`` - any numeric suffix not explicitly listed above.

Stringsuffix
~~~~~~~~~~~~~~

String suffixes that AMPL attaches to variables, constraints, and objectives:

``Astatus`` (algebraic status, ``.astatus``), ``Sstatus`` (solver status, ``.sstatus``),
``Status`` (combined status, ``.status``), ``Message`` (solver message, ``.message``),
``Result`` (solve result string, ``.result``), ``Sense`` (objective sense, ``"minimize"`` or
``"maximize"``, ``.sense``), ``Unknown(ffi::AMPL_STRINGSUFFIX)`` - any string suffix not explicitly
listed above.
