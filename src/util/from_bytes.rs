use num_traits::Num;


pub trait FromBytes<const N: usize> {
    fn from_be_bytes(bytes: [u8; N]) -> Self;
}

macro_rules! int_trait_impl {
    ($name:ident for $($t:ty, $n:literal)*) => ($(
        impl $name<$n> for $t {
            fn from_be_bytes(bytes: [u8; $n]) -> Self {
                // SAFETY: This is safe because the bytes are in big-endian order
                unsafe { std::mem::transmute_copy(&bytes) }
            }
        }
    )*)
}

int_trait_impl!(FromBytes for u8, 1 u16, 2 u32, 4 u64, 8 usize, 8 i8, 1 i16, 2 i32, 4 i64, 8 isize, 8);


pub trait NumByBytes<const N: usize> : Num + FromBytes<N> {}
impl<T, const N: usize> NumByBytes<N> for T where T : Num + FromBytes<N> {}