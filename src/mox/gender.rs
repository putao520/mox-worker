pub fn gender_to_string(id: u32) -> String {
    match id {
        1 => "Femenino",
        2 => "Masculino",
        _ => "X",
    }.to_string()
}