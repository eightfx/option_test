use polars::prelude::*;
use std::time::{Duration, Instant};
use std::f64::consts::E;
use probability::prelude::*;
use polars_lazy::prelude::*;
fn _d1(s: f64, k: f64, t: f64, r: f64, sigma: f64) -> f64 {
    ((s / k).ln() + (r + sigma.powi(2) * 0.5 * t)) / (sigma * t.sqrt())
}

fn _d2(d1: f64, sigma: f64, t: f64) -> f64 {
    d1 - sigma * t.sqrt()
}

fn _bsm_call(s: f64, k: f64, d1: f64, d2: f64, r: f64, t: f64) -> f64 {
	let g = Gaussian::new(0.0, 1.0);
    s *g.distribution(d1) - k * E.powf(-r * t) * g.distribution(d2)
}

fn _bsm_put(s: f64, k: f64, d1: f64, d2: f64, r: f64, t: f64) -> f64 {
	let g = Gaussian::new(0.0, 1.0);
    k * E.powf(-r * t) * g.distribution(-d2) - s * g.distribution(-d1)
}

fn bsm_call(s: f64, k: f64, t: f64, r: f64, sigma: f64) -> f64 {
    let d1 = _d1(s, k, t, r, sigma);
    let d2 = _d2(d1, sigma, t);
    _bsm_call(s, k, d1, d2, r, t)
}

fn bsm_put(s: f64, k: f64, t: f64, r: f64, sigma: f64) -> f64 {
    let d1 = _d1(s, k, t, r, sigma);
    let d2 = _d2(d1, sigma, t);
    _bsm_put(s, k, d1, d2, r, t)
}

fn implied_volatility_call(s: f64, k: f64, t: f64, r: f64, price: f64, sigma_est: f64) -> f64 {
    fn difference(s: f64, k: f64, t: f64, r: f64, price: f64, sigma: f64) -> f64 {
        bsm_call(s, k, t, r, sigma) - price
    }

    let mut sigma = sigma_est;
    let mut diff = difference(s, k, t, r, price, sigma);
    let epsilon = 1e-6;
    let max_iter = 100;
    let mut iter = 0;

    while diff.abs() > epsilon && iter < max_iter {
        let d1 = _d1(s, k, t, r, sigma);
		let g = Gaussian::new(0.0, 1.0);
        let vega = s * t.sqrt() * g.distribution(d1);
        sigma = sigma - diff / vega;
        diff = difference(s, k, t, r, price, sigma);
        iter += 1;
    }

    sigma
}

fn main(){
	let df = CsvReader::from_path("data/nk225e2203_20230215_call.csv").unwrap().finish().unwrap();
	let start = Instant::now();
	for i in 0..df.height(){
		let S = df.column("原資産終値").unwrap().f64().unwrap().get(i).unwrap();
		let K = df.column("権利行使価格").unwrap().f64().unwrap().get(i).unwrap();
		let T:f64 = 24./365.;
		let call_price:f64 = df.column("理論価格C").unwrap().f64().unwrap().get(i).unwrap();
		let r:f64 = 0.1492*0.01;
		println!("{:?}",implied_volatility_call(S, K, T, r, call_price, 0.2));
	}
	println!("{:?}", start.elapsed());



	

}
