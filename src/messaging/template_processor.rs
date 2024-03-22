use crate::di::prelude::Builder;

/// We use Handlebars to process templates.
pub use handlebars::Handlebars as TemplateProcessor;

/// A builder for the `TemplateProcessor`.
impl Builder for TemplateProcessor<'static> {
  type Input = ();
  type Output = TemplateProcessor<'static>;

  fn build(_: Self::Input) -> Self::Output {
    let mut result = TemplateProcessor::new();
    #[cfg(debug_assertions)]
    {
      result.set_dev_mode(true);
    }
    result
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::di::prelude::Container;
  use crate::test::init as test_init;
  use pretty_assertions::assert_eq;
  use serde_json::json;

  #[test]
  fn test_template_processor() {
    test_init();
    let mut container = Container::new();
    container.build::<TemplateProcessor>();
    let binding = container.get::<TemplateProcessor>().unwrap();
    let template_processor = binding.lock().unwrap();
    assert_eq!(template_processor.dev_mode(), cfg!(debug_assertions));
    assert_eq!(
      template_processor
        .render_template("Hello {{name}}", &json!({"name": "foo"}))
        .unwrap(),
      "Hello foo"
    );
  }
}