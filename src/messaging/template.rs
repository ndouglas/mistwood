/// The Template contains the specific format of the message.
///
/// The `Template` is a struct that contains a format string and a list of
/// arguments. It is used to create a message that can be printed to the screen
/// or logged to a file. The `Template` is used to create a `Message` by
/// combining the format string and the arguments with some system like
/// Mustache or Handlebars.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum Template<'a> {
  /// A static string.
  Static(&'static str),
  /// A borrowed string.
  Borrowed(&'a str),
  /// An owned string.
  Owned(String),
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::test::init as test_init;
  use pretty_assertions::assert_eq;

  #[test]
  fn test_template_static() {
    test_init();
    let template = Template::Static("Hello, world!");
    assert_eq!(template, Template::Static("Hello, world!"));
  }

  #[test]
  fn test_template_borrowed() {
    test_init();
    let template = Template::Borrowed("Hello, world!");
    assert_eq!(template, Template::Borrowed("Hello, world!"));
  }

  #[test]
  fn test_template_owned() {
    test_init();
    let template = Template::Owned("Hello, world!".to_string());
    assert_eq!(template, Template::Owned("Hello, world!".to_string()));
  }
}
