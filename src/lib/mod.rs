extern crate num_traits;
extern crate rand;

use num_traits::Float;
use std::collections::HashMap;

/// - 単精度浮動小数点
/// 1bit: 符号 8bit: 指数部 23bit: 仮数部
///
/// - 倍精度浮動小数点
/// 1bit: 符号 11bit: 指数部 52bit: 仮数部 (0以上1未満)

#[derive(Debug, Copy, Clone, Hash)]
pub struct FL {
    v: u32, // 仮数: l-bit
    p: i32, // 指数: k-bit
    z: u32, // a bit witch is set to 1 if u = 0
    s: u32, // a bit witch is set to 1 if u is negative
}

impl FL {
    const L: u32 = 23;
    const K: u32 = 8;

    fn get_fl(&self) -> &Self {
        &self
    }

    pub fn new(v: u32, p: i32, z: u32, s: u32) -> Self {
        Self {
            v, // 仮数: l-bit
            p, // 指数: k-bit
            z, // a bit witch is set to 1 if u = 0
            s, // a bit witch is set to 1 if u is negative
        }
    }

    pub fn decode(v: Self) -> f32 {
        //dbg!(v.z);
        let sign_f = if v.s == 0 { 1. } else { -1. };
        let mantissa_f = v.v as f32;
        let exponent_f = 2.0.powf(v.p as f32);
        sign_f * mantissa_f * exponent_f
    }

    fn distributer(v: f32) -> Vec<f32> {
        // use rand::Rng;
        // let mut rng = rand::thread_rng();
        // let t1 = rng.gen::<f32>();
        let t1 = 5_f32;
        let t2 = v - t1;
        vec![t1, t2]
    }

    pub fn split_m(mut m: f32, node_number: u32) -> Vec<f32> {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        (0..node_number)
            .map(|p| {
                if p + 1 == node_number {
                    m
                } else {
                    let r = rng.gen::<f32>();
                    m -= r;
                    r
                }
            })
            .collect()
    }

    pub fn float2fl(v: f32) -> Self {
        let (mantissa, exponent, sign) = Float::integer_decode(v);
        Self::new(
            mantissa as u32,
            exponent.into(),
            0,
            if sign >= 0 { 0 } else { 1 },
        )
    }

    pub fn encode(v: f32, number: u32) -> Vec<Self> {
        // Self::distributer(v)
        Self::split_m(v, number)
            .into_iter()
            .map(|x| Self::float2fl(x))
            .collect()
    }

    // 小数部分切り捨て: floor([v]/2^m)
    fn trunc(v: u64, m: u32) -> u32 {
        (v as f64 / (u32::pow(2, m) as f64)).floor() as u32
    }

    // 1 if x < y
    fn lt(x: u32, y: u32) -> u32 {
        if x < y {
            1
        } else {
            0
        }
    }

    fn or(z1: u32, z2: u32) -> u32 {
        z1 | z2
    }

    fn xor(z1: u32, z2: u32) -> u32 {
        z1 ^ z2
    }

    fn fl_mul(fl1: Self, fl2: Self) -> Self {
        let mut v = Self::trunc(fl1.v as u64 * fl2.v as u64, Self::L - 1);
        let b = Self::lt(v, 2u32.pow(Self::L));
        v = Self::trunc((2 * b * v + (1 - b) * v) as u64, 1);
        let z = Self::or(fl1.z, fl2.z);
        let s = Self::xor(fl1.s, fl2.s);
        let p = (fl1.p + fl2.p + (Self::L - b) as i32) * (1 - z as i32);
        Self::new(v, p, z, s)
    }
}

struct P {
    share: HashMap<String, FL>,
    node_number: u32,
}

impl P {
    fn new(node_number: u32) -> Self {
        Self {
            share: HashMap::new(),
            node_number
        }
    }

    fn get_fl(&self, values_name: &String) -> FL {
        self.share[values_name]
    }

    fn apply_share(&mut self, values_name: String, value: FL) {
        self.share.insert(values_name, value);
    }

    fn remove_share(&mut self, values_name: &String) {
        self.share.remove(values_name);
    }

    fn self_subtraction(&mut self, values_name: String, x: &String, y: &String) {
        self.apply_share(
            values_name,
            FL::float2fl(FL::decode(self.get_fl(x)) - FL::decode(self.get_fl(y))),
        );
    }

