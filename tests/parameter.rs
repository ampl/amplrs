use amplrs::{Ampl, Value};

fn init() -> Ampl {
    let mut ampl = Ampl::new();
    ampl.set_option("solver", "gurobi");
    ampl
}

#[test]
fn indexarity_and_num_instances() {
    let mut ampl = init();
    ampl.eval("set A; param p{A}; data; set A := a b c;");
    let mut p = ampl.get_parameter("p");
    assert_eq!(1, p.indexarity());
    assert_eq!(3, p.num_instances());
    assert!(p.declaration().contains("param p"));
}

#[test]
fn scalar_parameter_num_instances() {
    let mut ampl = init();
    ampl.eval("param q := 5;");
    let q = ampl.get_parameter("q");
    assert_eq!(0, q.indexarity());
    assert_eq!(1, q.num_instances());
}

#[test]
fn set_all_double_values() {
    let mut ampl = init();
    ampl.eval("set A; param p{A}; data; set A := a b c;");
    let p = ampl.get_parameter("p");
    p.set_all_double_values(&[10.0, 20.0, 30.0]);

    let df = ampl.get_data(&["p"]);
    let total: f64 = df.get_column("p").iter().map(|v| v.as_f64().unwrap()).sum();
    assert_eq!(60.0, total);
}

#[test]
fn set_some_double_values() {
    let mut ampl = init();
    ampl.eval("set A; param p{A} default 0; data; set A := a b c;");
    let p = ampl.get_parameter("p");
    p.set_some_double_values(&["b"], &[42.0]);

    // Only "b" now has an explicitly stored value; "a" and "c" still fall
    // back to the `default 0` and so are not enumerated by get_data.
    let df = ampl.get_data(&["p"]);
    assert_eq!(1, df.num_rows());
    let row = df.get_row(&[Value::Text("b".to_string())]);
    assert_eq!(42.0, row[1].as_f64().unwrap());
}

// AMPL's `drop`/`restore` statements only accept constraint or objective
// names; `Parameter::drop` builds a `drop <name>;` statement regardless of
// entity kind, which AMPL rejects with a syntax error for a parameter name.
#[test]
#[should_panic(expected = "syntax error")]
fn drop_is_not_supported_for_parameters() {
    let mut ampl = init();
    ampl.eval("param q := 5;");
    let q = ampl.get_parameter("q");
    q.drop();
}

#[test]
fn get_parameters_lists_all_declared_parameters() {
    let mut ampl = init();
    ampl.eval("param a := 1; param b := 2; param c := 3;");
    assert_eq!(3, ampl.get_parameters().len());
}
