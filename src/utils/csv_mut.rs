pub fn remove_data_by_row_no_mutably(records: &mut Vec<Vec<String>>, row_no_list: &[usize]) {
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

pub fn remove_data_by_column_no_mutably(records: &mut [Vec<String>], column_no_list: &[usize]) {
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

pub fn select_data_by_row_no_mutably(records: &mut Vec<Vec<String>>, row_no_list: &[usize]) {
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

pub fn select_data_by_column_no_mutably(records: &mut Vec<Vec<String>>, column_no_list: &[usize]) {
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

pub fn replace_data_at_row_no_mutably(
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

pub fn replace_data_at_column_no_mutably(
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

pub fn replace_data_at_position_mutably(
    records: &mut [Vec<String>],
    row_no: usize,
    column_no: usize,
    replace_data: &str,
) {
    let index = row_no - 1;
    if index < records.len() {
        records[index][column_no - 1] = replace_data.to_string();
    } else {
        panic!("Index {} out of bounds", row_no);
    }
}

pub fn replace_data_at_index_mutably(
    records: &mut [Vec<String>],
    row_index: usize,
    column_index: usize,
    replace_data: &str,
) {
    let index = row_index;
    if index < records.len() {
        records[index][column_index] = replace_data.to_string();
    } else {
        panic!("Index {} out of bounds", row_index);
    }
}

pub fn transpose_data_mutably(records: &mut Vec<Vec<String>>) {
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

pub fn insert_data_before_row_mutably(
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

pub fn insert_data_before_column_mutably(
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

pub fn append_data_row_mutably(records: &mut Vec<Vec<String>>, append_data: Vec<String>) {
    records.push(append_data);
}

pub fn append_data_column_mutably(records: &mut [Vec<String>], append_data: Vec<String>) {
    if records.len() != append_data.len() {
        panic!(
            "append_data length ({}) does not match number of rows ({})",
            append_data.len(),
            records.len()
        );
    }

    for (i, record) in records.iter_mut().enumerate() {
        record.push(append_data[i].clone());
    }
}

pub fn convert_string_data_to_csv_data_mutably(data: &mut Vec<Vec<String>>) {
    let mut result: Vec<Vec<String>> = Vec::new();

    for row in data.iter() {
        for val in row.iter() {
            let p: Vec<&str> = val.split_whitespace().collect();

            result.push(p.into_iter().map(|s| s.to_string()).collect());
        }
    }

    *data = result;
}
