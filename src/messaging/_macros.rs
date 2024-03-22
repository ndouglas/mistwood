/// A macro to generate a list of templates for a struct.
#[macro_export]
macro_rules! template_list {
  ($struct_name:ident, [$($template:expr),* $(,)?]) => {
    pub struct $struct_name;
    impl TemplateList for $struct_name {
      const TEMPLATES: &'static [&'static str] = &[
        $($template),*
      ];
    }
  };
}
