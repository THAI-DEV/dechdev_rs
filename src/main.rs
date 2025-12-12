use std::time::Instant;

use dechdev_rs::{example, utils::helper};

fn main() {
    let start_time = Instant::now();
    helper::scroll_console();
    println!("Welcome to DechDev-RS!\n");

    //example::example_random();

    // example::example_string_case();

    // example::example_crypto_ase_gcm();
    // example::example_crypto_aes();

    // example::example_bcrypt();
    // example::example_argon2();

    // example::example_send_message();
    // example::example_send_message_async().await;

    // example_elapsed_time_calculation();

    // example::example_path_file();

    // example::example_calculate_group();
    // example::example_arrange_group();

    // example::example_slice();

    // example::example_csv_mut();
    // example::example_csv_immut();

    // example::example_group();

    println!();
    println!("--- End ({:?}) ---", start_time.elapsed());
}
