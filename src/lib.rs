/// An implementation of the fast inverse square root function from quake 3
pub fn q_rsqrt(n: f32) -> f32 {
    let three_halfs = 1.5;
    let x2 = n * 0.5;
    let mut y = n;
    let mut i: i32 = unsafe {
        std::mem::transmute(y) // Uses memory transmutation instead of "evil floating point bit hack"
    };
    i = 0x5f3759df - (i >> 1); // what the fuck?
    y = unsafe { std::mem::transmute(i) };
    y = y * (three_halfs - (x2 * y * y)); // 1st iteration
                                          //y = y*(three_halfs - (x2*y*y)); // 2nd iteration, can be removed
    y
}

#[cfg(test)]
mod tests {
    use crate::q_rsqrt;

    #[test]
    fn test_autogen() {
        let mut i = 2;

        while i < 10_000_000 {
            is_within_one_percent(i as f32);
            i = i + 1
        }
        /*(1..=u32::MAX)
        .into_iter()
        .for_each(|n| is_within_one_percent(n as f32))*/
    }

    fn is_within_one_percent(n: f32) {
        let truth: f32 = 1.0 / n.sqrt();
        let q = q_rsqrt(n);

        let diff = (q - truth).abs();

        let percent_diff = diff / truth;

        println!("n: {}, true: {}, Q_rsqrt: {}", n, truth, q);
        assert!(percent_diff <= 0.01)
    }
}
