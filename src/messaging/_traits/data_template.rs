/// A trait for templates that require additional data to be supplied.
pub trait DataTemplate {
  /// The type of data required by the template.
  type DataType;
}
