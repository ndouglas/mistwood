use crate::messaging::_error::MessagingError;

/// This trait describes a struct that provides message templates. It simply
/// takes an integer and returns a message template from its list; normally,
/// it's just going to return the message template at $number % $vector_length.
/// Errors should just not occur at this point.
pub trait TemplateProvider {
  /// The list of templates provided.
  ///
  /// Each template is written in Handlebars syntax.
  ///
  /// Examples:
  /// - "Hello, world!"
  /// - "Thank you for playing {{ game_name }}!"
  const TEMPLATES: &'static [&'static str];

  /// Get a template.
  fn get_template(number: usize) -> Result<String, MessagingError> {
    if Self::TEMPLATES.is_empty() {
      return Err(MessagingError::NoTemplatesInTemplateProvider);
    }
    let length = Self::TEMPLATES.len();
    let index = number % length;
    let template = Self::TEMPLATES[index].to_string();
    Ok(template)
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::test::init as test_init;
  use pretty_assertions::assert_eq;

  struct TestTemplateProvider;

  impl TemplateProvider for TestTemplateProvider {
    const TEMPLATES: &'static [&'static str] = &["Hello, world!", "Goodbye, world!"];
  }

  #[test]
  fn test_template_provider() {
    test_init();
    assert_eq!(
      TestTemplateProvider::get_template(0).unwrap(),
      "Hello, world!".to_string()
    );
    assert_eq!(
      TestTemplateProvider::get_template(1).unwrap(),
      "Goodbye, world!".to_string()
    );
  }
}
