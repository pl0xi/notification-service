use handlebars::Handlebars;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TemplateClientError {
    #[error("Failed to get template")]
    FailedToGetTemplate,

    #[error("Error registering template")]
    TemplateRegistrationError,
}

#[derive(Clone)]
pub struct TemplateClient {
    templates: Handlebars<'static>,
}

impl TemplateClient {
    pub fn new(templates: Handlebars<'static>) -> Self {
        Self { templates }
    }

    pub fn get_template_filled<T: Serialize>(&self, template_name: &str, template_args: T) -> Result<String, TemplateClientError> {
        match self.templates.render(template_name, &template_args) {
            Ok(rendered_template) => Ok(rendered_template),
            Err(_) => Err(TemplateClientError::FailedToGetTemplate),
        }
    }

    #[allow(unused)]
    pub fn upsert_template(&mut self, template_name: &str, template: &str) -> Result<(), TemplateClientError> {
        match self.templates.register_template_string(template_name, template) {
            Ok(_) => Ok(()),
            Err(e) => Err(TemplateClientError::TemplateRegistrationError),
        }
    }
}
