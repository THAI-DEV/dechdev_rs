use csv::{ReaderBuilder, Writer};
use std::fs::OpenOptions;

pub fn read_csv_file(file_path: &str, has_headers: bool) -> Vec<Vec<String>> {
    let mut reader = ReaderBuilder::new()
        .has_headers(has_headers)
        .from_path(file_path)
        .expect("Failed to open CSV file");
    let mut records = Vec::new();
    for result in reader.records() {
        let record = result.expect("Failed to read a record");
        records.push(record.iter().map(|s| s.to_string()).collect());
    }
    records
}

pub fn write_csv_file(file_path: &str, data: Vec<Vec<String>>) {
    let mut writer = Writer::from_path(file_path).expect("Failed to create CSV writer");
    for row in data {
        writer.write_record(&row).expect("Failed to write a record");
    }
    writer.flush().expect("Failed to flush writer");
}

pub fn append_csv_file(file_path: &str, data: Vec<Vec<String>>) {
    let file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .expect("Failed to open or create file");

    let mut writer = Writer::from_writer(file);
    for row in data {
        writer.write_record(&row).expect("Failed to write a record");
    }
    writer.flush().expect("Failed to flush writer");
}

pub fn show_csv_data(records: &Vec<Vec<String>>) {
    for record in records {
        println!("{:?}", record);
    }
}

pub fn remove_data_by_row_no(records: &mut Vec<Vec<String>>, row_no_list: &[usize]) {
    let mut row_no_list: Vec<usize> = row_no_list.iter().map(|&n| n - 1).collect();
    row_no_list.sort_unstable_by(|a, b| b.cmp(a)); // Sort in descending order to avoid shifting issues

    for &row_no in &row_no_list {
        if row_no < records.len() {
            records.remove(row_no);
        } else {
            panic!("Index {} out of bounds", row_no + 1);
        }
    }
}

pub fn remove_data_by_column_no(records: &mut [Vec<String>], column_no_list: &[usize]) {
    let mut column_no_list: Vec<usize> = column_no_list.iter().map(|&n| n - 1).collect();
    column_no_list.sort_unstable_by(|a, b| b.cmp(a)); // Sort in descending order to avoid shifting issues

    for record in records.iter_mut() {
        for &col_no in &column_no_list {
            if col_no < record.len() {
                record.remove(col_no);
            } else {
                panic!("Index {} out of bounds", col_no + 1);
            }
        }
    }
}

pub fn select_data_by_row_no(records: &mut Vec<Vec<String>>, row_no_list: &[usize]) {
    let mut selected_records = Vec::new();
    for &row_no in row_no_list {
        let index = row_no - 1;
        if index < records.len() {
            selected_records.push(records[index].clone());
        } else {
            panic!("Index {} out of bounds", row_no);
        }
    }
    *records = selected_records;
}

pub fn select_data_by_column_no(records: &mut Vec<Vec<String>>, column_no_list: &[usize]) {
    let mut selected_records = Vec::new();
    for record in records.iter() {
        let mut selected_row = Vec::new();
        for &col_no in column_no_list {
            let index = col_no - 1;
            if index < record.len() {
                selected_row.push(record[index].clone());
            } else {
                panic!("Index {} out of bounds", col_no);
            }
        }
        selected_records.push(selected_row);
    }
    *records = selected_records;
}

pub fn replace_data_at_row_no(
    records: &mut [Vec<String>],
    row_no: usize,
    replace_data: Vec<String>,
) {
    let index = row_no - 1;
    if index < records.len() {
        records[index] = replace_data;
    } else {
        panic!("Index {} out of bounds", row_no);
    }
}

pub fn replace_data_at_column_no(
    records: &mut [Vec<String>],
    column_no: usize,
    replace_data: Vec<String>,
) {
    let index = column_no - 1;
    for (i, record) in records.iter_mut().enumerate() {
        if index < record.len() {
            record[index] = replace_data[i].clone();
        } else {
            panic!("Index {} out of bounds", column_no);
        }
    }
}

pub fn is_found_data(records: &[Vec<String>], find_data: &str) -> bool {
    for record in records.iter() {
        for field in record.iter() {
            if field == find_data {
                return true;
            }
        }
    }

    false
}

pub fn find_data(records: &[Vec<String>], find_data: &str) -> Vec<(usize, usize)> {
    let mut results = Vec::new();
    for (row_no, record) in records.iter().enumerate() {
        for (col_no, field) in record.iter().enumerate() {
            if field == find_data {
                results.push((row_no + 1, col_no + 1)); // Store 1-based indices
            }
        }
    }

    results // Return all matches
}

pub fn show_csv_size(records: &[Vec<String>]) -> (usize, usize) {
    let row_count = records.len();
    let col_count = if row_count > 0 { records[0].len() } else { 0 };

    (row_count, col_count)
}

pub fn flatten_csv_data(data: &[Vec<String>]) -> Vec<String> {
    data.iter().map(|row| row.join(",")).collect()
}

pub fn transpose_data(records: &mut Vec<Vec<String>>) {
    let mut transposed = Vec::new();
    let row_count = records.len();
    if row_count > 0 {
        records[0].len()
    } else {
        0
    };

    for col in records[0].iter().enumerate() {
        let mut new_row = Vec::new();
        for row in records.iter() {
            new_row.push(row[col.0].clone());
        }
        transposed.push(new_row);
    }

    *records = transposed;
}

pub fn insert_data_before_row(
    records: &mut Vec<Vec<String>>,
    row_no: usize,
    insert_data: Vec<String>,
) {
    if let Some(index) = row_no.checked_sub(1) {
        if index <= records.len() {
            records.insert(index, insert_data);
        } else {
            panic!("Row number {} is out of bounds", row_no);
        }
    } else {
        panic!("Row number must be greater than 0");
    }
}

pub fn insert_data_before_column(
    records: &mut [Vec<String>],
    column_no: usize,
    insert_data: Vec<String>,
) {
    if records.len() != insert_data.len() {
        panic!(
            "insert_data length ({}) does not match number of rows ({})",
            insert_data.len(),
            records.len()
        );
    }

    let index = column_no.checked_sub(1).unwrap_or_else(|| {
        panic!("Column number must be greater than or equal to 1");
    });

    for (i, record) in records.iter_mut().enumerate() {
        if index > record.len() {
            panic!(
                "Column number {} is out of bounds for row {}",
                column_no,
                i + 1
            );
        }
        record.insert(index, insert_data[i].clone());
    }
}
