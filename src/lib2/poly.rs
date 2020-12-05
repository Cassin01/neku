extern crate rand;

struct Polynomial {
    coefficients: Vec<u8>,
    n: u32,
}

impl Polynomial {
    fn new(k: u32, n: u32, seacret: u8) -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        Self {
            coefficients: (0..n).map(|p| rng.gen::<u8>() % 2).collect(),
            n,
        }
    }

    // TODO: FFTにする
    // http://myumori.hatenablog.com/entry/2018_advent_calendar_1
    fn mul(f_coefs, g_coefs) -> Vec<u8> {
        f_size = f_coefs.len();
        g_size = g_coefs.len();
        h_coefs = vec![0; f_size + g_size - 1]
        for i in 0..f_size()
            for j in 0..f_size()
                h_coefs[i+j] += f_coefs[i] * g_coefs[j];
        h_coefs
    }

    fn add(coefs_f: Vec<u8>, coefs_g: Vec<u8>) -> Vec<u8> {
        let mut i = -1;
        coefs_f
            .map(|x| {
                i += 1;
                x + coefs_g[i]
            })
            .collect()
    }

    // evaluate the polynomial at the given point
    fn eval(polynomial: Vec<u8>) {
        result = 0;
        for i in 1..polynomial.len() + 1 {
            mul(result)
        }
    }
}

#[test]

