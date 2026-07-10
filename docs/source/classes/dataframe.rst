.. _ref::DataFrame:

DataFrame
---------

An AMPL DataFrame for passing tabular data between Rust and AMPL.

A DataFrame has zero or more *index* columns (used to key rows) followed by zero or more *data* columns.
The total column count is ``num_indices() + num_data_cols``. Cell values are represented by the
``Value`` enum (``Value::Numeric(f64)`` or ``Value::Text(String)``), with ``as_f64()``/``as_str()``
accessors to get back the underlying number/string (returning ``None`` for the other variant).

Construction and shape
~~~~~~~~~~~~~~~~~~~~~~~~

``DataFrame::new(num_index_cols: usize, num_data_cols: usize, headers: &[&str]) -> Self``
    Create a new DataFrame with ``num_index_cols`` index columns and ``num_data_cols`` data columns.
    ``headers`` must contain exactly ``num_index_cols + num_data_cols`` entries, in order.

``num_rows(&self) -> usize`` / ``num_cols(&self) -> usize`` / ``num_indices(&self) -> usize``
    Number of data rows / total columns (index + data) / index columns.

``headers(&self) -> Vec<String>``
    Column headers in order (index columns first, then data columns).

``reserve(&self, n: usize)``
    Pre-allocate space for ``n`` rows. Rows still must be added one by one via ``add_row``.

Adding rows
~~~~~~~~~~~~

``add_row(&self, values: &[Value])``
    Append a row of mixed-type values. The number of values must equal ``num_cols()``.

``add_row_doubles(&self, values: &[f64])`` / ``add_row_strings(&self, values: &[&str])``
    Append an all-numeric or all-string row.

Adding and setting columns
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

``add_empty_column(&self, header: &str)``
    Append a new empty data column with the given header.

``add_column_doubles(&self, header: &str, values: &[f64])`` / ``add_column_strings(&self, header: &str, values: &[&str])``
    Append a new data column named ``header`` with the given values. The number of rows must already be
    established; ``values.len()`` must equal the current row count.

``set_column_doubles(&self, header: &str, values: &[f64])`` / ``set_column_strings(&self, header: &str, values: &[&str])``
    Overwrite an entire column identified by ``header`` with the given values.

Reading and writing values
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

``get_value(&self, row: usize, col: usize) -> Value`` / ``set_value(&self, row: usize, col: usize, value: &Value)``
    Get/set the value at the given 0-based ``(row, col)`` position.

``get_row_by_index(&self, index: usize) -> Vec<Value>``
    All values in row ``index`` (all columns, left to right).

``get_column(&self, header: &str) -> Vec<Value>``
    All values in the column named ``header``, one entry per row.

``find_row(&self, key: &[Value]) -> Option<usize>``
    Find the 0-based row position for the index tuple ``key``; ``None`` if no row with that key exists.

``get_row(&self, key: &[Value]) -> Vec<Value>``
    All values in the row identified by the index tuple ``key``. Panics if no row with that key exists.

``set_value_at(&self, key: &[Value], header: &str, value: &Value)``
    Set the value in the column named ``header`` at the row identified by the index ``key`` (one
    ``Value`` per index column).

``rows(&self) -> impl Iterator<Item = Vec<Value>>``
    Iterate over all rows, each as a ``Vec<Value>`` of all column values.

Bulk fill helpers
~~~~~~~~~~~~~~~~~~~

``set_array(&self, indices: &[&str], values: &[f64])``
    Fill a 1-index / 1-data-column DataFrame from parallel slices of string indices and ``f64`` values.

``set_matrix_doubles(&self, row_indices: &[&str], col_indices: &[&str], values: &[f64])`` / ``set_matrix_strings(&self, row_indices: &[&str], col_indices: &[&str], values: &[&str])``
    Fill a 2-index / 1-data-column DataFrame from parallel slices of string row/column indices and a flat
    row-major values array. ``values.len()`` must equal ``row_indices.len() * col_indices.len()``.

Other
~~~~~~

``to_string(&self) -> String``
    Human-readable tabular string representation of the DataFrame.

``PartialEq``
    Two DataFrames compare equal if they have identical structure and contents.

.. note::

    Rows produced via the string-taking constructors (``set_array``, ``set_matrix_strings``,
    ``add_row_strings``, ``set_column_strings``, ...) always store their index/data values as
    ``Value::Text``, even when the text looks numeric (e.g. index ``"1"`` stays a string, not
    ``Value::Numeric(1.0)``). Use ``add_row``/``add_row_doubles``/``set_matrix_doubles`` if you need
    genuinely numeric index or data values.
