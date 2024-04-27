pub fn format_for_nut(birth_date: &str) -> String {
    // YYYY-MM-DD -> DD/MM/YYYY
    let mut date = birth_date.split("-");
    let year = date.next().unwrap();
    let month = date.next().unwrap();
    let day = date.next().unwrap();
    format!("{}/{}/{}", day, month, year)
}