use std::fmt::Display;

///  Prints a message to the console with a specified color code and prefix.
/// 
/// # Arguments 
/// 
/// * `color_code`: Color code for the text (e.g., "31" for red).
/// * `prefix`:  A prefix to be displayed before the message.
/// * `message`:  The message to be printed, which implements the `Display` trait.
/// 
/// returns: () 
/// 
/// # Examples 
/// 
/// ```
/// 
/// ```
pub fn print_ln_with_color<T: Display>(color_code: &str, prefix: &str, message: T) {
    println!("\x1b[{}m{}: {}\x1b[0m", color_code, prefix, message);
}

