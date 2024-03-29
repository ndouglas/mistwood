/// Define a list of templates for a given struct, with an optional schema.
#[macro_export]
macro_rules! template_list {
  // Variant with data schema
  ($struct_name:ident, [$($template:expr),* $(,)?], { $($field_name:ident: $field_type:ty),* $(,)? }) => {
    paste::paste! {
      /// This data type is used to store the data for the template.
      /// You specify the fields and their types in the macro call.
      #[derive(serde::Serialize, Debug)]
      #[allow(missing_docs,missing_copy_implementations,missing_debug_implementations,unreachable_pub)]
      pub struct [<$struct_name DataType>] {
        $(pub $field_name: $field_type),*
      }

      /// This struct is used to store the template list.
      #[allow(missing_docs,missing_copy_implementations,missing_debug_implementations,unreachable_pub)]
      pub struct $struct_name;

      /// Implement the TemplateList trait for the struct.
      #[allow(unused_qualifications)]
      impl $crate::messaging::prelude::TemplateList for $struct_name {
        /// The list of templates for the struct.
        const TEMPLATES: &'static [&'static str] = &[
          $($template),*
        ];
      }

      /// Implement the SimpleTemplate trait for the struct.
      #[allow(unused_qualifications)]
      impl $crate::messaging::traits::data_template::DataTemplate for $struct_name {
        type DataType = [<$struct_name DataType>];
      }
    }
  };
  // Variant without data schema
  ($struct_name:ident, [$($template:expr),* $(,)?]) => {
    /// This struct is used to store the template list.
    #[allow(missing_docs,missing_copy_implementations,missing_debug_implementations,unreachable_pub)]
    pub struct $struct_name;

    /// Implement the TemplateList trait for the struct.
    #[allow(unused_qualifications)]
    impl $crate::messaging::prelude::TemplateList for $struct_name {
      /// The list of templates for the struct.
      const TEMPLATES: &'static [&'static str] = &[
        $($template),*
      ];
    }

    /// Implement the SimpleTemplate trait for the struct.
    #[allow(unused_qualifications)]
    impl $crate::messaging::traits::simple_template::SimpleTemplate for $struct_name {}
  };
}

#[cfg(test)]
mod test {
  #[allow(unused_imports)]
  use super::*;
  use crate::messaging::traits::template_list::TemplateList;
  use crate::test::init as test_init;
  use pretty_assertions::assert_eq;

  template_list!(TestTemplateList, ["Hello, world!", "Goodbye, world!"]);
  template_list!(TestTemplateList2, ["Hello, world!", "Goodbye, world!"], {name: String, age: u8});

  #[test]
  fn test_template_provider() {
    test_init();
    assert_eq!(TestTemplateList::get_template(0).unwrap(), "Hello, world!".to_string());
    assert_eq!(
      TestTemplateList::get_template(1).unwrap(),
      "Goodbye, world!".to_string()
    );
  }

  #[test]
  fn test_template_provider_with_data() {
    test_init();
    let data = TestTemplateList2DataType {
      name: "Alice".to_string(),
      age: 42,
    };
    let data_json = serde_json::to_value(&data).unwrap();
    assert_eq!(data_json, serde_json::json!({"name": "Alice", "age": 42}));
  }
}
