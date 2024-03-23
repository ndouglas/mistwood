use crate::di::prelude::Builder;
use crate::di::prelude::DiError;
use crate::messaging::prelude::MessagingError;
use crate::messaging::prelude::TemplateList;
use rand::prelude::*;

/// The Template Provider proxies requests to Template Lists.
///
/// It provides a random number generator and a method to get a template from a
/// template list.
#[derive(Derivative)]
#[derivative(Debug)]
pub struct TemplateProvider {
  /// The random number generator.
  #[derivative(Debug = "ignore")]
  pub rng: Box<dyn RngCore>,
}

impl TemplateProvider {
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

impl Default for TemplateProvider {
  fn default() -> Self {
    Self::new()
  }
}

/// A builder for the `TemplateProvider`.
impl Builder for TemplateProvider {
  type Input = ();
  type Output = TemplateProvider;

  fn build(_: Self::Input) -> Result<Self::Output, DiError> {
    Ok(TemplateProvider::new())
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
    let mut registry = TemplateProvider::new();
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
    container.build::<TemplateProvider>().unwrap();
    let binding = container.get::<TemplateProvider>().unwrap();
    let mut registry = binding.lock().unwrap();
    registry.rng = Box::new(StepRng::new(2, 1));
    assert_eq!(
      registry.get_template::<TestTemplateList>().unwrap(),
      "a green and yellow basket".to_string()
    );
  }
}
