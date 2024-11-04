pub trait HtmlSanitize {
    fn html_sanitize(&self) -> String;
}

impl HtmlSanitize for String {
    fn html_sanitize(&self) -> String {
        self.replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('&', "&amp;")
    }
}
