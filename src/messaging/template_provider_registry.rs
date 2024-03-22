use crate::di::prelude::Builder;
use crate::messaging::_traits::template_provider::TemplateProvider;
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
  pub fn get_template<T: TemplateProvider + 'static>(&self, number: i64) -> Option<String> {
    self.0.get::<T>().map(|provider| provider.get_template(number))
  }
}

/// A builder for the `TemplateProviderRegistry`.
impl Builder for TemplateProviderRegistry {
  type Input = ();
  type Output = TemplateProviderRegistry;

  fn build(_: Self::Input) -> Self::Output {
    TemplateProviderRegistry::new()
  }
}

#[cfg(test)]
mod tests {
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
  fn test_template_provider_registry() {
    test_init();
    let mut registry = TemplateProviderRegistry::new();
    let provider = TestTemplateProvider {
      templates: vec![
        "a tisket".to_string(),
        "a tasket".to_string(),
        "a green and yellow basket".to_string(),
      ],
    };
    registry.set(provider.clone());
    assert_eq!(
      registry.get_template::<TestTemplateProvider>(0),
      Some("a tisket".to_string())
    );
    assert_eq!(
      registry.get_template::<TestTemplateProvider>(1),
      Some("a tasket".to_string())
    );
    assert_eq!(
      registry.get_template::<TestTemplateProvider>(2),
      Some("a green and yellow basket".to_string())
    );
    assert_eq!(
      registry.get_template::<TestTemplateProvider>(3),
      Some("a tisket".to_string())
    );
  }

  #[test]
  fn test_template_provider_registry_builder() {
    test_init();
    let mut container = crate::di::prelude::Container::new();
    container.build::<TemplateProviderRegistry>();
    let binding = container.get::<TemplateProviderRegistry>().unwrap();
    let mut registry = binding.lock().unwrap();
    registry.set::<TestTemplateProvider>(TestTemplateProvider {
      templates: vec![
        "all creatures great and small".to_string(),
        "all things wise and wonderful".to_string(),
      ],
    });
    assert_eq!(
      registry.get_template::<TestTemplateProvider>(0),
      Some("all creatures great and small".to_string())
    );
  }
}
