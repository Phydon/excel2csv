extern crate excel2csv as e2c;

#[test]
fn test_test() {
    let indeed: bool = true;
    assert!(indeed);
}

#[test]
#[should_panic(expected = "panic msg")]
fn panic_test() {
    panic!("panic msg");
}

#[test]
fn get_a1_test() {
    let result: String = "column A1".to_string();
    assert_eq!(e2c::read_it(), result);
}

#[test]
fn convert_to_csv_test() {
    let container = e2c::read_it_all();
    match e2c::convert_to_csv(container) {
        Ok(()) => println!("CSV written"),
        _ => println!("Error while writing CSV"),
    }
}
