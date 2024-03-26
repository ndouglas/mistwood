/// Create a message from a template struct and data.
#[macro_export]
macro_rules! create_message {
  // Variant with data
  ($template_provider:expr, $template_struct:ident, $gravity:expr, {$($data_key:ident: $data_value:expr),* $(,)?}) => {{
    paste::paste! {
      // Create a struct to hold the data for the template.
      // Missing fields will cause a compile error.
      type TemplateDataType = <$template_struct as $crate::messaging::traits::data_template::DataTemplate>::DataType;
      let data = TemplateDataType {
        $($data_key: $data_value),*
      };
      // Serialize the data to JSON.
      let data_json = serde_json::to_value(&data).expect(&format!("failed to serialize {:#?}", data));
      let message_gravity = $gravity;
      let template_string = $template_provider.get_template::<$template_struct>()
        .expect("failed to retrieve template");
      $crate::messaging::prelude::Message {
        template: template_string,
        gravity: message_gravity,
        data: data_json,
        metadata: None,
      }
    }
  }};
  // Variant without data
  ($template_provider:expr, $template_struct:ident, $gravity:expr) => {{
    paste::paste! {
      // Force a compile error if the template struct does not implement the
      // SimpleTemplate trait. This means that the template requires data.
      let _: &dyn $crate::messaging::traits::simple_template::SimpleTemplate = &$template_struct;
      let data_json = serde_json::Value::Null;
      let message_gravity = $gravity;
      let template_string = $template_provider.get_template::<$template_struct>()
        .expect("failed to retrieve template");
      $crate::messaging::prelude::Message {
        template: template_string,
        gravity: message_gravity,
        data: data_json,
        metadata: None,
      }
    }
  }};
}

/// Create an info message from a template struct and data.
#[macro_export]
macro_rules! info_message {
  // Variant with data
  ($template_provider:expr, $template_struct:ident, {$($data_key:ident: $data_value:expr),* $(,)?}) => {
    create_message!($template_provider, $template_struct, $crate::messaging::prelude::Gravity::Info, {$($data_key: $data_value),*})
  };
  // Variant without data
  ($template_provider:expr, $template_struct:ident) => {
    create_message!($template_provider, $template_struct, $crate::messaging::prelude::Gravity::Info)
  };
}

/// Create a notice message from a template struct and data.
#[macro_export]
macro_rules! notice_message {
  // Variant with data
  ($template_provider:expr, $template_struct:ident, {$($data_key:ident: $data_value:expr),* $(,)?}) => {
    create_message!($template_provider, $template_struct, $crate::messaging::prelude::Gravity::Notice, {$($data_key: $data_value),*})
  };
  // Variant without data
  ($template_provider:expr, $template_struct:ident) => {
    create_message!($template_provider, $template_struct, $crate::messaging::prelude::Gravity::Notice)
  };
}

/// Create a warning message from a template struct and data.
#[macro_export]
macro_rules! warning_message {
  // Variant with data
  ($template_provider:expr, $template_struct:ident, {$($data_key:ident: $data_value:expr),* $(,)?}) => {
    create_message!($template_provider, $template_struct, $crate::messaging::prelude::Gravity::Warning, {$($data_key: $data_value),*})
  };
  // Variant without data
  ($template_provider:expr, $template_struct:ident) => {
    create_message!($template_provider, $template_struct, $crate::messaging::prelude::Gravity::Warning)
  };
}

/// Create an alert message from a template struct and data.
#[macro_export]
macro_rules! alert_message {
  // Variant with data
  ($template_provider:expr, $template_struct:ident, {$($data_key:ident: $data_value:expr),* $(,)?}) => {
    create_message!($template_provider, $template_struct, $crate::messaging::prelude::Gravity::Alert, {$($data_key: $data_value),*})
  };
  // Variant without data
  ($template_provider:expr, $template_struct:ident) => {
    create_message!($template_provider, $template_struct, $crate::messaging::prelude::Gravity::Alert)
  };
}

/// Create a critical message from a template struct and data.
#[macro_export]
macro_rules! critical_message {
  // Variant with data
  ($template_provider:expr, $template_struct:ident, {$($data_key:ident: $data_value:expr),* $(,)?}) => {
    create_message!($template_provider, $template_struct, $crate::messaging::prelude::Gravity::Critical, {$($data_key: $data_value),*})
  };
  // Variant without data
  ($template_provider:expr, $template_struct:ident) => {
    create_message!($template_provider, $template_struct, $crate::messaging::prelude::Gravity::Critical)
  };
}

/// Create a fatal message from a template struct and data.
#[macro_export]
macro_rules! fatal_message {
  // Variant with data
  ($template_provider:expr, $template_struct:ident, {$($data_key:ident: $data_value:expr),* $(,)?}) => {{
    create_message!($template_provider, $template_struct, $crate::messaging::prelude::Gravity::Fatal, {$($data_key: $data_value),*})
  }};
  // Variant without data
  ($template_provider:expr, $template_struct:ident) => {
    create_message!($template_provider, $template_struct, $crate::messaging::prelude::Gravity::Fatal)
  };
}

