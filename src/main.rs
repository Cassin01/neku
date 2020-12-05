use std::time::Instant;

macro_rules! measure {
    ( $x:expr) => {{
        let start = Instant::now();
        let result = $x;
        let end = start.elapsed();
        println!(
            "計測開始から{}.{:03}秒経過しました。",
            end.as_secs(),
            end.subsec_nanos() / 1_000_000
        );
        result
    }};
}

mod lib2;

fn main() {
    // measure!({
    //     let mut x = 0.001;
    //     let a = 2.;
    //     for _ in 0..10 {
    //         x = Field::mul_all(2.0, x) - Field::mul_all(Field::mul_all(x, x), a);
    //         dbg!(x);
    //     }
    // });

    measure!({
        let mut ps = lib2::PS::init();
        let t = 2.;
        let x = 0.001;
        for _ in 0..10 {
             ps.let_value("x".to_string(), x);
             ps.let_value("t".to_string(), t);
             ps.mul("z".to_string(), &"x".to_string(), &"t".to_string());

        }
    })
}

#[test]
fn test_split_m() {
    assert_eq!(lib2::PS::split_m(8_f32, 3).iter().sum::<f32>(), 8.0);
}

#[test]
fn test_self_computation() {
    let mut ps = lib2::PS::init();
    ps.let_value("x".to_string(), 2.);
    ps.let_value("x".to_string(), 2.);
    ps.mul("x".to_string(), &"x".to_string(), &"x".to_string());

    dbg!(4. - ps.decode(&"x".to_string()));
}

