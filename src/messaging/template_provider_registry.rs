use crate::messaging::_traits::template_provider::TemplateProvider;
use crate::messaging::prelude::Template;
use crate::prelude::TypeMap;

/// The Template Provider Registry is a registry of Template Providers.
#[derive(Debug, Default)]
pub struct TemplateProviderRegistry(pub TypeMap);

impl TemplateProviderRegistry {
  /// Create a new, empty `TemplateProviderRegistry`.
  pub fn new() -> Self {
    Self(TypeMap::new())
  }

  /// Set a template provider.
  pub fn set<T: TemplateProvider + 'static>(&mut self, t: T) {
    self.0.set::<T>(t);
  }

  /// Get a template from the provider.
  pub fn get_template<T: TemplateProvider + 'static>(&self, number: i64) -> Option<Template> {
    self.0.get::<T>().map(|provider| provider.get_template(number))
  }
}

#[cfg(test)]
mod tests {
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
  fn test_template_provider_registry() {
    test_init();
    let mut registry = TemplateProviderRegistry::new();
    let provider = TestTemplateProvider {
      templates: vec![
        Template::Static("a static string"),
        Template::Borrowed("a borrowed string"),
        Template::Owned("an owned string".to_string()),
      ],
    };
    registry.set(provider.clone());
    assert_eq!(
      registry.get_template::<TestTemplateProvider>(0),
      Some(Template::Static("a static string"))
    );
    assert_eq!(
      registry.get_template::<TestTemplateProvider>(1),
      Some(Template::Borrowed("a borrowed string"))
    );
    assert_eq!(
      registry.get_template::<TestTemplateProvider>(2),
      Some(Template::Owned("an owned string".to_string()))
    );
    assert_eq!(
      registry.get_template::<TestTemplateProvider>(3),
      Some(Template::Static("a static string"))
    );
  }
}
