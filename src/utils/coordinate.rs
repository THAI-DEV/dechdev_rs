pub fn coordinate_index_all(
    row_index: usize,
    column_index: usize,
    max_row_index: usize,
    max_column_index: usize,
    step: usize,
) -> CoordinateIndexType {
    // Example Grid: 3x10
    // 00 01 02 03 04 05 06 07 08 09
    // 10 11 12 13 14 15 16 17 18 19
    // 20 21 22 23 24 25 26 27 28 29

    //* Right
    let mut temp_row = row_index as i32;
    let mut temp_col = column_index as i32 + step as i32;

    if temp_col > max_column_index as i32 {
        temp_col = -1;
    }

    let right = (temp_row, temp_col);
    //* Right

    //* Left
    temp_row = row_index as i32;
    temp_col = column_index as i32 - 1;
    if temp_col < 0 {
        temp_col = -1;
    }

    let left = (temp_row, temp_col);
    //* Left

    //* Down
    temp_row = row_index as i32 + 1;
    temp_col = column_index as i32;

    if temp_row > max_row_index as i32 {
        temp_row = -1;
    }

    let down = (temp_row, temp_col);
    //* Down

    //* Up
    temp_row = row_index as i32 - 1;
    temp_col = column_index as i32;
    if temp_row < 0 {
        temp_row = -1;
    }

    let up = (temp_row, temp_col);
    //* Up

    //* Right Up
    temp_row = row_index as i32 - 1;
    temp_col = column_index as i32 + 1;
    if temp_row < 0 || temp_col > max_column_index as i32 {
        temp_row = -1;
        temp_col = -1;
    }

    let right_up = (temp_row, temp_col);
    //* Right Up

    //* Right Down
    temp_row = row_index as i32 + 1;
    temp_col = column_index as i32 + 1;
    if temp_row > max_row_index as i32 || temp_col > max_column_index as i32 {
        temp_row = -1;
        temp_col = -1;
    }

    let right_down = (temp_row, temp_col);
    //* Right Down

    //* Left Up
    temp_row = row_index as i32 - 1;
    temp_col = column_index as i32 - 1;
    if temp_row < 0 || temp_col < 0 {
        temp_row = -1;
        temp_col = -1;
    }

    let left_up = (temp_row, temp_col);
    //* Left Up

    //* Left Down
    temp_row = row_index as i32 + 1;
    temp_col = column_index as i32 - 1;
    if temp_row > max_row_index as i32 || temp_col < 0 {
        temp_row = -1;
        temp_col = -1;
    }
    let left_down = (temp_row, temp_col);
    //* Left Down

    // println!("right: {:?}, left: {:?}", right, left);
    // println!("down: {:?}, up: {:?}", down, up);
    // println!("right_up: {:?}, right_down: {:?}", right_up, right_down);
    // println!("left_up: {:?}, left_down: {:?}", left_up, left_down);

    CoordinateIndexType {
        right,
        left,
        down,
        up,
        right_down,
        right_up,
        left_up,
        left_down,
    }
}

pub fn coordinate_index_right(
    row_index: usize,
    column_index: usize,
    max_column_index: usize,
    step: usize,
) -> (i32, i32) {
    //* Right
    let temp_row = row_index as i32;
    let mut temp_col = column_index as i32 + step as i32;

    if temp_col > max_column_index as i32 {
        temp_col = -1;
    }

    (temp_row, temp_col)
}

pub fn coordinate_index_left(row_index: usize, column_index: usize, step: usize) -> (i32, i32) {
    //* Left
    let temp_row = row_index as i32;
    let mut temp_col = column_index as i32 - step as i32;
    if temp_col < 0 {
        temp_col = -1;
    }

    (temp_row, temp_col)
    //* Left
}

pub fn coordination_index_down(
    row_index: usize,
    column_index: usize,
    max_row_index: usize,
    step: usize,
) -> (i32, i32) {
    //* Down
    let mut temp_row = row_index as i32 + step as i32;
    let temp_col = column_index as i32;

    if temp_row > max_row_index as i32 {
        temp_row = -1;
    }

    (temp_row, temp_col)
    //* Down
}

pub fn coordination_index_up(row_index: usize, column_index: usize, step: usize) -> (i32, i32) {
    //* Up
    let mut temp_row = row_index as i32 - step as i32;
    let temp_col = column_index as i32;
    if temp_row < 0 {
        temp_row = -1;
    }

    (temp_row, temp_col)
    //* Up
}

pub fn coordination_index_right_up(
    row_index: usize,
    column_index: usize,
    max_column_index: usize,
    step: usize,
) -> (i32, i32) {
    //* Right Up
    let mut temp_row = row_index as i32 - step as i32;
    let mut temp_col = column_index as i32 + step as i32;
    if temp_row < 0 || temp_col > max_column_index as i32 {
        temp_row = -1;
        temp_col = -1;
    }

    (temp_row, temp_col)
    //* Right Up
}

pub fn coordination_index_right_down(
    row_index: usize,
    column_index: usize,
    max_column_index: usize,
    max_row_index: usize,
    step: usize,
) -> (i32, i32) {
    //* Right Down
    let mut temp_row = row_index as i32 + step as i32;
    let mut temp_col = column_index as i32 + step as i32;
    if temp_row > max_row_index as i32 || temp_col > max_column_index as i32 {
        temp_row = -1;
        temp_col = -1;
    }

    (temp_row, temp_col)
    //* Right Down
}

pub fn coordination_index_left_up(
    row_index: usize,
    column_index: usize,
    step: usize,
) -> (i32, i32) {
    //* Left Up
    let mut temp_row = row_index as i32 - step as i32;
    let mut temp_col = column_index as i32 - step as i32;
    if temp_row < 0 || temp_col < 0 {
        temp_row = -1;
        temp_col = -1;
    }

    (temp_row, temp_col)
    //* Left Up
}

pub fn coordination_index_left_down(
    row_index: usize,
    column_index: usize,
    max_row_index: usize,
    step: usize,
) -> (i32, i32) {
    //* Left Down
    let mut temp_row = row_index as i32 + step as i32;
    let mut temp_col = column_index as i32 - step as i32;
    if temp_row > max_row_index as i32 || temp_col < 0 {
        temp_row = -1;
        temp_col = -1;
    }
    (temp_row, temp_col)
    //* Left Down
}

#[derive(Debug)]
pub struct CoordinateIndexType {
    pub right: (i32, i32),
    pub left: (i32, i32),
    pub down: (i32, i32),
    pub up: (i32, i32),
    pub right_down: (i32, i32),
    pub right_up: (i32, i32),
    pub left_up: (i32, i32),
    pub left_down: (i32, i32),
}
