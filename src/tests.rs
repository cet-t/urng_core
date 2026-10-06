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

            #[test]
            fn [<test_randi_bounds $bits>]() {
                use crate::Word;
                let (lo, hi) = ([<i $bits>]::MIN, [<i $bits>]::MAX);
                for w in [0, 1, [<u $bits>]::MAX / 2, [<u $bits>]::MAX / 2 + 1, [<u $bits>]::MAX] {
                    let _ = w.randi(lo, hi);
                    let _ = w.randi(-10, hi);
                    let _ = w.randi(lo, 10);
                }
                assert_eq!((0 as [<u $bits>]).randi(lo, hi), lo);
                assert_eq!([<u $bits>]::MAX.randi(lo, hi), hi);
                assert_eq!([<u $bits>]::MAX.randi(-10, 10), 10);
                assert_eq!((0 as [<u $bits>]).randi(-10, 10), -10);
            }

            #[test]
            fn [<test_randf_half_open $bits>]() {
                use crate::Word;
                let min: [<f $bits>] = 1.0;
                let max = min + [<f $bits>]::EPSILON;
                let v = [<u $bits>]::MAX.randf(min, max);
                assert!(min <= v && v < max);
                let v = [<u $bits>]::MAX.randf(-10.0, 10.0);
                assert!(v < 10.0);
            }
        })+
    };
}

impl_test_rng! { 32, 64 }
