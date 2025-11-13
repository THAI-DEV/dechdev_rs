pub fn calculate_paging(page_no: i32, row_per_page: i32, total_item: i32) -> (i32, i32, i32) {
    let mut begin_index = (page_no - 1) * row_per_page;
    let mut end_index = page_no * row_per_page;
    end_index -= 1;
    if end_index >= total_item {
        end_index = total_item - 1;
    }

    let mut max_page = total_item / row_per_page;

    let total_item_mod = total_item % row_per_page;
    if total_item_mod > 0 {
        max_page += 1;
    }

    if page_no > max_page {
        begin_index = -1;
        end_index = -1;
    }

    (begin_index, end_index, max_page)
}

pub fn calculate_group_index_range(total_item: i32, num_group: i32) -> Vec<(i32, i32, i32)> {
    let mut num_group = num_group;
    num_group = num_group.min(total_item);

    let num_item_in_group = total_item / num_group;
    let num_remainder_data = total_item % num_group;

    let mut index_data_list: Vec<(i32, i32, i32)> = vec![];
    let mut begin_index = 0;
    let mut end_index;
    for i in 1..=num_group {
        let mut additional_index = -1;
        if num_remainder_data > 0 {
            let temp_additional_index = (num_group * num_item_in_group) + (i - 1);
            if temp_additional_index < total_item {
                additional_index = temp_additional_index;
            }
        }

        end_index = begin_index + num_item_in_group - 1;
        index_data_list.push((begin_index, end_index, additional_index));

        begin_index += num_item_in_group;
    }

    index_data_list
}

pub fn convert_string_param_to_vector(str_param: &str, delimiter: &str) -> Vec<String> {
    /*
     str_msg = "DM-01-01-01-01, MPS001"
     delimiter = ","
     result = [DM-01-01-01-01] [MPS001]
    */

    let mut result_list: Vec<String> = Vec::new();

    let str_arr: Vec<String> = if !str_param.is_empty() && !delimiter.is_empty() {
        let trimmed_param = str_param.trim();
        let trimmed_delimiter = delimiter.trim();
        trimmed_param
            .split(trimmed_delimiter)
            .map(|s| s.to_string())
            .collect()
    } else {
        Vec::new()
    };

    if !str_arr.is_empty() {
        for s in str_arr {
            if !s.is_empty() && !delimiter.trim().is_empty() {
                result_list.push(s.trim().to_string());
            }
        }
    }

    result_list
}

pub fn arrange_group_as_separate(
    input_slice: &[String],
    mut group_num: usize,
    is_asymmetry_member: bool,
) -> Vec<String> {
    /*
        input := []string{"01", "02", "03", "04", "05", "06", "07", "08", "09"}

        output := arrange_group_as_separate(input, 2, true)
        Result : group1 = 01 , 02 , 03 , 04 , 05
                 group2 = 06 , 07 , 08 , 09

        output := arrange_group_as_separate(input, 2, false)
        Result : group1 = 01 , 02 , 03 , 04 , 09
                 group2 = 05 , 06 , 07 , 08

    */

    if group_num > input_slice.len() {
        group_num = input_slice.len();
    }

    let total_member = input_slice.len();
    let member_in_group = if group_num > 0 {
        total_member / group_num
    } else {
        0
    };
    let beyond_member = if group_num > 0 {
        total_member % group_num
    } else {
        0
    };

    let mut result_list: Vec<String> = Vec::new();

    if member_in_group > 0 && input_slice.len() >= group_num {
        let mut ind = 0;

        for _ in 0..group_num {
            let mut parts: Vec<String> = Vec::new();
            for _ in 0..member_in_group {
                parts.push(input_slice[ind].clone());
                ind += 1;
            }
            result_list.push(parts.join(" , "));
        }

        if beyond_member > 0 {
            for _ in 0..beyond_member {
                for result in result_list.iter_mut() {
                    if ind == input_slice.len() {
                        break;
                    }
                    *result = format!("{} , {}", result, input_slice[ind]);
                    ind += 1;
                }
            }
        }

        if beyond_member > 0 && is_asymmetry_member {
            let mut new_result_list: Vec<String> = Vec::new();
            let mut ind = 0;

            for r in &result_list {
                // count items by splitting on comma
                let list: Vec<&str> = r.split(',').collect();
                let mut parts: Vec<String> = Vec::new();
                for _ in 0..list.len() {
                    if ind < input_slice.len() {
                        parts.push(input_slice[ind].clone());
                        ind += 1;
                    }
                }
                new_result_list.push(parts.join(" , "));
            }

            result_list = new_result_list;
        }
    }

    result_list
}

pub fn arrange_group_as_distribute(input_slice: &[String], mut group_num: usize) -> Vec<String> {
    /*
        input := []string{"01", "02", "03", "04", "05", "06", "07", "08", "09"}

        output := arrange_group_as_distribute(input, 2)
        Result : group1 = 01 , 03 , 05 , 07 , 09
                 group2 = 02 , 04 , 06 , 08
    */

    let mut result_list: Vec<String> = Vec::new();

    if group_num > input_slice.len() {
        group_num = input_slice.len();
    }

    if group_num == 0 || input_slice.len() < group_num {
        return result_list;
    }

    let mut ind = 0usize;

    // initialize groups with the first `group_num` elements
    for _ in 0..group_num {
        result_list.push(input_slice[ind].clone());
        ind += 1;
    }

    // distribute remaining elements round-robin, concatenating with " , "
    for _ in 0..input_slice.len() {
        for result in result_list.iter_mut() {
            if ind == input_slice.len() {
                break;
            }
            *result = format!("{} , {}", result, input_slice[ind]);
            ind += 1;
        }
    }

    result_list
}
