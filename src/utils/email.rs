pub struct Email {
    pub to: String,
    pub subject: String,
    pub html_body: String,
    pub attachment: Option<Vec<u8>>,
}
