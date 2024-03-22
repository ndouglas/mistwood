/// This trait describes an object that provides message templates. It may be a
/// struct or an enum variant. It simply takes an integer and returns a message
/// template from its list; normally, it's just going to return the message
/// template at $number % $vector_length. Errors should just not occur at this
/// point.
pub trait TemplateProvider {
  /// Produces a string from the given number.
  fn get_template(&self, number: i64) -> String;
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::test::init as test_init;
  use pretty_assertions::assert_eq;

  #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
  struct TestTemplateProvider {
    templates: Vec<String>,
  }

  impl TemplateProvider for TestTemplateProvider {
    fn get_template(&self, number: i64) -> String {
      self.templates[(number % self.templates.len() as i64) as usize].clone()
    }
  }

  #[test]
  fn test_template_provider() {
    test_init();
    let provider = TestTemplateProvider {
      templates: vec!["Hello, world!".to_string(), "Goodbye, world!".to_string()],
    };
    assert_eq!(provider.get_template(0), "Hello, world!".to_string());
    assert_eq!(provider.get_template(1), "Goodbye, world!".to_string());
  }
}
