.. _secRustQuickStart:

Quick start
===========

This section shows a simple example to illustrate various functionalities of the ``amplrs`` interface.
The full example loads a model and its data programmatically via :ref:`DataFrame <ref::DataFrame>`
objects, solves it, gets some of the AMPL entities in Rust, and uses them to read results and assign
data programmatically.

This section assumes you are already familiar with the Rust language. Data can be loaded from various
sources: table handlers, data files, and ``DataFrame``\ s built from native Rust slices/vectors. Full
class reference is given in :ref:`secReferenceRust`.

Complete listing
----------------

This is the complete listing of the example. You can find it at
:download:`examples/firstexample.rs <../../examples/firstexample.rs>`, and a version that builds the
whole model's data programmatically (instead of reading it from a ``.dat`` file) at
:download:`examples/dietexample.rs <../../examples/dietexample.rs>`. For clarity of presentation, the
code below omits handling of the panics that ``amplrs`` raises on AMPL errors.

.. code-block:: rust

    use amplrs::Ampl;

    fn main() {
        let mut ampl = Ampl::new();
        ampl.set_option("solver", "highs");

        // Load the model (alternatively, ampl.eval("""...""") defines a model from a string)
        ampl.read("models/diet/diet.mod");
        ampl.read_data("models/diet/diet.dat");

        ampl.solve("", "");

        // Get objective entity by AMPL name
        let total_cost = ampl.get_objective("Total_Cost");
        println!("Objective is: {}", total_cost.value());

        // Reassign data - specific instances
        let cost = ampl.get_parameter("cost");
        cost.set_some_double_values(&["BEEF", "HAM"], &[5.01, 4.55]);
        println!("Increased costs of beef and ham.");

        ampl.solve("", "");
        println!("New objective value: {}", ampl.get_objective("Total_Cost").value());

        // Reassign data - all instances
        cost.set_all_double_values(&[3.0, 5.0, 5.0, 6.0, 1.0, 2.0, 5.01, 4.55]);
        println!("Updated all costs.");

        ampl.solve("", "");
        println!("New objective value: {}", ampl.get_objective("Total_Cost").value());

        // Get the values of the variable Buy in a DataFrame
        let buy = ampl.get_variable("Buy");
        let df = buy.get_values();
        println!("{}", df.to_string());

        // Get the values of an expression into a DataFrame
        let df2 = ampl.get_data(&["{j in FOOD} 100*Buy[j]/Buy[j].ub"]);
        println!("{}", df2.to_string());
    }

The output of this program is::

    Objective is: 118.05940323955669
    Increased costs of beef and ham.
    New objective value: 144.41572037510653
    Updated all costs.
    New objective value: 164.54375000000002

    index0    |   Buy.val
     'BEEF'   |     10
     'CHK'    |      2
     ...

Needed crates and AMPL instance creation
------------------------------------------

For a simple hello-world program, first bring ``Ampl`` into scope:

.. code-block:: rust

  use amplrs::Ampl;

Then create an ``Ampl`` instance and get the value of the option ``version`` as defined by the underlying
AMPL executable:

.. code-block:: rust

   let mut ampl = Ampl::new();
   println!("{}", ampl.get_option("version"));

``Ampl::new()`` creates a new AMPL instance with all default settings, spawning the ``ampl`` executable
found on ``PATH``. ``get_option`` is the preferred way to access AMPL options: it gets the value of the
option ``version`` from AMPL as a string and prints it.

If your AMPL installation directory is not on the system search path, use ``Environment`` as described in
:ref:`lblGettingStarted`.

Load model and data from files
-------------------------------

If you have AMPL model and data files, use ``Ampl::read`` to load model files and ``Ampl::read_data`` to
load data files. If the files are not found, the call panics with the AMPL "file not found" error.

.. code-block:: rust

   ampl.read("models/diet.mod");
   ampl.read_data("models/diet.dat");

Once these commands are executed, the AMPL interpreter will have interpreted the content of the two
files. No further communication is made between the AMPL interpreter and the Rust program until an
entity's properties are accessed, as entities are created lazily (as needed).

Load model using eval
----------------------

An alternative to ``Ampl::read`` for loading models is ``Ampl::eval``, which loads a model directly from
a string:

