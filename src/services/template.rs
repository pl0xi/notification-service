use handlebars::Handlebars;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ManagerError {
    #[error("Failed to get template")]
    FailedToGetTemplate,

    #[error("Error registering template")]
    TemplateRegistrationError,
}

#[derive(Clone)]
pub struct Manager {
    templates: Handlebars<'static>,
}

impl Manager {
    /// Creates a new template manager.
    ///
    /// # Panics
    ///
    /// Panics if the templates cannot be registered.
    #[must_use]
    pub fn new(templates: Handlebars<'static>) -> Self {
        Self { templates }
    }

    /// Gets a filled template.
    ///
    /// # Errors
    ///
    /// Returns `ManagerError::FailedToGetTemplate` if the template cannot be retrieved.
    pub fn get_template_filled<T: Serialize>(&self, template_name: &str, template_args: T) -> Result<String, ManagerError> {
        match self.templates.render(template_name, &template_args) {
            Ok(rendered_template) => Ok(rendered_template),
            Err(_) => Err(ManagerError::FailedToGetTemplate),
        }
    }

    /// Upserts a template.
    ///
    /// # Errors
    ///
    /// Returns `ManagerError::TemplateRegistrationError` if the template cannot be registered.
    #[allow(unused)]
    pub fn upsert_template(&mut self, template_name: &str, template: &str) -> Result<(), ManagerError> {
        match self.templates.register_template_string(template_name, template) {
            Ok(()) => Ok(()),
            Err(_) => Err(ManagerError::TemplateRegistrationError),
        }
    }
}
