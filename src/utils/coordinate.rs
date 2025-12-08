pub fn all_index(
    row_index: usize,
    column_index: usize,
    max_row_index: usize,
    max_column_index: usize,
    step: usize,
) -> CoordinateIndexStruct {
    // Example Grid: 3x10
    // 00 01 02 03 04 05 06 07 08 09
    // 10 11 12 13 14 15 16 17 18 19
    // 20 21 22 23 24 25 26 27 28 29

    //* Right
    let mut temp_row = row_index as i32;
    let mut temp_column = column_index as i32 + step as i32;

    if temp_column > max_column_index as i32 {
        temp_row = -1;
        temp_column = -1;
    }

    let right = (temp_row, temp_column);
    //* Right

    //* Left
    temp_row = row_index as i32;
    temp_column = column_index as i32 - 1;
    if temp_column < 0 {
        temp_row = -1;
        temp_column = -1;
    }

    let left = (temp_row, temp_column);
    //* Left

    //* Down
    temp_row = row_index as i32 + 1;
    temp_column = column_index as i32;

    if temp_row > max_row_index as i32 {
        temp_row = -1;
        temp_column = -1;
    }

    let down = (temp_row, temp_column);
    //* Down

    //* Up
    temp_row = row_index as i32 - 1;
    temp_column = column_index as i32;
    if temp_row < 0 {
        temp_row = -1;
        temp_column = -1;
    }

    let up = (temp_row, temp_column);
    //* Up

    //* Right Up
    temp_row = row_index as i32 - 1;
    temp_column = column_index as i32 + 1;
    if temp_row < 0 || temp_column > max_column_index as i32 {
        temp_row = -1;
        temp_column = -1;
    }

    let right_up = (temp_row, temp_column);
    //* Right Up

    //* Right Down
    temp_row = row_index as i32 + 1;
    temp_column = column_index as i32 + 1;
    if temp_row > max_row_index as i32 || temp_column > max_column_index as i32 {
        temp_row = -1;
        temp_column = -1;
    }

    let right_down = (temp_row, temp_column);
    //* Right Down

    //* Left Up
    temp_row = row_index as i32 - 1;
    temp_column = column_index as i32 - 1;
    if temp_row < 0 || temp_column < 0 {
        temp_row = -1;
        temp_column = -1;
    }

    let left_up = (temp_row, temp_column);
    //* Left Up

    //* Left Down
    temp_row = row_index as i32 + 1;
    temp_column = column_index as i32 - 1;
    if temp_row > max_row_index as i32 || temp_column < 0 {
        temp_row = -1;
        temp_column = -1;
    }
    let left_down = (temp_row, temp_column);
    //* Left Down

    // println!("right: {:?}, left: {:?}", right, left);
    // println!("down: {:?}, up: {:?}", down, up);
    // println!("right_up: {:?}, right_down: {:?}", right_up, right_down);
    // println!("left_up: {:?}, left_down: {:?}", left_up, left_down);

    CoordinateIndexStruct {
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

pub fn right_index(
    row_index: usize,
    column_index: usize,
    _max_row_index: usize,
    max_column_index: usize,
    step: usize,
) -> (i32, i32) {
    //* Right
    let mut temp_row = row_index as i32;
    let mut temp_column = column_index as i32 + step as i32;

    if temp_column > max_column_index as i32 {
        temp_row = -1;
        temp_column = -1;
    }

    (temp_row, temp_column)
}

pub fn left_index(
    row_index: usize,
    column_index: usize,
    _max_row_index: usize,
    _max_column_index: usize,
    step: usize,
) -> (i32, i32) {
    //* Left
    let mut temp_row = row_index as i32;
    let mut temp_column = column_index as i32 - step as i32;
    if temp_column < 0 {
        temp_row = -1;
        temp_column = -1;
    }

    (temp_row, temp_column)
    //* Left
}

pub fn down_index(
    row_index: usize,
    column_index: usize,
    max_row_index: usize,
    _max_column_index: usize,
    step: usize,
) -> (i32, i32) {
    //* Down
    let mut temp_row = row_index as i32 + step as i32;
    let mut temp_column = column_index as i32;

    if temp_row > max_row_index as i32 {
        temp_row = -1;
        temp_column = -1;
    }

    (temp_row, temp_column)
    //* Down
}

pub fn up_index(
    row_index: usize,
    column_index: usize,
    _max_row_index: usize,
    _max_column_index: usize,
    step: usize,
) -> (i32, i32) {
    //* Up
    let mut temp_row = row_index as i32 - step as i32;
    let mut temp_column = column_index as i32;
    if temp_row < 0 {
        temp_row = -1;
        temp_column = -1;
    }

    (temp_row, temp_column)
    //* Up
}

pub fn right_up_index(
    row_index: usize,
    column_index: usize,
    _max_row_index: usize,
    max_column_index: usize,
    step: usize,
) -> (i32, i32) {
    //* Right Up
    let mut temp_row = row_index as i32 - step as i32;
    let mut temp_column = column_index as i32 + step as i32;
    if temp_row < 0 || temp_column > max_column_index as i32 {
        temp_row = -1;
        temp_column = -1;
    }

    (temp_row, temp_column)
    //* Right Up
}

pub fn right_down_index(
    row_index: usize,
    column_index: usize,
    max_row_index: usize,
    max_column_index: usize,
    step: usize,
) -> (i32, i32) {
    //* Right Down
    let mut temp_row = row_index as i32 + step as i32;
    let mut temp_column = column_index as i32 + step as i32;

    if temp_row > max_row_index as i32 || temp_column > max_column_index as i32 {
        temp_row = -1;
        temp_column = -1;
    }

    (temp_row, temp_column)
    //* Right Down
}

pub fn left_up_index(
    row_index: usize,
    column_index: usize,
    _max_row_index: usize,
    _max_column_index: usize,
    step: usize,
) -> (i32, i32) {
    //* Left Up
    let mut temp_row = row_index as i32 - step as i32;
    let mut temp_column = column_index as i32 - step as i32;

    if temp_row < 0 || temp_column < 0 {
        temp_row = -1;
        temp_column = -1;
    }

    (temp_row, temp_column)
    //* Left Up
}

pub fn left_down_index(
    row_index: usize,
    column_index: usize,
    max_row_index: usize,
    _max_column_index: usize,
    step: usize,
) -> (i32, i32) {
    //* Left Down
    let mut temp_row = row_index as i32 + step as i32;
    let mut temp_column = column_index as i32 - step as i32;
    if temp_row > max_row_index as i32 || temp_column < 0 {
        temp_row = -1;
        temp_column = -1;
    }
    (temp_row, temp_column)
    //* Left Down
}

#[derive(Debug)]
pub struct CoordinateIndexStruct {
    pub right: (i32, i32),
    pub left: (i32, i32),
    pub down: (i32, i32),
    pub up: (i32, i32),
    pub right_down: (i32, i32),
    pub right_up: (i32, i32),
    pub left_up: (i32, i32),
    pub left_down: (i32, i32),
}
