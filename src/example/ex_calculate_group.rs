use crate::utils::util::calculate_group_index_range;

pub fn example_calculate_group() {
    //* 1. Simulate source data list
    let total_data = 10;
    let num_group = 4;
    let source_data_list = sim_list_data(total_data);

    //* 2. Calculate group index ranges
    let index_list = calculate_group_index_range(total_data, num_group);
    println!("index_list: {:?}", index_list);

    //* 3. Fetch data based on calculated index ranges
    let mut result_data_list = vec![];
    for &(begin_index, end_index, additional_index) in &index_list {
        let group_data =
            fetch_data_each_group(begin_index, end_index, additional_index, &source_data_list);

        result_data_list.push(group_data);
    }

    println!("result_data_list: {:?}", result_data_list);
}

fn sim_list_data(total_data: i32) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    for i in 0..total_data {
        result.push(format!("Item {}", i + 1));
    }

    result
}

fn fetch_data_each_group(
    begin_index: i32,
    end_index: i32,
    additional_index: i32,
    source_data_list: &[String],
) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    result.push(
        source_data_list[begin_index as usize..=end_index as usize]
            .to_vec()
            .join(", "),
    );
    if additional_index > -1 {
        result.push(source_data_list[additional_index as usize].to_string());
    }

    result
}
