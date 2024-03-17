use crate::prelude::EnumArgument;
use anyhow::Error as AnyError;
use std::collections::HashMap;

define_list_argument_trait_and_impl!(EnumListArgument, EnumArgument, "EnumList", String);
define_map_argument_trait_and_impl!(EnumMapArgument, EnumArgument, "EnumMap", String);
