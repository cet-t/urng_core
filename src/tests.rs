use crate::Rng;

macro_rules! impl_test_rng {
    ($($bits:expr),+ $(,)?) => {
        $(::pastey::paste! {
            struct [<MyRng $bits>]([<u $bits>]);

            impl Rng for [<MyRng $bits>] {
                type Word = [<u $bits>];

                fn nextu(&mut self) -> Self::Word {
                    self.0 = self.0.wrapping_add(1);
                    self.0
                }
            }

            #[test]
            fn [<test_rng $bits>]() {
                let mut rng = [<MyRng $bits>](0);
                let _: [<u $bits>] = rng.nextu();
                let _: [<f $bits>] = rng.nextf();
                let _: [<i $bits>] = rng.randi(0, 10);
                let _: [<f $bits>] = rng.randf(0.0, 10.0);
            }
        })+
    };
}

impl_test_rng! { 32, 64 }
