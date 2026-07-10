use amplrs::{Ampl, DataFrame, Value};

fn init() -> Ampl {
    let mut ampl = Ampl::new();
    ampl.set_option("solver", "gurobi");
    ampl
}

#[test]
fn new_dataframe_is_empty_with_given_headers() {
    let df = DataFrame::new(1, 1, &["food", "cost"]);
    assert_eq!(1, df.num_indices());
    assert_eq!(2, df.num_cols());
    assert_eq!(0, df.num_rows());
    assert_eq!(vec!["food".to_string(), "cost".to_string()], df.headers());
}

#[test]
fn reserve_does_not_add_rows() {
    let df = DataFrame::new(1, 1, &["food", "cost"]);
    df.reserve(10);
    assert_eq!(0, df.num_rows());
}

#[test]
fn add_row_mixed_values_and_get_row_by_index() {
    let df = DataFrame::new(1, 1, &["food", "cost"]);
    df.add_row(&[Value::Text("BEEF".to_string()), Value::Numeric(3.19)]);
    df.add_row(&[Value::Text("CHK".to_string()), Value::Numeric(2.59)]);
    assert_eq!(2, df.num_rows());

    let row0 = df.get_row_by_index(0);
    assert_eq!("BEEF", row0[0].as_str().unwrap());
    assert_eq!(3.19, row0[1].as_f64().unwrap());

    let row1 = df.get_row_by_index(1);
    assert_eq!("CHK", row1[0].as_str().unwrap());
    assert_eq!(2.59, row1[1].as_f64().unwrap());
}

#[test]
fn add_row_doubles_and_strings() {
    let df = DataFrame::new(1, 1, &["i", "v"]);
    df.add_row_doubles(&[1.0, 2.0]);
    assert_eq!(1.0, df.get_value(0, 0).as_f64().unwrap());
    assert_eq!(2.0, df.get_value(0, 1).as_f64().unwrap());

    let df2 = DataFrame::new(1, 1, &["k", "s"]);
    df2.add_row_strings(&["a", "b"]);
    assert_eq!("a", df2.get_value(0, 0).as_str().unwrap());
    assert_eq!("b", df2.get_value(0, 1).as_str().unwrap());
}

#[test]
fn find_row_and_get_row_by_key() {
    let df = DataFrame::new(1, 1, &["food", "cost"]);
    df.add_row(&[Value::Text("BEEF".to_string()), Value::Numeric(3.19)]);
    df.add_row(&[Value::Text("CHK".to_string()), Value::Numeric(2.59)]);

    assert_eq!(Some(1), df.find_row(&[Value::Text("CHK".to_string())]));
    assert_eq!(None, df.find_row(&[Value::Text("NOPE".to_string())]));

    let row = df.get_row(&[Value::Text("BEEF".to_string())]);
    assert_eq!(3.19, row[1].as_f64().unwrap());
}

#[test]
fn get_column_returns_all_rows_for_header() {
    let df = DataFrame::new(1, 1, &["food", "cost"]);
    df.add_row(&[Value::Text("BEEF".to_string()), Value::Numeric(3.19)]);
    df.add_row(&[Value::Text("CHK".to_string()), Value::Numeric(2.59)]);

    let costs: Vec<f64> = df.get_column("cost").iter().map(|v| v.as_f64().unwrap()).collect();
    assert_eq!(vec![3.19, 2.59], costs);

    let foods: Vec<String> = df.get_column("food").iter().map(|v| v.as_str().unwrap().to_string()).collect();
    assert_eq!(vec!["BEEF".to_string(), "CHK".to_string()], foods);
}

#[test]
fn set_value_by_position_and_by_key() {
    let df = DataFrame::new(1, 1, &["food", "cost"]);
    df.add_row(&[Value::Text("BEEF".to_string()), Value::Numeric(3.19)]);

    df.set_value(0, 1, &Value::Numeric(9.99));
    assert_eq!(9.99, df.get_value(0, 1).as_f64().unwrap());

    df.set_value_at(&[Value::Text("BEEF".to_string())], "cost", &Value::Numeric(1.11));
    assert_eq!(1.11, df.get_value(0, 1).as_f64().unwrap());
}

#[test]
fn add_empty_column_then_set_column_doubles_and_strings() {
    let df = DataFrame::new(1, 0, &["food"]);
    df.add_row(&[Value::Text("BEEF".to_string())]);
    df.add_row(&[Value::Text("CHK".to_string())]);

    df.add_empty_column("cost");
    assert_eq!(2, df.num_cols());
    df.set_column_doubles("cost", &[3.19, 2.59]);
    let costs: Vec<f64> = df.get_column("cost").iter().map(|v| v.as_f64().unwrap()).collect();
    assert_eq!(vec![3.19, 2.59], costs);

    df.add_empty_column("tag");
    df.set_column_strings("tag", &["meat", "meat"]);
    let tags: Vec<String> = df.get_column("tag").iter().map(|v| v.as_str().unwrap().to_string()).collect();
    assert_eq!(vec!["meat".to_string(), "meat".to_string()], tags);
}

