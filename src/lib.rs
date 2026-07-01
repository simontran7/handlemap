pub mod bit_set;
pub mod handle_map;
pub mod list;

#[macro_export]
macro_rules! handle_impl {
    ($vis:vis $name:ident) => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        $vis struct $name(u32);

        impl $crate::handle_map::Handle for $name {
            fn new(i: usize) -> Self { Self(i as u32) }
            fn index(&self) -> usize { self.0 as usize }
        }

        impl $crate::handle_map::ReservedValue for $name {
            fn reserved() -> Self {
                Self(u32::MAX)
            }
            fn is_reserved(&self) -> bool {
                self.0 == u32::MAX
            }
        }
    };
}