    fn self_addition(&mut self, values_name: String, x: &String, y: &String) {
        self.apply_share(
            values_name,
            FL::float2fl(FL::decode(self.get_fl(y)) + FL::decode(self.get_fl(x))),
        );
    }


    fn compute_z(
        &mut self,
        leted_value: String,
        sigma: f32,
        rho: f32,
        a: &String,
        b: &String,
        c: &String,
    ) {
        self.apply_share(
            leted_value,
            FL::float2fl(
                rho * FL::decode(self.get_fl(a))
                    + sigma * FL::decode(self.get_fl(b))
                    + FL::decode(self.get_fl(c)) + sigma * rho / self.node_number as f32,
            ),
        );
    }
}

pub struct PS {
    node_number: u32,
    ps: Vec<P>,
}

impl PS {
    fn new(node_number: u32) -> Self {
        Self {
            node_number,
            ps: (0..node_number).map(|_| P::new(node_number)).collect(),
        }
    }

    fn self_subtraction(&mut self, leted_value: String, x: &String, y: &String) {
        for p in self.ps.iter_mut() {
            p.self_subtraction(leted_value.clone(), x, y);
        }
    }

    fn self_addition(&mut self, leted_value: String, x: &String, y: &String) {
        for p in self.ps.iter_mut() {
            p.self_addition(leted_value.clone(), x, y);
        }
    }

    pub fn decode(&self, values_name: &String) -> f32 {
        self.ps
            .iter()
            .fold(0., |sum, p| sum + FL::decode(p.get_fl(values_name)))
    }
    fn compute_z(
        &mut self,
        leted_value: String,
        sigma: f32,
        rho: f32,
        a: &String,
        b: &String,
        c: &String,
    ) {
        for p in self.ps.iter_mut() {
            p.compute_z(leted_value.clone(), sigma, rho, a, b, c);
        }
    }

    pub fn let_value(&mut self, value_name: String, value: f32) {
        let fl = FL::encode(value, self.node_number);
        for (i, p) in self.ps.iter_mut().enumerate() {
            p.apply_share(value_name.to_string(), fl[i]);
        }
    }

    fn remove_value(&mut self, value_name: &String) {
        for p in self.ps.iter_mut() {
            p.remove_share(value_name);
        }
    }


    fn add(&mut self, value_name: String, x: &String, y: &String) {
        self.self_addition(value_name, x, y);
    }

    pub fn mul(&mut self, value_name: String, x: &String, y: &String) {
        // rho, sigma を分散
        self.self_subtraction("sigma".to_string(), x,  &"a".to_string());
        self.self_subtraction("rho".to_string(), y, &"b".to_string());

        // rho, sigma を復元
        let sigma = self.decode(&"sigma".to_string());
        let rho = self.decode(&"rho".to_string());

        // z := x * y を算出
        self.compute_z(
            value_name,
            sigma,
            rho,
            &"a".to_string(),
            &"b".to_string(),
            &"c".to_string(),
        );
    }

    pub fn init() -> Self {
        // 事前計算
        let a = 9f32;
        let b = 6f32;
        let c = a * b;

        let mut ps = PS::new(100);

        ps.let_value("a".to_string(), a);
        ps.let_value("b".to_string(), b);
        ps.let_value("c".to_string(), c);
        ps
    }
}

struct Field {}

impl Field {
    fn mul_all(x: f32, y: f32) -> f32 {
        // 事前計算
        let a = 9f32;
        let b = 6f32;
        let c = a * b;

        let mut ps = PS::new(10);

        ps.let_value("a".to_string(), a);
        ps.let_value("b".to_string(), b);
        ps.let_value("c".to_string(), c);

        ps.let_value("x".to_string(), x);
        ps.let_value("y".to_string(), y);

        // rho, sigma を分散
        ps.self_subtraction("sigma".to_string(), &"a".to_string(), &"x".to_string());
        ps.self_subtraction("rho".to_string(), &"b".to_string(), &"y".to_string());

        // rho, sigma を復元
        let sigma = ps.decode(&"sigma".to_string());
        let rho = ps.decode(&"rho".to_string());

        // z := x * y を算出
        ps.compute_z(
            "z".to_string(),
            sigma,
            rho,
            &"a".to_string(),
            &"b".to_string(),
            &"c".to_string(),
        );

        // zを復元
        let z = ps.decode(&"z".to_string()) + sigma * rho;
        z
    }
}
