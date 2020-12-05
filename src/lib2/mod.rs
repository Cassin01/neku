extern crate num_traits;
extern crate rand;

use std::collections::HashMap;

/// - 単精度浮動小数点
/// 1bit: 符号 8bit: 指数部 23bit: 仮数部
///
/// - 倍精度浮動小数点
/// 1bit: 符号 11bit: 指数部 52bit: 仮数部 (0以上1未満)

struct P {
    share: HashMap<String, f32>,
    node_number: u32,
}

impl P {
    fn new(node_number: u32) -> Self {
        Self {
            share: HashMap::new(),
            node_number
        }
    }

    fn get_fl(&self, values_name: &String) -> f32 {
        self.share[values_name]
    }

    fn apply_share(&mut self, values_name: String, value: f32) {
        self.share.insert(values_name, value);
    }

    fn remove_share(&mut self, values_name: &String) {
        self.share.remove(values_name);
    }

    fn self_subtraction(&mut self, values_name: String, x: &String, y: &String) {
        self.apply_share(
            values_name,
            self.get_fl(x) - self.get_fl(y),
        );
    }

    fn self_addition(&mut self, values_name: String, x: &String, y: &String) {
        self.apply_share(
            values_name,
            self.get_fl(y) + self.get_fl(x),
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
            rho * self.get_fl(a)
                + sigma * self.get_fl(b)
                + self.get_fl(c) + sigma * rho / self.node_number as f32,
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
            .fold(0., |sum, p| sum + p.get_fl(values_name))
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


    pub fn let_value(&mut self, value_name: String, value: f32) {
        let fl = Self::split_m(value, self.node_number);
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
