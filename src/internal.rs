#[macro_export(local_inner_macros)]
macro_rules! randi_wide {
    (i 32) => {
        i64
    };
    (i 64) => {
        i128
    };
    (u 32) => {
        u64
    };
    (u 64) => {
        u128
    };
}

#[macro_export(local_inner_macros)]
macro_rules! i2f_bits {
    (32 bits) => {
        0x3F800000
    };
    (32 bias) => {
        9
    };
    (64 bits) => {
        0x3FF0000000000000
    };
    (64 bias) => {
        11
    };
}

#[macro_export(local_inner_macros)]
macro_rules! u2f_01 {
    ($ft:ty, $bits:tt, $x:expr) => {{
        <$ft>::from_bits(($x >> i2f_bits!($bits bias)) | i2f_bits!($bits bits)) - 1.0
    }};
}
