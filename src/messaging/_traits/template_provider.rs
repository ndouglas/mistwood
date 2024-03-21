use crate::messaging::prelude::Template;

/// This trait describes an object that provides message templates. It may be a
/// struct or an enum variant. It simply takes an integer and returns a message
/// template from its list; normally, it's just going to return the message
/// template at $number % $vector_length. Errors should just not occur at this
/// point.
pub trait TemplateProvider {
  /// Produces a string from the given number.
  fn get_template(&self, number: i64) -> Template;
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::test::init as test_init;
  use pretty_assertions::assert_eq;

  #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
  struct TestTemplateProvider {
    templates: Vec<Template<'static>>,
  }

  impl TemplateProvider for TestTemplateProvider {
    fn get_template(&self, number: i64) -> Template {
      self.templates[(number % self.templates.len() as i64) as usize].clone()
    }
  }

  #[test]
  fn test_template_provider() {
    test_init();
    let provider = TestTemplateProvider {
      templates: vec![
        Template::Static("Hello, world!"),
        Template::Borrowed("Hello, world!"),
        Template::Owned("Hello, world!".to_string()),
      ],
    };
    assert_eq!(provider.get_template(0), Template::Static("Hello, world!"));
    assert_eq!(provider.get_template(1), Template::Borrowed("Hello, world!"));
    assert_eq!(provider.get_template(2), Template::Owned("Hello, world!".to_string()));
    assert_eq!(provider.get_template(3), Template::Static("Hello, world!"));
  }

  /// This struct contains a static string.
  struct StaticProvider;

  /// This struct contains a string that is borrowed from another object.
  struct BorrowedProvider<'a> {
    data: &'a str,
  }

  /// This struct contains a string that is owned by the object.
  struct OwnedProvider {
    data: String,
  }

  impl TemplateProvider for StaticProvider {
    /// Returns a static string template.
    fn get_template(&self, _number: i64) -> Template {
      Template::Static("I am a static string")
    }
  }

  impl TemplateProvider for BorrowedProvider<'_> {
    /// Returns a borrowed string template.
    fn get_template(&self, _number: i64) -> Template {
      Template::Borrowed(self.data)
    }
  }

  impl TemplateProvider for OwnedProvider {
    /// Returns an owned string template.
    fn get_template(&self, _number: i64) -> Template {
      Template::Owned(self.data.clone())
    }
  }

  struct StringOwner {
    data: String,
  }

  #[test]
  fn test_providers() {
    test_init();
    let string_owner = StringOwner {
      data: "I am a borrowed string".to_string(),
    };
    let static_provider = StaticProvider;
    let borrowed_provider = BorrowedProvider {
      data: &string_owner.data,
    };
    let owned_provider = OwnedProvider {
      data: "I am an owned string".to_string(),
    };
    assert_eq!(
      static_provider.get_template(0),
      Template::Static("I am a static string")
    );
    assert_eq!(
      borrowed_provider.get_template(0),
      Template::Borrowed("I am a borrowed string")
    );
    assert_eq!(
      owned_provider.get_template(0),
      Template::Owned("I am an owned string".to_string())
    );
  }
}
