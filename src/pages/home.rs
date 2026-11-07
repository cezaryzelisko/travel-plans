use askama::Template;

use crate::domain::country::Country;

#[derive(Template)]
#[template(path = "home.html")]
pub struct HomeTemplate<'a> {
    countries: &'a [Country],
}

impl<'a> HomeTemplate<'a> {
    pub fn new(countries: &'a [Country]) -> Self {
        Self { countries }
    }
}