#[cfg(test)]
mod test {
  #[allow(unused_imports)]
  use super::*;
  use crate::messaging::prelude::*;
  use crate::messaging::template_provider::TemplateProvider;
  use crate::messaging::traits::template_processor::TemplateProcessor as TemplateProcessorTrait;
  use crate::template_list;
  use crate::test::init as test_init;
  use pretty_assertions::assert_eq;
  use rand::rngs::mock::StepRng;
  use serde_json::json;

  template_list!(TestTemplateList, ["Hello, world!", "Goodbye, world!"]);
  template_list!(TestTemplateList2, ["Hello, {{ name }}!", "Goodbye, {{ name }}!"], {name: String, age: u8});

  #[test]
  fn test_create_message() {
    test_init();
    let mut template_provider = TemplateProvider::new();
    let step_rng = StepRng::new(0, 0);
    template_provider.rng = Box::new(step_rng);
    let message = create_message!(template_provider, TestTemplateList, Gravity::Info);
    assert_eq!(message.template, "Hello, world!");
    assert_eq!(message.gravity, Gravity::Info);
    assert_eq!(message.data, serde_json::Value::Null);
  }

  #[test]
  fn test_create_message_with_data() {
    test_init();
    let mut template_provider = TemplateProvider::new();
    let step_rng = StepRng::new(0, 0);
    template_provider.rng = Box::new(step_rng);
    let message =
      create_message!(template_provider, TestTemplateList2, Gravity::Info, {name: "Alice".to_string(), age: 42});
    let rendered = TemplateProcessor::new().process_message(&message).unwrap();
    assert_eq!(message.template, "Hello, {{ name }}!");
    assert_eq!(message.gravity, Gravity::Info);
    assert_eq!(message.data, json!({"name": "Alice", "age": 42}));
    assert_eq!(rendered, "Hello, Alice!");
  }

  fn setup_template_provider() -> TemplateProvider {
    test_init();
    let mut template_provider = TemplateProvider::new();
    let step_rng = StepRng::new(0, 0);
    template_provider.rng = Box::new(step_rng);
    template_provider
  }

  fn check_message(message: &Message, template: &str, gravity: Gravity, data: serde_json::Value, expected: &str) {
    let rendered = TemplateProcessor::new().process_message(&message).unwrap();
    assert_eq!(message.template, template);
    assert_eq!(message.gravity, gravity);
    assert_eq!(message.data, data);
    assert_eq!(rendered, expected);
  }

  #[test]
  fn test_info_message() {
    let mut template_provider = setup_template_provider();
    let message = info_message!(template_provider, TestTemplateList);
    check_message(
      &message,
      "Hello, world!",
      Gravity::Info,
      serde_json::Value::Null,
      "Hello, world!",
    );
  }

  #[test]
  fn test_info_message2() {
    let mut template_provider = setup_template_provider();
    let message = info_message!(template_provider, TestTemplateList2, {name: "Bob".to_string(), age: 42});
    check_message(
      &message,
      "Hello, {{ name }}!",
      Gravity::Info,
      json!({"name": "Bob", "age": 42}),
      "Hello, Bob!",
    );
  }

  #[test]
  fn test_notice_message() {
    let mut template_provider = setup_template_provider();
    let message = notice_message!(template_provider, TestTemplateList2, {name: "Alice".to_string(), age: 42});
    check_message(
      &message,
      "Hello, {{ name }}!",
      Gravity::Notice,
      json!({"name": "Alice", "age": 42}),
      "Hello, Alice!",
    );
  }

  #[test]
  fn test_warning_message() {
    let mut template_provider = setup_template_provider();
    let message = warning_message!(template_provider, TestTemplateList2, {name: "Alice".to_string(), age: 42});
    check_message(
      &message,
      "Hello, {{ name }}!",
      Gravity::Warning,
      json!({"name": "Alice", "age": 42}),
      "Hello, Alice!",
    );
  }

  #[test]
  fn test_alert_message() {
    let mut template_provider = setup_template_provider();
    let message = alert_message!(template_provider, TestTemplateList2, {name: "Alice".to_string(), age: 42});
    check_message(
      &message,
      "Hello, {{ name }}!",
      Gravity::Alert,
      json!({"name": "Alice", "age": 42}),
      "Hello, Alice!",
    );
  }

  #[test]
  fn test_critical_message() {
    let mut template_provider = setup_template_provider();
    let message = critical_message!(template_provider, TestTemplateList2, {name: "Alice".to_string(), age: 42});
    check_message(
      &message,
      "Hello, {{ name }}!",
      Gravity::Critical,
      json!({"name": "Alice", "age": 42}),
      "Hello, Alice!",
    );
  }

  #[test]
  fn test_fatal_message() {
    let mut template_provider = setup_template_provider();
    let message = fatal_message!(template_provider, TestTemplateList2, {name: "Alice".to_string(), age: 42});
    check_message(
      &message,
      "Hello, {{ name }}!",
      Gravity::Fatal,
      json!({"name": "Alice", "age": 42}),
      "Hello, Alice!",
    );
  }
}
