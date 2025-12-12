pub fn remove_data_by_row_no_mutably(records: &mut Vec<Vec<String>>, row_no_list: &[usize]) {
    if row_no_list.contains(&0) {
        println!("Index 0 is invalid");
        return;
    }

    let mut row_no_list: Vec<usize> = row_no_list.iter().map(|&n| n - 1).collect();
    row_no_list.sort_unstable_by(|a, b| b.cmp(a)); // Sort in descending order to avoid shifting issues

    for &row_no in &row_no_list {
        if row_no < records.len() {
            records.remove(row_no);
        } else {
            println!("Index {} out of bounds", row_no + 1);
            break;
        }
    }
}

pub fn remove_data_by_column_no_mutably(records: &mut [Vec<String>], column_no_list: &[usize]) {
    if column_no_list.contains(&0) {
        println!("Index 0 is invalid");
        return;
    }

    let mut column_no_list: Vec<usize> = column_no_list.iter().map(|&n| n - 1).collect();
    column_no_list.sort_unstable_by(|a, b| b.cmp(a)); // Sort in descending order to avoid shifting issues

    for record in records.iter_mut() {
        for &col_no in &column_no_list {
            if col_no < record.len() {
                record.remove(col_no);
            } else {
                println!("Index {} out of bounds", col_no + 1);
                break;
            }
        }
    }
}

pub fn select_data_by_row_no_mutably(records: &mut Vec<Vec<String>>, row_no_list: &[usize]) {
    if row_no_list.contains(&0) {
        println!("Index 0 is invalid");
        return;
    }

    if row_no_list.contains(&(records[0].len() + 1)) {
        println!("Index {} is out of bounds", records[0].len() + 1);
        return;
    }

    let mut selected_records = Vec::new();
    for &row_no in row_no_list {
        let index = row_no - 1;
        if index < records.len() {
            let record = std::mem::take(&mut records[index]);
            selected_records.push(record);
        }
    }
    *records = selected_records;
}

pub fn select_data_by_column_no_mutably(records: &mut Vec<Vec<String>>, column_no_list: &[usize]) {
    if column_no_list.contains(&0) {
        println!("Index 0 is invalid");
        return;
    }

    if column_no_list.contains(&(records[0].len() + 1)) {
        println!("Index {} is out of bounds", records[0].len() + 1);
        return;
    }

    let mut selected_records = Vec::new();
    for record in records.iter_mut() {
        let mut selected_row = Vec::new();
        for &col_no in column_no_list {
            let index = col_no - 1;
            if index < record.len() {
                selected_row.push(std::mem::take(&mut record[index]));
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
    if row_no == 0 {
        println!("Index 0 is invalid");
        return;
    }

    if row_no > records.len() {
        println!("Index {} out of bounds", row_no);
        return;
    }

    let index = row_no - 1;
    if index < records.len() {
        records[index] = replace_data;
    }
}

pub fn replace_data_at_column_no_mutably(
    records: &mut [Vec<String>],
    column_no: usize,
    mut replace_data: Vec<String>,
) {
    if column_no == 0 {
        println!("Index 0 is invalid");
        return;
    }

    if column_no > records[0].len() {
        println!("Index {} out of bounds", column_no);
        return;
    }

    let index = column_no - 1;
    for (i, record) in records.iter_mut().enumerate() {
        if let Some(value) = replace_data.get_mut(i).filter(|_| index < record.len()) {
            record[index] = std::mem::take(value);
        }
    }
}

pub fn replace_data_at_position_mutably(
    records: &mut [Vec<String>],
    row_no: usize,
    column_no: usize,
    replace_data: &str,
) {
    if row_no == 0 || column_no == 0 {
        println!("Index 0 is invalid");
        return;
    }

    let index = row_no - 1;
    if index < records.len() {
        records[index][column_no - 1] = replace_data.to_string();
    } else {
        println!("Index {} out of bounds", row_no);
    }
}

pub fn replace_data_at_index_mutably(
    records: &mut [Vec<String>],
    row_index: usize,
    column_index: usize,
    replace_data: &str,
) {
    if row_index >= records[0].len() || column_index >= records[0].len() {
        println!("Row index Or Column index out of bounds");
        return;
    }

    let index = row_index;
    if index < records.len() {
        records[index][column_index] = replace_data.to_string();
    } else {
        println!("Index {} out of bounds", row_index);
    }
}

pub fn transpose_data_mutably(records: &mut Vec<Vec<String>>) {
    let row_count = records.len();
    let col_count = if row_count > 0 { records[0].len() } else { 0 };

    let mut transposed = vec![Vec::with_capacity(row_count); col_count];

    for row in records.iter_mut() {
        for (col_index, value) in row.drain(..).enumerate() {
            transposed[col_index].push(value);
        }
    }

    *records = transposed;
}

pub fn insert_data_before_row_mutably(
    records: &mut Vec<Vec<String>>,
    row_no: usize,
    insert_data: Vec<String>,
) {
    if row_no == 0 {
        println!("Row number must be greater than 0");
        return;
    }

    if row_no > records.len() {
        println!("Row number {} is out of bounds", row_no);
        return;
    }

    if let Some(index) = row_no.checked_sub(1) {
        if index <= records.len() {
            records.insert(index, insert_data);
        }
    } else {
        println!("Row number must be greater than 0");
    }
}

pub fn insert_data_before_column_mutably(
    records: &mut [Vec<String>],
    column_no: usize,
    mut insert_data: Vec<String>,
) {
    if column_no == 0 {
        println!("Column number must be greater than 0");
        return;
    }

    if column_no > records.len() {
        println!("Column number {} is out of bounds", column_no);
        return;
    }

    if records.len() != insert_data.len() {
        println!(
            "insert_data length ({}) does not match number of rows ({})",
            insert_data.len(),
            records.len()
        );
    }

    let index = column_no.checked_sub(1).unwrap_or_else(|| {
        println!("Column number must be greater than or equal to 1");
        0
    });

    for (i, record) in records.iter_mut().enumerate() {
        if index > record.len() {
            println!(
                "Column number {} is out of bounds for row {}",
                column_no,
                i + 1
            );
        }
        if let Some(value) = insert_data.get_mut(i) {
            record.insert(index, std::mem::take(value));
        }
    }
}

pub fn append_data_row_mutably(records: &mut Vec<Vec<String>>, append_data: Vec<String>) {
    records.push(append_data);
}

pub fn append_data_column_mutably(records: &mut [Vec<String>], mut append_data: Vec<String>) {
    if records.len() != append_data.len() {
        println!(
            "append_data length ({}) does not match number of rows ({})",
            append_data.len(),
            records.len()
        );
    }

    for (i, record) in records.iter_mut().enumerate() {
        if let Some(value) = append_data.get_mut(i) {
            record.push(std::mem::take(value));
        }
    }
}

pub fn convert_string_data_to_csv_data_mutably(data: &mut Vec<Vec<String>>) {
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

    *data = result;
}
