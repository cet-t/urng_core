mod sealed {
    pub trait Sealed: Copy + Sized {}
    impl self::Sealed for u32 {}
    impl self::Sealed for u64 {}
}

/// The unsigned output word of a generator ([`u32`], [`u64`]).
pub trait Word: self::sealed::Sealed {
    type Int: Copy + Sized;
    type Float: Copy + Sized;

    fn to_float(self) -> Self::Float;
    fn randi(self, min: Self::Int, max: Self::Int) -> Self::Int;
    fn randf(self, min: Self::Float, max: Self::Float) -> Self::Float;
}

macro_rules! impl_word {
    ($($bits:tt),+ $(,)?) => {
        $(::pastey::paste! {
            impl self::Word for [<u $bits>] {
                type Float = [<f $bits>];
                type Int = [<i $bits>];

                #[inline(always)]
                fn to_float(self) -> Self::Float {
                    $crate::u2f_01!(Self::Float, $bits, self)
                }

                #[inline(always)]
                fn randi(self, min: Self::Int, max: Self::Int) -> Self::Int {
                    let range = {
                        (max as $crate::randi_wide!(i $bits) - min as $crate::randi_wide!(i $bits) + 1)
                        as $crate::randi_wide!(u $bits)
                    };
                    ((self as $crate::randi_wide!(u $bits) * range) >> $bits) as [<i $bits>] + min
                }

                #[inline(always)]
                fn randf(self, min: Self::Float, max: Self::Float) -> Self::Float {
                    self.to_float() * (max - min) + min
                }
            }
        })+
    };
}

impl_word!(32, 64);
