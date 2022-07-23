use std::path::Path;
use std::error::Error;

// use umya_spreadsheet::*;

pub fn read_it() -> String {
    // READER
    let path = Path::new("./input/test.xlsx");
    let book = umya_spreadsheet::reader::xlsx::read(path).unwrap();

    // read value in cell A1
    let a_one_value = book.get_sheet_by_name("Sheet1").unwrap().get_value("A1");
    println!("A1 = {}", a_one_value);

    a_one_value
}

pub fn read_it_all() -> Vec<Vec<String>> {
    let path = Path::new("./input/test.xlsx");
    let book = umya_spreadsheet::reader::xlsx::read(path).unwrap();

    // read everything until cell (10, 10)
    let mut container: Vec<Vec<String>> = Vec::new();
    for i in 1..=10 {
        let mut line_storage: Vec<String> = Vec::new();
        for j in 1..=10 {
            let val = book.get_sheet_by_name("Sheet1").unwrap().get_value_by_column_and_row(&i, &j);
            line_storage.push(val);
        }
        container.push(line_storage);
    }
    
    // println!("{:?}", container);
    container
}

pub fn convert_to_csv(container: Vec<Vec<String>>) -> Result<(), Box<dyn Error>> {
    let file_path = "./output/test.csv";
    let mut wtr = csv::Writer::from_path(file_path)?;

    for line in container {
        wtr.write_record(&line)?;
    }

    wtr.flush()?;
    Ok(())
}
