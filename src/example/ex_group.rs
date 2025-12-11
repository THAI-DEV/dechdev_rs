use crate::utils::group::{
    arrange_group_as_distribute, arrange_group_as_separate, calculate_group_index_range,
    calculate_paging, convert_string_param_to_vector,
};

pub fn example_group() {
    println!("example_group");
    let (begin_index, end_index, max_page) = calculate_paging(2, 10, 100);
    println!(
        "begin_index: {}, end_index: {}, max_page: {}",
        begin_index, end_index, max_page
    );

    let r = calculate_group_index_range(29, 3);
    println!("calculate_group_index_range: {:?}", r);

    let r = convert_string_param_to_vector("DM-01-01-01-01, MPS001", ",");
    println!("convert_string_param_to_vector: {:?}", r);

    let input_vec: Vec<String> = vec![
        "01".to_string(),
        "02".to_string(),
        "03".to_string(),
        "04".to_string(),
        "05".to_string(),
        "06".to_string(),
        "07".to_string(),
        "08".to_string(),
        "09".to_string(),
    ];
    let input_slice = &input_vec;
    let group_num = 2;

    let result_1 = arrange_group_as_separate(input_slice, group_num, true);
    let result_2 = arrange_group_as_separate(input_slice, group_num, false);
    let result_3 = arrange_group_as_distribute(input_slice, group_num);

    println!("arrange_group_as_separate (true): {:?}", result_1);
    println!("arrange_group_as_separate (false): {:?}", result_2);
    println!("arrange_group_as_distribute: {:?}", result_3);
}
