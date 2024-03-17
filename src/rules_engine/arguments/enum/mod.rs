use crate::prelude::EnumValue;
use anyhow::Error as AnyError;
use std::collections::HashMap;

define_list_argument_trait_and_impl!(EnumListValue, EnumValue, "EnumList", String);
define_map_argument_trait_and_impl!(EnumMapValue, EnumValue, "EnumMap", String);
