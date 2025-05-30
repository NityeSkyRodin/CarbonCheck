/// # Error Handling Module
/// 
/// # Arguments 
/// 
/// * `error_msg`:  A string containing the error message to be displayed.
/// 
/// returns: () 
/// 
/// # Examples 
/// 
/// ```
/// 
/// ```
pub fn handle_error(error_msg: String){
    eprintln!("\x1b[31mError: {}\x1b[0m", error_msg);
}