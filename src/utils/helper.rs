/// Clears the console screen and positions the cursor at the top-left corner.
///
/// This function uses ANSI escape sequences to:
/// - Clear the entire screen (`[2J`)
/// - Move the cursor to position (1,1) which is the top-left corner (`[1;1H`)
///
/// # Note
/// This function will only work properly in terminals that support ANSI escape sequences.
/// On some Windows systems or terminals without ANSI support, this may not work as expected.
pub fn scroll_console() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
