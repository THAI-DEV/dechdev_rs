use std::time::Instant;

use crate::utils::date_time::calculate_elapsed_duration;

pub fn example_elapsed_time_calculation() {
    //1 days, 5 hours, 3 minutes, 20 seconds, 150 milliseconds
    let start_time = Instant::now()
        - std::time::Duration::from_secs((24 * 60 * 60) + (5 * 60 * 60) + (3 * 60) + 20)
        - std::time::Duration::from_millis(150);

    let (days, hours, minutes, seconds, millis) = calculate_elapsed_duration(start_time);
    println!(
        "Elapsed time: {} days, {} hours, {} minutes, {} seconds, {} milliseconds",
        days, hours, minutes, seconds, millis
    );
}
