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

#[cfg(test)]
mod tests {
    use super::*;
    use handlebars::Handlebars;
    use serde_json::json;

    #[test]
    fn test_manager_new() {
        let handlebars = Handlebars::new();
        let manager = Manager::new(handlebars);
        assert!(manager.templates.get_template("non_existent").is_none());
    }

    #[test]
    fn test_get_template_filled_success() {
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("test_template", "Hello {{name}}!").unwrap();
        let manager = Manager::new(handlebars);

        let result = manager.get_template_filled("test_template", json!({"name": "World"}));
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello World!");
    }

    #[test]
    fn test_get_template_filled_error() {
        let handlebars = Handlebars::new(); // No templates registered
        let manager = Manager::new(handlebars);

        let result = manager.get_template_filled("non_existent", json!({"name": "World"}));
        assert!(matches!(result, Err(ManagerError::FailedToGetTemplate)));
    }

    #[test]
    fn test_upsert_template_success() {
        let mut manager = Manager::new(Handlebars::new());
        let result = manager.upsert_template("test_template", "Hello {{name}}!");

        assert!(result.is_ok());
        assert!(manager.templates.has_template("test_template"));
    }

    #[test]
    fn test_upsert_template_error() {
        let mut manager = Manager::new(Handlebars::new());
        let result = manager.upsert_template("test_template", "{{#invalid}}");

        assert!(matches!(result, Err(ManagerError::TemplateRegistrationError)));
    }
}
