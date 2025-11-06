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
