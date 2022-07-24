extern crate excel2csv as e2c;

const PATH_TO_EXCEL: &str = "./input/test.xlsx";
const PATH_TO_CSV: &str = "./output/test.csv";
const SHEETNAME: &str = "Sheet1";

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
    assert_eq!(e2c::read_it(PATH_TO_EXCEL, SHEETNAME), result);
}

#[test]
fn convert_to_csv_test() {
    let container = e2c::read_it_all(PATH_TO_EXCEL, SHEETNAME);
    match e2c::convert_to_csv(container, PATH_TO_CSV) {
        Ok(()) => println!("CSV written"),
        _ => println!("Error while writing CSV"),
    }
}

#[test]
fn excel2csv_test() {
    e2c::excel2csv(PATH_TO_EXCEL, SHEETNAME, PATH_TO_CSV);
}
