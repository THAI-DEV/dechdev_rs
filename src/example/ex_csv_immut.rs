use crate::utils::csv::{insert_data_before_column, show_csv_data};

pub fn example_csv_immut() {
    println!("example_csv_immut");

    let v1 = vec![
        vec![
            "Name".to_string(),
            "Age".to_string(),
            "City".to_string(),
            "Country".to_string(),
        ],
        vec![
            "Alice".to_string(),
            "30".to_string(),
            "New York".to_string(),
            "USA".to_string(),
        ],
        vec![
            "Bob".to_string(),
            "25".to_string(),
            "Los Angeles".to_string(),
            "USA".to_string(),
        ],
        vec![
            "Charlie".to_string(),
            "35".to_string(),
            "Chicago".to_string(),
            "USA".to_string(),
        ],
    ];

    let v2 = vec![
        "เดช".to_string(),
        "126".to_string(),
        "Bangkok".to_string(),
        "Thailand".to_string(),
    ];

    show_csv_data(&v1);
    println!("------------------");

    let result = insert_data_before_column(&v1, 4, &v2);

    //let result = append_data_row(&v1, &v2);
    // let result = append_data_column(&v1, &v2);
    show_csv_data(&result);
}
