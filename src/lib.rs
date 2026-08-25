mod handle;
mod handle_map;
mod side_handle_map;

pub use handle::Handle;
pub use handle_map::{HandleMap, IntoIter, Iter, IterMut};
pub use side_handle_map::SideHandleMap;

#[macro_export]
macro_rules! handle_impl {
    ($vis:vis $name:ident) => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        $vis struct $name(core::num::NonZeroU32);

        impl $crate::Handle for $name {
            fn new(i: usize) -> Self {
                Self(core::num::NonZeroU32::new(i as u32 + 1).expect("index too large"))
            }
            fn index(&self) -> usize {
                (self.0.get() - 1) as usize
            }
        }
    };
}
