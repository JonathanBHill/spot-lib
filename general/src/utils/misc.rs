use term_size;
pub fn get_type<T>(_: &T, short: bool) -> &'static str {
    let full_type = std::any::type_name::<T>();
    if short {
        let split: Vec<&str> = full_type.split("::").collect();
        return split.last().unwrap();
    }
    
    return std::any::type_name::<T>();
}
pub fn print_separator() {
    if let Some((width, _)) = term_size::dimensions() {
        let separator = "-".repeat(width);
        println!("{}", separator);
    } else {
        // Fallback if the terminal size can't be determined
        println!("{}", "-".repeat(80));
    }
}
