use crate::utils::csv::{
    find_data, flatten_csv_row, is_found_data, read_csv_file, select_data_by_row_no, show_csv_data,
    show_csv_size,
};

pub fn example_csv() {
    let source = read_csv_file("./test.csv", true);
    let mut r = source.clone();

    show_csv_data(&r);

    println!("-----------------");
    select_data_by_row_no(&mut r, &[2]);

    let rr = flatten_csv_row(&r);
    println!("flatten {:?}", rr);

    // replace_data_at_column_no(
    //     &mut r,
    //     2,
    //     vec![
    //         "new_value1",
    //         "new_value2",
    //         "new_value3",
    //         "new_value4",
    //         "new_value5",
    //         "new_value6",
    //     ]
    //     .into_iter()
    //     .map(|s| s.to_string())
    //     .collect(),
    // );

    show_csv_data(&r);

    println!("-----------------");
    println!("Original data:");
    show_csv_data(&source);

    let (_row_count, _col_count) = show_csv_size(&source);
    println!("Total Rows: {}, Total Columns: {}", _row_count, _col_count);

    let found = is_found_data(&source, "b");
    println!("Data found: {}", found);

    let result = find_data(&source, "b");
    println!("Find results: {:?}", result);
}
