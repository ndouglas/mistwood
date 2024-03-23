use crate::di::prelude::Builder;
use crate::di::prelude::DiError;
use crate::messaging::_traits::template_processor::TemplateProcessor as TemplateProcessorTrait;
use crate::messaging::prelude::Message;
use crate::messaging::prelude::MessagingError;

/// We use Handlebars to process templates.
pub use handlebars::Handlebars as TemplateProcessor;

impl TemplateProcessorTrait for TemplateProcessor<'static> {
  /// Process a message.
  fn process_message(&self, message: &Message) -> Result<String, MessagingError> {
    let template = &message.template;
    let data = &message.data;
    Ok(self.render_template(template, data)?)
  }
}

/// A builder for the `TemplateProcessor`.
impl Builder for TemplateProcessor<'static> {
  type Input = ();
  type Output = TemplateProcessor<'static>;

  fn build(_: Self::Input) -> Result<Self::Output, DiError> {
    #[allow(unused_mut)]
    let mut result = TemplateProcessor::new();
    #[cfg(debug_assertions)]
    {
      result.set_dev_mode(true);
    }
    Ok(result)
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
    container.build::<TemplateProcessor>().unwrap();
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
