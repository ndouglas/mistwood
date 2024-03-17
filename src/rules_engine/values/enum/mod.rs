use crate::prelude::EnumListValue;
use crate::prelude::EnumMapValue;
use crate::prelude::EnumValue;

value_list_impl!(EnumListValue, EnumValue, "EnumList", String);
value_map_impl!(EnumMapValue, EnumValue, "EnumMap", String);
