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
