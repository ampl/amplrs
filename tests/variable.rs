use amplrs::Ampl;

fn init() -> Ampl {
    let mut ampl = Ampl::new();
    ampl.set_option("solver", "gurobi");
    ampl
}

#[test]
fn scalar_variable_fix_and_set_value() {
    let mut ampl = init();
    ampl.eval("var x >= 0 <= 10; maximize z: x;");
    let x = ampl.get_variable("x");
    assert_eq!(0, x.indexarity());
    assert_eq!(1, x.num_instances());

    x.set_value(4.0);
    assert_eq!(4.0, x.value());

    x.fix();
    ampl.solve("", "");
    assert_eq!(4.0, x.value());

    x.unfix();
    ampl.solve("", "");
    assert_eq!(10.0, x.value());

    x.fix_with_value(2.0);
    ampl.solve("", "");
    assert_eq!(2.0, x.value());
}

#[test]
fn declaration_contains_variable_name() {
    let mut ampl = init();
    ampl.eval("var y >= 0;");
    let mut y = ampl.get_variable("y");
    assert!(y.declaration().contains("var y"));
}

#[test]
fn indexed_variable_instances_and_get_values() {
    let mut ampl = init();
    ampl.eval("var x{1..3} >= 0 <= 10; maximize z: sum{i in 1..3} x[i];");
    let x = ampl.get_variable("x");
    assert_eq!(1, x.indexarity());
    assert_eq!(3, x.num_instances());
    assert_eq!(3, x.instances().len());

    ampl.solve("", "");
    let df = x.get_values();
    assert_eq!(3, df.num_rows());
    let total: f64 = df.get_column("x.val").iter().map(|v| v.as_f64().unwrap()).sum();
    assert_eq!(30.0, total);
}

#[test]
fn string_indexed_instance_key() {
    let mut ampl = init();
    ampl.eval("set A; var x{A} >= 0 <= 10; maximize z: sum{i in A} x[i]; data; set A := beef chk fish;");
    let x = ampl.get_variable("x");
    ampl.solve("", "");

    let mut keys: Vec<String> = x.instances().iter()
        .map(|inst| inst.key()[0].as_str().unwrap().to_string())
        .collect();
    keys.sort();
    assert_eq!(vec!["beef".to_string(), "chk".to_string(), "fish".to_string()], keys);
}

#[test]
fn get_scalar_matches_only_instance() {
    let mut ampl = init();
    ampl.eval("var x >= 0 <= 7; maximize z: x;");
    let x = ampl.get_variable("x");
    ampl.solve("", "");
    assert_eq!(x.value(), x.get_scalar().value());
}

// AMPL's `drop`/`restore` statements only accept constraint or objective
// names; `Variable::drop` builds a `drop <name>;` statement regardless of
// entity kind, which AMPL rejects with a syntax error for a variable name.
#[test]
#[should_panic(expected = "syntax error")]
fn drop_is_not_supported_for_variables() {
    let mut ampl = init();
    ampl.eval("var x >= 0;");
    let x = ampl.get_variable("x");
    x.drop();
}

#[test]
fn get_variables_lists_all_declared_variables() {
    let mut ampl = init();
    ampl.eval("var a; var b; var c;");
    assert_eq!(3, ampl.get_variables().len());
}
