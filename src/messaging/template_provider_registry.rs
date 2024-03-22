use crate::di::prelude::Builder;
use crate::messaging::prelude::MessagingError;
use crate::messaging::prelude::TemplateList;
use rand::prelude::*;

/// The Template Provider Registry is a registry of Template Providers.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct TemplateProviderRegistry {
  /// The random number generator.
  #[derivative(Debug = "ignore")]
  pub rng: Box<dyn RngCore>,
}

impl TemplateProviderRegistry {
  /// Create a new instance with a specified random number generator.
  pub fn new() -> Self {
    Self {
      rng: Box::new(thread_rng()),
    }
  }

  /// Get a template from the provider.
  pub fn get_template<T: TemplateList + 'static>(&mut self) -> Result<String, MessagingError> {
    let number = self.rng.gen::<usize>();
    T::get_template(number)
  }
}

impl Default for TemplateProviderRegistry {
  fn default() -> Self {
    Self::new()
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
  use rand::rngs::mock::StepRng;

  struct TestTemplateList;

  impl TemplateList for TestTemplateList {
    const TEMPLATES: &'static [&'static str] = &["a tisket", "a tasket", "a green and yellow basket"];
  }

  #[test]
  fn test_template_provider_registry() {
    test_init();
    let step_rng = StepRng::new(0, 1);
    let mut registry = TemplateProviderRegistry::new();
    registry.rng = Box::new(step_rng);
    assert_eq!(
      registry.get_template::<TestTemplateList>().unwrap(),
      "a tisket".to_string()
    );
    assert_eq!(
      registry.get_template::<TestTemplateList>().unwrap(),
      "a tasket".to_string()
    );
    assert_eq!(
      registry.get_template::<TestTemplateList>().unwrap(),
      "a green and yellow basket".to_string()
    );
    assert_eq!(
      registry.get_template::<TestTemplateList>().unwrap(),
      "a tisket".to_string()
    );
  }

  #[test]
  fn test_template_provider_registry_builder() {
    test_init();
    let mut container = crate::di::prelude::Container::new();
    container.build::<TemplateProviderRegistry>();
    let binding = container.get::<TemplateProviderRegistry>().unwrap();
    let mut registry = binding.lock().unwrap();
    registry.rng = Box::new(StepRng::new(2, 1));
    assert_eq!(
      registry.get_template::<TestTemplateList>().unwrap(),
      "a green and yellow basket".to_string()
    );
  }
}
