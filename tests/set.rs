use amplrs::Ampl;

fn init() -> Ampl {
    let mut ampl = Ampl::new();
    ampl.set_option("solver", "gurobi");
    ampl
}

#[test]
fn scalar_set_basics() {
    let mut ampl = init();
    ampl.eval("set A; data; set A := a b c;");
    let mut a = ampl.get_set("A");
    assert_eq!(0, a.indexarity());
    assert_eq!(1, a.num_instances());
    assert!(a.declaration().contains("set A"));
}

#[test]
fn indexed_set_num_instances() {
    let mut ampl = init();
    ampl.eval("set A; set B{A}; data; set A := 1 2 3;");
    ampl.eval("let B[1] := {\"a\",\"b\"}; let B[2] := {\"c\"}; let B[3] := {\"d\",\"e\",\"f\"};");
    let b = ampl.get_set("B");
    assert_eq!(1, b.indexarity());
    assert_eq!(3, b.num_instances());
}

// AMPL's `drop`/`restore` statements only accept constraint or objective
// names; `Set::drop` builds a `drop <name>;` statement regardless of entity
// kind, which AMPL rejects with a syntax error for a set name.
#[test]
#[should_panic(expected = "syntax error")]
fn drop_is_not_supported_for_sets() {
    let mut ampl = init();
    ampl.eval("set A; data; set A := a b c;");
    let a = ampl.get_set("A");
    a.drop();
}

#[test]
fn get_sets_lists_all_declared_sets() {
    let mut ampl = init();
    ampl.eval("set A; set B; data; set A := a b; set B := 1 2 3;");
    assert_eq!(2, ampl.get_sets().len());
}
