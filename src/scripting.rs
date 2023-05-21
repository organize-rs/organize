use crate::commands::filter::FilterCmd;
use rhai::{CustomType, TypeBuilder};

// impl CustomType for FilterCmd {
//     fn build(mut builder: TypeBuilder<'_, Self>) {
//         #[allow(deprecated)] // The TypeBuilder api is volatile.
//         builder
//             .with_name("FilterCmd")
//             .with_fn("run_filter", Self::run_filter);
//         // .is_iterable()
//         // .with_get_set("x", Self::get_x, Self::set_x);
//     }
// }

pub fn add(x: i64, y: i64) -> i64 {
    x + y
}
