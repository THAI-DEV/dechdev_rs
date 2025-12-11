use std::fs::OpenOptions;

use csv::{ReaderBuilder, Writer};

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

pub fn retrieve_data_by_position(
    records: &[Vec<String>],
    row_no: usize,
    column_no: usize,
) -> String {
    if row_no == 0 || column_no == 0 {
        println!("Row and column indices must be 1 or greater");
        return String::new();
    }

    records[row_no - 1][column_no - 1].to_string()
}

pub fn retrieve_data_by_index(
    records: &[Vec<String>],
    row_index: usize,
    column_index: usize,
) -> String {
    if row_index >= records.len() || column_index >= records[0].len() {
        println!("Row and column indices must be within the valid range");
        return String::new();
    }

    records[row_index][column_index].to_string()
}

pub fn show_csv_size(records: &[Vec<String>]) -> (usize, usize) {
    let row_count = records.len();
    let col_count = if row_count > 0 { records[0].len() } else { 0 };

    (row_count, col_count)
}

pub fn remove_data_by_row_no(records: &[Vec<String>], row_no_list: &[usize]) -> Vec<Vec<String>> {
    let mut row_no_list: Vec<usize> = row_no_list.iter().map(|&n| n - 1).collect();
    row_no_list.sort_unstable_by(|a, b| b.cmp(a)); // Sort in descending order to avoid shifting issues

    let mut new_records = records.to_vec();
    for &row_no in &row_no_list {
        if row_no < records.len() {
            new_records.remove(row_no);
        } else {
            println!("Index {} out of bounds", row_no + 1);
            return records.to_vec();
        }
    }
    new_records
}

pub fn remove_data_by_column_no(
    records: &[Vec<String>],
    column_no_list: &[usize],
) -> Vec<Vec<String>> {
    let mut column_no_list: Vec<usize> = column_no_list.iter().map(|&n| n - 1).collect();
    column_no_list.sort_unstable_by(|a, b| b.cmp(a)); // Sort in descending order to avoid shifting issues

    records
        .iter()
        .map(|record| {
            let mut record = record.to_vec();
            for &col_no in &column_no_list {
                if col_no < record.len() {
                    record.remove(col_no);
                } else {
                    println!("Index {} out of bounds", col_no + 1);
                }
            }
            record
        })
        .collect()
}

pub fn select_data_by_row_no(records: &[Vec<String>], row_no_list: &[usize]) -> Vec<Vec<String>> {
    let mut selected_records = Vec::new();
    for &row_no in row_no_list {
        let index = row_no - 1;
        if index < records.len() {
            selected_records.push(records[index].to_vec());
        } else {
            println!("Index {} out of bounds", row_no);
        }
    }
    selected_records
}

pub fn select_data_by_column_no(
    records: &[Vec<String>],
    column_no_list: &[usize],
) -> Vec<Vec<String>> {
    let mut selected_records = Vec::new();
    for record in records.iter() {
        let mut selected_row = Vec::new();
        for &col_no in column_no_list {
            let index = col_no - 1;
            if index < record.len() {
                selected_row.push(record[index].to_string());
            } else {
                println!("Index {} out of bounds", col_no);
            }
        }
        selected_records.push(selected_row);
    }
    selected_records
}

pub fn replace_data_at_row_no(
    records: &[Vec<String>],
    row_no: usize,
    replace_data: &[String],
) -> Vec<Vec<String>> {
    let index = row_no - 1;
    if index < records.len() {
        let mut new_records = records.to_vec();
        new_records[index] = replace_data.to_vec();
        new_records
    } else {
        println!("Index {} out of bounds", row_no);
        records.to_vec()
    }
}

pub fn replace_data_at_column_no(
    records: &[Vec<String>],
    column_no: usize,
    replace_data: &[String],
) -> Vec<Vec<String>> {
    let index = column_no - 1;
    records
        .iter()
        .enumerate()
        .map(|(i, record)| {
            if index < record.len() {
                let mut new_record = record.to_vec();
                new_record[index] = replace_data[i].to_string();
                new_record
            } else {
                println!("Index {} out of bounds", column_no);
                record.to_vec()
            }
        })
        .collect()
}

pub fn transpose_data(records: &[Vec<String>]) -> Vec<Vec<String>> {
    let mut transposed = Vec::new();
    let row_count = records.len();
    let col_count = if row_count > 0 { records[0].len() } else { 0 };

    for col in 0..col_count {
        let mut new_row = Vec::with_capacity(row_count);
        for row in records {
            new_row.push(row[col].to_owned());
        }
        transposed.push(new_row);
    }

    transposed
}

pub fn insert_data_before_row(
    records: &[Vec<String>],
    row_no: usize,
    insert_data: &[String],
) -> Vec<Vec<String>> {
    if let Some(index) = row_no.checked_sub(1) {
        if index <= records.len() {
            let mut new_records = records.to_vec();
            new_records.insert(index, insert_data.to_vec());
            new_records
        } else {
            print!("Row number {} is out of bounds", row_no);
            records.to_vec()
        }
    } else {
        println!("Row number must be greater than 0");
        records.to_vec()
    }
}

pub fn insert_data_before_column(
    records: &[Vec<String>],
    column_no: usize,
    insert_data: &[String],
) -> Vec<Vec<String>> {
    if records.len() != insert_data.len() {
        println!(
            "insert_data length ({}) does not match number of rows ({})",
            insert_data.len(),
            records.len()
        );
    }

    let index = column_no.checked_sub(1).unwrap_or_else(|| {
        println!("Column number must be greater than or equal to 1");
        0 // Default to 0 if column_no is invalid
    });

    records
        .iter()
        .enumerate()
        .map(|(i, record)| {
            if index > record.len() {
                panic!(
                    "Column number {} is out of bounds for row {}",
                    column_no,
                    i + 1
                );
            }
            let mut new_record = Vec::with_capacity(record.len() + 1);
            new_record.extend_from_slice(&record[..index]);
            new_record.push(insert_data[i].to_owned());
            new_record.extend_from_slice(&record[index..]);
            new_record
        })
        .collect()
}

pub fn append_data_row(records: &[Vec<String>], append_data: &[String]) -> Vec<Vec<String>> {
    let mut new_records = records.to_vec();
    new_records.push(append_data.to_vec());
    new_records
}

pub fn append_data_column(records: &[Vec<String>], append_data: &[String]) -> Vec<Vec<String>> {
    if records.len() != append_data.len() {
        println!(
            "append_data length ({}) does not match number of rows ({})",
            append_data.len(),
            records.len()
        );

        return records.to_vec();
    }

    records
        .iter()
        .zip(append_data)
        .map(|(record, value)| {
            let mut new_record = record.to_vec();
            new_record.push(value.to_owned());
            new_record
        })
        .collect()
}

pub fn flatten_csv_data(data: &[Vec<String>]) -> Vec<String> {
    data.iter().map(|row| row.join(",")).collect()
}

pub fn convert_string_data_to_csv_data(data: &[Vec<String>]) -> Vec<Vec<String>> {
    /*
       123 328   51  64
        45  64  387  23
         6  98  215 314
    */
    let mut result: Vec<Vec<String>> = Vec::new();

    for row in data.iter() {
        for val in row.iter() {
            let p: Vec<&str> = val.split_whitespace().collect();

            result.push(p.into_iter().map(|s| s.to_string()).collect());
        }
    }

    result
}
