use askama::Template;

// Define a struct for the registration template
#[derive(Template)]
#[template(path = "register.html")]
pub struct RegisterTemplate;
