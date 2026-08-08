pub mod bit_set;
pub mod handle_map;

#[macro_export]
macro_rules! handle_impl {
    ($vis:vis $name:ident) => {
        #[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
        $vis struct $name(core::num::NonZeroU32);

        impl $crate::handle_map::Handle for $name {
            fn new(i: usize) -> Self {
                Self(core::num::NonZeroU32::new(i as u32 + 1).expect("index too large"))
            }
            fn index(&self) -> usize {
                (self.0.get() - 1) as usize
            }
        }
    };
}
