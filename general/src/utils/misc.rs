use term_size;
pub fn print_type_of<T>(_: &T) -> &'static str {
    // #[cfg(test)]
    // println!("{}", std::any::type_name::<T>());
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
