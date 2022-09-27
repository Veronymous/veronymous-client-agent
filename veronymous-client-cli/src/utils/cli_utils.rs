pub fn get_user_input() -> String {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // Remove next line character
    input.pop();

    input
}

pub fn get_password() -> String {
    rpassword::read_password().unwrap()
}
