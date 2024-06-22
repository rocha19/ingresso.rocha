use regex::Regex;

#[derive(Clone, Debug)]
pub struct Email {
    value: String,
}

impl Email {
    pub fn new(value: String) -> Result<Email, String> {
        let email_regex = Regex::new(r"^[\w\.-]+@[\w\.-]+\.\w+$").unwrap();

        match email_regex.is_match(&value) {
            true => Ok(Email { value }),
            false => Err("Invalid email format".to_string()),
        }
    }

    pub fn get_value(&self) -> String {
        self.value.clone()
    }
}