.. code-block:: rust

    ampl.eval(r#"
        set NUTR;
        set FOOD;

        param cost {FOOD} > 0;
        param f_min {FOOD} >= 0;
        param f_max {j in FOOD} >= f_min[j];

        param n_min {NUTR} >= 0;
        param n_max {i in NUTR} >= n_min[i];

        param amt {NUTR,FOOD} >= 0;

        var Buy {j in FOOD} >= f_min[j], <= f_max[j];

        minimize Total_Cost:  sum {j in FOOD} cost[j] * Buy[j];

        subject to Diet {i in NUTR}:
        n_min[i] <= sum {j in FOOD} amt[i,j] * Buy[j] <= n_max[i];
    "#);

Using ``Ampl::eval`` or ``Ampl::read`` to load a model is a matter of preference.

Load the data using DataFrames
--------------------------------

Data can be loaded in various ways; one of them is building :ref:`DataFrame <ref::DataFrame>` objects
directly from Rust slices, as ``dietexample.rs`` does:

.. code-block:: rust

    use amplrs::DataFrame;

    let foods = ["BEEF", "CHK", "FISH", "HAM", "MCH", "MTL", "SPG", "TUR"];
    let costs = [3.59, 2.59, 2.29, 2.89, 1.89, 1.99, 1.99, 2.49];
    let fmin  = [2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0, 2.0];
    let fmax  = [10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0, 10.0];

    // 1 index column ("FOOD"), 3 data columns
    let df_food = DataFrame::new(1, 3, &["FOOD", "cost", "f_min", "f_max"]);
    df_food.set_column_strings("FOOD", &foods);
    df_food.set_column_doubles("cost", &costs);
    df_food.set_column_doubles("f_min", &fmin);
    df_food.set_column_doubles("f_max", &fmax);
    // Send the data to AMPL and initialize the indexing set "FOOD" from the index column
    ampl.set_data(&df_food, Some("FOOD"));

Two-dimensional data (e.g. the ``amt`` matrix, indexed over both ``NUTR`` and ``FOOD``) can be built with
``DataFrame::set_matrix_doubles``:

.. code-block:: rust

    let nutrients = ["A", "C", "B1", "B2", "NA", "CAL"];
    #[rustfmt::skip]
    let amounts: &[f64] = &[
         60.0,    8.0,   8.0,  40.0,   15.0,   70.0,   25.0,   60.0,  // A
         20.0,    0.0,  10.0,  40.0,   35.0,   30.0,   50.0,   20.0,  // C
         // ... one row per nutrient, one column per food, row-major
    ];

    let df_amt = DataFrame::new(2, 1, &["NUTR", "FOOD", "amt"]);
    df_amt.set_matrix_doubles(&nutrients, &foods, amounts);
    ampl.set_data(&df_amt, None);

See :download:`examples/dietexample.rs <../../examples/dietexample.rs>` for the complete, runnable
version of this snippet.

Solve a problem
-----------------

To solve the currently loaded problem instance:

.. code-block:: rust

   // Specify the solver to use (e.g. HiGHS)
   ampl.set_option("solver", "highs");

   // Solve the problem
   ampl.solve("", "");

``Ampl::solve`` takes an optional problem name and an optional solver override; pass empty strings to use
the defaults already configured on the model/options.

Get an AMPL entity in the programming environment (get objective value)
--------------------------------------------------------------------------

``amplrs`` provides Rust representations of the AMPL entities. The generic procedure is:

1. Identify the entities that need interaction (either data read or modification).
2. For each of these entities, get the entity through the API using one of ``Ampl::get_variable``,
   ``Ampl::get_constraint``, ``Ampl::get_objective``, ``Ampl::get_parameter``, or ``Ampl::get_set``.

.. code-block:: rust

    let total_cost = ampl.get_objective("Total_Cost");
    println!("Objective is: {}", total_cost.value());

For scalar entities (declared with e.g. ``minimize Total_Cost: ...;``, which has exactly one instance),
the value-access methods live directly on the entity type itself - there is no separate "get the single
instance" step needed. The output of the snippet above is::

   Objective is: 118.05940323955669

The same is true for all other entities.

Modify model data (assign values to parameters)
--------------------------------------------------

The input data of an optimization model is stored in its parameters; these can be scalar or vectorial
entities. Two ways are provided to change the value of a vectorial parameter: change specific values, or
change all values at once, via ``Parameter::set_some_double_values`` and
``Parameter::set_all_double_values`` respectively.

.. code-block:: rust

   let cost = ampl.get_parameter("cost");
   cost.set_some_double_values(&["BEEF", "HAM"], &[5.01, 4.55]);
   println!("Increased costs of beef and ham.");
   ampl.solve("", "");
   println!("New objective value: {}", total_cost.value());

The code above assigns the values 5.01 and 4.55 to the parameter ``cost`` for BEEF and HAM respectively.
If the order of the indexing of an entity is known, ``set_all_double_values`` reassigns every instance at
once, in the order they are represented in AMPL:

.. code-block:: rust

   cost.set_all_double_values(&[3.0, 5.0, 5.0, 6.0, 1.0, 2.0, 5.01, 4.55]);
   println!("Updated all costs.");
   ampl.solve("", "");
   println!("New objective value: {}", total_cost.value());

The statements above produce the following output::

   Objective is: 118.05940323955669
   Increased costs of beef and ham.
   New objective value: 144.41572037510653
   Updated all costs.
   New objective value: 164.54375000000002

Get numeric values from variables
------------------------------------

To access all the numeric values contained in a ``Variable`` or any other entity, use
``Entity::get_values()``, which returns a :ref:`DataFrame <ref::DataFrame>`. Doing so decouples the data
from the entity, at a considerable performance gain over reading values instance by instance.

.. code-block:: rust

   let buy = ampl.get_variable("Buy");
   let df = buy.get_values();
   println!("{}", df.to_string());

Get arbitrary values via AMPL expressions
---------------------------------------------

Often we are interested in very specific values coming out of the optimization session. To use the power
of AMPL expressions without cluttering up the environment with new entities, fetch data through arbitrary
AMPL expressions with ``Ampl::get_data``. For the diet model, we can find out how close each decision
variable is to its upper bound, in percent:

.. code-block:: rust

  let df2 = ampl.get_data(&["{j in FOOD} 100*Buy[j]/Buy[j].ub"]);
  println!("{}", df2.to_string());
