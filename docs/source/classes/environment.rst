.. _ref::Environment:

Environment
-----------

Describes the location of the AMPL binary used when creating an :ref:`Ampl <ref::Ampl>` instance.

Pass an ``Environment`` to ``Ampl::new_with_env`` when the AMPL installation directory is not on the
system ``PATH``.

``Environment::new(bin_dir: &str, bin_name: &str) -> Self``
    Create an environment pointing to the AMPL binary in ``bin_dir`` with executable name ``bin_name``.

``clone(&self) -> Self``
    Return a shallow copy sharing the same underlying environment pointer.

``add_environment_variable(&self, name: &str, value: &str)``
    Add an environment variable ``name=value`` that is passed to the AMPL process.

``get_bin_dir(&self) -> String`` / ``set_bin_dir(&self, bin_dir: &str)``
    Get/set the directory in which AMPL looks for the binary.

``get_bin_name(&self) -> String`` / ``set_bin_name(&self, bin_name: &str)``
    Get/set the name of the AMPL executable (without directory).

``to_string(&self) -> String``
    Human-readable string representation of the environment configuration.

``size(&self) -> usize``
    Number of environment variables registered with this environment.
