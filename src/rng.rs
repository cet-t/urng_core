use crate::word::Word;

pub trait Rng {
    type Word: crate::word::Word;

    #[must_use]
    fn nextu(&mut self) -> Self::Word;

    #[must_use]
    fn nextf(&mut self) -> <Self::Word as crate::word::Word>::Float {
        self.nextu().to_float()
    }

    #[must_use]
    fn randi(
        &mut self,
        min: <Self::Word as crate::word::Word>::Int,
        max: <Self::Word as crate::word::Word>::Int,
    ) -> <Self::Word as crate::word::Word>::Int {
        self.nextu().randi(min, max)
    }

    #[must_use]
    fn randf(
        &mut self,
        min: <Self::Word as crate::word::Word>::Float,
        max: <Self::Word as crate::word::Word>::Float,
    ) -> <Self::Word as crate::word::Word>::Float {
        self.nextu().randf(min, max)
    }
}
