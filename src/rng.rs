use crate::word::Word;

/// A random number generator characterized by a single output [`Word`].
///
/// ## Example
/// ```
/// struct MyRng32(u32);
///
/// impl ::urng_core::Rng for MyRng32 {
///     type Word = u32;
///
///     fn nextu(&mut self) -> Self::Word {
///         self.0 = self.0.wrapping_add(1);
///         self.0
///     }
/// }
/// ```
pub trait Rng {
    type Word: crate::word::Word;

    #[must_use]
    fn nextu(&mut self) -> Self::Word;

    #[must_use]
    #[inline]
    fn nextf(&mut self) -> <Self::Word as crate::word::Word>::Float {
        self.nextu().to_float()
    }

    #[must_use]
    #[inline]
    fn randi(
        &mut self,
        min: <Self::Word as crate::word::Word>::Int,
        max: <Self::Word as crate::word::Word>::Int,
    ) -> <Self::Word as crate::word::Word>::Int {
        self.nextu().randi(min, max)
    }

    #[must_use]
    #[inline]
    fn randf(
        &mut self,
        min: <Self::Word as crate::word::Word>::Float,
        max: <Self::Word as crate::word::Word>::Float,
    ) -> <Self::Word as crate::word::Word>::Float {
        self.nextu().randf(min, max)
    }
}