#[test]
fn add_column_doubles_and_strings_in_one_shot() {
    let df = DataFrame::new(1, 0, &["food"]);
    df.add_row(&[Value::Text("BEEF".to_string())]);
    df.add_row(&[Value::Text("CHK".to_string())]);

    df.add_column_doubles("cost", &[3.19, 2.59]);
    df.add_column_strings("tag", &["meat", "meat"]);

    assert_eq!(3, df.num_cols());
    assert_eq!(2.59, df.get_value(1, 1).as_f64().unwrap());
    assert_eq!("meat", df.get_value(0, 2).as_str().unwrap());
}

#[test]
fn set_array_fills_single_index_dataframe() {
    let df = DataFrame::new(1, 1, &["food", "cost"]);
    df.set_array(&["BEEF", "CHK"], &[3.19, 2.59]);
    assert_eq!(2, df.num_rows());
    assert_eq!(Some(1), df.find_row(&[Value::Text("CHK".to_string())]));
    assert_eq!(2.59, df.get_row(&[Value::Text("CHK".to_string())])[1].as_f64().unwrap());
}

#[test]
fn set_matrix_doubles_fills_two_index_dataframe() {
    let df = DataFrame::new(2, 1, &["nutr", "food", "amt"]);
    df.set_matrix_doubles(&["A", "B"], &["BEEF", "CHK"], &[1.0, 2.0, 3.0, 4.0]);
    assert_eq!(4, df.num_rows());

    let row = df.get_row(&[Value::Text("A".to_string()), Value::Text("CHK".to_string())]);
    assert_eq!(2.0, row[2].as_f64().unwrap());

    let row = df.get_row(&[Value::Text("B".to_string()), Value::Text("BEEF".to_string())]);
    assert_eq!(3.0, row[2].as_f64().unwrap());
}

#[test]
fn set_matrix_strings_fills_two_index_dataframe() {
    let df = DataFrame::new(2, 1, &["nutr", "food", "tag"]);
    df.set_matrix_strings(&["A", "B"], &["BEEF", "CHK"], &["x1", "x2", "x3", "x4"]);
    assert_eq!(4, df.num_rows());

    let row = df.get_row(&[Value::Text("A".to_string()), Value::Text("CHK".to_string())]);
    assert_eq!("x2", row[2].as_str().unwrap());
}

#[test]
fn rows_iterator_visits_every_row_in_order() {
    let df = DataFrame::new(1, 1, &["food", "cost"]);
    df.add_row(&[Value::Text("BEEF".to_string()), Value::Numeric(3.19)]);
    df.add_row(&[Value::Text("CHK".to_string()), Value::Numeric(2.59)]);

    let collected: Vec<Vec<Value>> = df.rows().collect();
    assert_eq!(2, collected.len());
    assert_eq!("BEEF", collected[0][0].as_str().unwrap());
    assert_eq!("CHK", collected[1][0].as_str().unwrap());
}

#[test]
fn partial_eq_compares_structure_and_contents() {
    let df1 = DataFrame::new(1, 1, &["food", "cost"]);
    df1.add_row(&[Value::Text("BEEF".to_string()), Value::Numeric(3.19)]);

    let df2 = DataFrame::new(1, 1, &["food", "cost"]);
    df2.add_row(&[Value::Text("BEEF".to_string()), Value::Numeric(3.19)]);
    assert!(df1 == df2);

    df2.add_row(&[Value::Text("CHK".to_string()), Value::Numeric(2.59)]);
    assert!(df1 != df2);
}

#[test]
fn to_string_contains_headers_and_values() {
    let df = DataFrame::new(1, 1, &["food", "cost"]);
    df.add_row(&[Value::Text("BEEF".to_string()), Value::Numeric(3.19)]);
    let s = df.to_string();
    assert!(s.contains("food"));
    assert!(s.contains("cost"));
    assert!(s.contains("BEEF"));
}

#[test]
fn set_data_and_get_data_round_trip_through_ampl() {
    let mut ampl = init();
    ampl.eval("set A; param p{A} default 0;");

    let df = DataFrame::new(1, 1, &["A", "p"]);
    df.set_array(&["a", "b", "c"], &[10.0, 20.0, 30.0]);
    ampl.set_data(&df, Some("A"));

    let out = ampl.get_data(&["p"]);
    assert_eq!(3, out.num_rows());
    let total: f64 = out.get_column("p").iter().map(|v| v.as_f64().unwrap()).sum();
    assert_eq!(60.0, total);
}
