.. _secReferenceRust:

API Reference
=============

All types provided by ``amplrs`` are re-exported at the crate root (``amplrs::Ampl``,
``amplrs::DataFrame``, ...), except the suffix enums which live under ``amplrs::suffix`` (see
:ref:`ref::suffix`). For brevity, only the type name is used below.

This reference is hand-maintained from the ``///`` doc comments in the crate's source; for the most
up-to-date and complete listing (including private-field layouts and trait impls), also consider running
``cargo doc --open`` against the crate itself.

Core types
----------

.. toctree::
   :maxdepth: 2

   classes/ampl
   classes/dataframe
   classes/environment
   classes/variant
   classes/suffix

.. _secRustAlgebraicEntitiesReference:

Algebraic entity types
------------------------

.. toctree::
   :maxdepth: 2

   classes/variable
   classes/constraint
   classes/objective
   classes/set
   classes/parameter

Error handling
--------------

``amplrs`` does not use a dedicated exception/error-handler type. Every method that talks to the AMPL
interpreter checks the resulting ``AMPL_ERRORINFO`` (or, for a few plain-``int``-returning C API calls,
the status code) and **panics** with the AMPL-provided message on failure - there is no ``Result`` to
match on. Wrap calls in `std::panic::catch_unwind <https://doc.rust-lang.org/std/panic/fn.catch_unwind.html>`_
if you need to recover from an AMPL-side error without aborting the whole program (`amplrs::error::catch_ampl_error`
does exactly this, suppressing the default panic hook while it runs).
