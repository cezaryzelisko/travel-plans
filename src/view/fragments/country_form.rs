use askama::Template;

#[derive(Template)]
#[template(path = "fragments/country_form.html")]
pub struct CountryFormTemplate {
    operation: &'static str,
    country_code: String,
    country_name: String,
}

impl CountryFormTemplate {
    pub fn new(operation: &'static str, country_code: String, country_name: String) -> Self {
        Self {
            operation,
            country_code,
            country_name,
        }
    }
}
