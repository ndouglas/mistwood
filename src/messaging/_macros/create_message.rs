/// Create a message from a template struct and data.
#[macro_export]
macro_rules! create_message {
  // Variant with data
  ($template_struct:ident, $gravity:expr, {$($data_key:ident: $data_value:expr),* $(,)?}) => {{
    paste::paste! {
      // Force a compile error if the template struct does not implement the
      // DataTemplate trait. This means the template does not require data.
      type TemplateDataType = <$template_struct as $crate::messaging::_traits::data_template::DataTemplate>::DataType;
      let data = TemplateDataType {
        $($data_key: $data_value),*
      };
      let data_json = serde_json::to_value(&data).expect(&format!("failed to serialize {:#?}", data));
      let message_gravity = $gravity;
      // @todo: Remove this once Template Provider service is implemented.
      let mut template_provider = $crate::messaging::template_provider::TemplateProvider::new();
      // @todo: Remove this once RNG service is implemented.
      let step_rng = rand::rngs::mock::StepRng::new(0, 0);
      template_provider.rng = Box::new(step_rng);
      let template_string = template_provider.get_template::<$template_struct>()
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
  ($template_struct:ident, $gravity:expr) => {{
    paste::paste! {
      // Force a compile error if the template struct does not implement the
      // SimpleTemplate trait. This means that the template requires data.
      let _: &dyn $crate::messaging::_traits::simple_template::SimpleTemplate = &$template_struct;
      let data_json = serde_json::Value::Null;
      let message_gravity = $gravity;
      // @todo: Remove this once Template Provider service is implemented.
      let mut template_provider = $crate::messaging::template_provider::TemplateProvider::new();
      // @todo: Remove this once RNG service is implemented.
      let step_rng = rand::rngs::mock::StepRng::new(0, 0);
      template_provider.rng = Box::new(step_rng);
      let template_string = template_provider.get_template::<$template_struct>()
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
  ($template_struct:ident, {$($data_key:ident: $data_value:expr),* $(,)?}) => {
    create_message!($template_struct, $crate::messaging::prelude::Gravity::Info, {$($data_key: $data_value),*})
  };
  // Variant without data
  ($template_struct:ident) => {
    create_message!($template_struct, $crate::messaging::prelude::Gravity::Info)
  };
}

/// Create a notice message from a template struct and data.
#[macro_export]
macro_rules! notice_message {
  // Variant with data
  ($template_struct:ident, {$($data_key:ident: $data_value:expr),* $(,)?}) => {
    create_message!($template_struct, $crate::messaging::prelude::Gravity::Notice, {$($data_key: $data_value),*})
  };
  // Variant without data
  ($template_struct:ident) => {
    create_message!($template_struct, $crate::messaging::prelude::Gravity::Notice)
  };
}

/// Create a warning message from a template struct and data.
#[macro_export]
macro_rules! warning_message {
  // Variant with data
  ($template_struct:ident, {$($data_key:ident: $data_value:expr),* $(,)?}) => {
    create_message!($template_struct, $crate::messaging::prelude::Gravity::Warning, {$($data_key: $data_value),*})
  };
  // Variant without data
  ($template_struct:ident) => {
    create_message!($template_struct, $crate::messaging::prelude::Gravity::Warning)
  };
}

/// Create an alert message from a template struct and data.
#[macro_export]
macro_rules! alert_message {
  // Variant with data
  ($template_struct:ident, {$($data_key:ident: $data_value:expr),* $(,)?}) => {
    create_message!($template_struct, $crate::messaging::prelude::Gravity::Alert, {$($data_key: $data_value),*})
  };
  // Variant without data
  ($template_struct:ident) => {
    create_message!($template_struct, $crate::messaging::prelude::Gravity::Alert)
  };
}

/// Create a critical message from a template struct and data.
#[macro_export]
macro_rules! critical_message {
  // Variant with data
  ($template_struct:ident, {$($data_key:ident: $data_value:expr),* $(,)?}) => {
    create_message!($template_struct, $crate::messaging::prelude::Gravity::Critical, {$($data_key: $data_value),*})
  };
  // Variant without data
  ($template_struct:ident) => {
    create_message!($template_struct, $crate::messaging::prelude::Gravity::Critical)
  };
}

/// Create a fatal message from a template struct and data.
#[macro_export]
macro_rules! fatal_message {
  // Variant with data
  ($template_struct:ident, {$($data_key:ident: $data_value:expr),* $(,)?}) => {{
    create_message!($template_struct, $crate::messaging::prelude::Gravity::Fatal, {$($data_key: $data_value),*})
  }};
  // Variant without data
  ($template_struct:ident) => {
    create_message!($template_struct, $crate::messaging::prelude::Gravity::Fatal)
  };
}

#[cfg(test)]
mod test {
  #[allow(unused_imports)]
  use super::*;
  use crate::messaging::_traits::template_processor::TemplateProcessor as TemplateProcessorTrait;
  use crate::messaging::prelude::*;
  use crate::template_list;
  use crate::test::init as test_init;
  use pretty_assertions::assert_eq;
  use serde_json::json;

  template_list!(TestTemplateList, ["Hello, world!", "Goodbye, world!"]);
  template_list!(TestTemplateList2, ["Hello, {{ name }}!", "Goodbye, {{ name }}!"], {name: String, age: u8});

  #[test]
  fn test_create_message() {
    test_init();
    let message = create_message!(TestTemplateList, Gravity::Info);
    assert_eq!(message.template, "Hello, world!");
    assert_eq!(message.gravity, Gravity::Info);
    assert_eq!(message.data, serde_json::Value::Null);
  }

  #[test]
  fn test_create_message_with_data() {
    test_init();
    let message = create_message!(TestTemplateList2, Gravity::Info, {name: "Alice".to_string(), age: 42});
    let rendered = TemplateProcessor::new().process_message(&message).unwrap();
    assert_eq!(message.template, "Hello, {{ name }}!");
    assert_eq!(message.gravity, Gravity::Info);
    assert_eq!(message.data, json!({"name": "Alice", "age": 42}));
    assert_eq!(rendered, "Hello, Alice!");
  }

  #[test]
  fn test_info_message() {
    test_init();
    let message = info_message!(TestTemplateList);
    let rendered = TemplateProcessor::new().process_message(&message).unwrap();
    assert_eq!(message.template, "Hello, world!");
    assert_eq!(message.gravity, Gravity::Info);
    assert_eq!(message.data, serde_json::Value::Null);
    assert_eq!(rendered, "Hello, world!");
  }

  #[test]
  fn test_info_message2() {
    test_init();
    let message = info_message!(TestTemplateList2, {name: "Bob".to_string(), age: 42});
    let rendered = TemplateProcessor::new().process_message(&message).unwrap();
    assert_eq!(message.template, "Hello, {{ name }}!");
    assert_eq!(message.gravity, Gravity::Info);
    assert_eq!(message.data, json!({"name": "Bob", "age": 42}));
    assert_eq!(rendered, "Hello, Bob!");
  }

  #[test]
  fn test_notice_message() {
    test_init();
    let message = notice_message!(TestTemplateList2, {name: "Alice".to_string(), age: 42});
    let rendered = TemplateProcessor::new().process_message(&message).unwrap();
    assert_eq!(message.template, "Hello, {{ name }}!");
    assert_eq!(message.gravity, Gravity::Notice);
    assert_eq!(message.data, json!({"name": "Alice", "age": 42}));
    assert_eq!(rendered, "Hello, Alice!");
  }

  #[test]
  fn test_warning_message() {
    test_init();
    let message = warning_message!(TestTemplateList2, {name: "Alice".to_string(), age:
42});
    let rendered = TemplateProcessor::new().process_message(&message).unwrap();
    assert_eq!(message.template, "Hello, {{ name }}!");
    assert_eq!(message.gravity, Gravity::Warning);
    assert_eq!(message.data, json!({"name": "Alice", "age": 42}));
    assert_eq!(rendered, "Hello, Alice!");
  }

  #[test]
  fn test_alert_message() {
    test_init();
    let message = alert_message!(TestTemplateList2, {name: "Alice".to_string(), age: 42});
    let rendered = TemplateProcessor::new().process_message(&message).unwrap();
    assert_eq!(message.template, "Hello, {{ name }}!");
    assert_eq!(message.gravity, Gravity::Alert);
    assert_eq!(message.data, json!({"name": "Alice", "age": 42}));
    assert_eq!(rendered, "Hello, Alice!");
  }

  #[test]
  fn test_critical_message() {
    test_init();
    let message = critical_message!(TestTemplateList2, {name: "Alice".to_string(), age: 42});
    let rendered = TemplateProcessor::new().process_message(&message).unwrap();
    assert_eq!(message.template, "Hello, {{ name }}!");
    assert_eq!(message.gravity, Gravity::Critical);
    assert_eq!(message.data, json!({"name": "Alice", "age": 42}));
    assert_eq!(rendered, "Hello, Alice!");
  }

  #[test]
  fn test_fatal_message() {
    test_init();
    let message = fatal_message!(TestTemplateList2, {name: "Alice".to_string(), age: 42});
    let rendered = TemplateProcessor::new().process_message(&message).unwrap();
    assert_eq!(message.template, "Hello, {{ name }}!");
    assert_eq!(message.gravity, Gravity::Fatal);
    assert_eq!(message.data, json!({"name": "Alice", "age": 42}));
    assert_eq!(rendered, "Hello, Alice!");
  }
}
