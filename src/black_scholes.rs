//! This trait is used to calculate each value of the BLACK SCHOLES model.
//! # How to use
//! ```
//! mod structs;
//! mod greeks;
//! use crate::structs::*;
//! use crate::greeks::Greeks;
//! fn main() {
//! let option = OptionTick::builder().strike(250.).asset_price(100.).risk_free_rate(0.001)
//!                   .implied_volatility(10.).expiry(30./365.).option_type(OptionType::Call).build();
//! dbg!(option.theoretical_price());
//! }
//! ```
//! # Formula
//! See BlackScholes trait page.


use probability::prelude::*;
use crate::structs::{FloatType,OptionTick, OptionType};

#[cfg_attr(doc, katexit::katexit)]
/// This is the trait for calculating European Greeks.
/// ## Variables
/// * S_t: spot price of asset
/// * K: strike price
/// * $\tau$: Remaining time to maturity (normalized in years, e.g. $\tau$ = 2 means 2years)
/// * r: risk free rate
/// * q: dividend yield
/// * $\sigma$: implied volatility
pub trait BlackScholes{
	/// Returns the d1 
	/// # Formula
	/// $$
	/// 	d_{1}={\frac {\ln(S/K)+\left(r-q+{\frac {1}{2}}\sigma ^{2}\right)\tau }{\sigma {\sqrt {\tau }}}}
	/// $$
	fn d1(&self) ->FloatType;
	/// Returns the d1 
	/// # Formula
	/// $$
	/// 	d_{2}={\frac {\ln(S/K)+\left(r-q-{\frac {1}{2}}\sigma ^{2}\right)\tau }{\sigma {\sqrt {\tau }}}}=d_{1}-\sigma {\sqrt {\tau }}
	/// $$
	fn d2(&self) ->FloatType;

	/// Returns the phi 
	/// # Formula
	/// $$
	/// 	\phi(x) = \frac{1}{\sqrt{2\pi}}e^{-\frac{x^2}{2}}
	/// $$
	fn phi(x:&FloatType) ->FloatType;

	/// Returns the phi 
	/// # Formula
	/// $$
	/// 	\Phi(x) =  \frac{1}{\sqrt{2\pi}}\int_{-\infty}^{x}e^{-\frac{t^2}{2}}dt
	/// $$
	fn Phi(x:&FloatType) ->FloatType;

	/// Returns the theoretical price of the option: calc iv -> premium
	/// # Formula
	/// $$
	/// 	C(S_t,K,\tau,r,q,\sigma) = S_t e^{-q\tau} \Phi(d_1) - K e^{-r\tau} \Phi(d_2)
	/// $$
	/// $$
	/// 	P(S_t,K,\tau,r,q,\sigma) = K e^{-r\tau} \Phi(-d_2) - S_t e^{-q\tau} \Phi(-d_1)
	/// $$
	fn theoretical_price(&self) -> FloatType;

	/// Returns the iv of the option: calc premium -> iv
	fn get_implied_volatility(&self, sigma_est:FloatType, epsilon:FloatType)->FloatType;
	fn _difference(&self, implied_volatility:FloatType)->FloatType;
	
}

impl BlackScholes for OptionTick{
	fn d1(&self) -> FloatType {

		((self.asset_price / self.strike).log(std::f64::consts::E) + (self.risk_free_rate  - self.dividend_yield + 0.5 * self.implied_volatility * self.implied_volatility) * self.expiry) / (self.implied_volatility * self.expiry.sqrt()) 

	}
	fn d2(&self) -> FloatType {

		((self.asset_price / self.strike).log(std::f64::consts::E) + (self.risk_free_rate - self.dividend_yield  - 0.5 * self.implied_volatility * self.implied_volatility) * self.expiry) / (self.implied_volatility * self.expiry.sqrt())

	}
	
	fn phi(x:&FloatType) ->FloatType {
		(-0.5*x*x).exp() / (2.0 * std::f64::consts::PI).sqrt()
	}

	fn Phi(x:&FloatType) ->FloatType {
		let g = Gaussian::new(0.0, 1.0);
		g.distribution(*x)
	}

	fn theoretical_price(&self) -> FloatType{
		let d1 = self.d1();
		let d2 = self.d2();

		match self.option_type {
			OptionType::Call => (-self.dividend_yield * self.expiry).exp() * self.asset_price * Self::Phi(&d1) - self.strike * (-self.risk_free_rate * self.expiry).exp() * Self::Phi(&d2),
			OptionType::Put => self.strike * (-self.risk_free_rate * self.expiry).exp() * Self::Phi(&(-d2)) - (-self.dividend_yield * self.expiry).exp() * self.asset_price * Self::Phi(&(-d1)),
		}
		
	}

	fn get_implied_volatility(&self, sigma_est:FloatType, epsilon:FloatType) -> FloatType{
		let mut sigma = sigma_est;
		let mut diff = self._difference(sigma);

		match self.option_type{
			OptionType::Call =>{
				
				let max_iter = 1000;
				let mut iter = 0;
				while diff.abs() > epsilon && iter < max_iter {

					let mut option_with_iv = OptionTick::builder().asset_price(self.asset_price).strike(self.strike).risk_free_rate(self.risk_free_rate).dividend_yield(self.dividend_yield).expiry(self.expiry).implied_volatility(sigma).option_type(self.option_type.clone()).build();
					let d1 = option_with_iv.d1();
					let g = Gaussian::new(0.0, 1.0);
					let vega = self.asset_price * self.expiry.sqrt() * g.distribution(d1);
					sigma = sigma - diff / vega;
					diff = self._difference(sigma.clone());
					iter += 1;
				}
				sigma
			}
			OptionType::Put =>{
				// TODO
				0.
			}
		}
	}

	fn _difference(&self, implied_volatlity:FloatType) -> FloatType{
		// Change only IV
		let mut option_with_iv = OptionTick::builder().asset_price(self.asset_price).strike(self.strike).risk_free_rate(self.risk_free_rate).dividend_yield(self.dividend_yield).expiry(self.expiry).implied_volatility(implied_volatlity).option_type(self.option_type.clone()).build();
		option_with_iv.theoretical_price() - self.premium
	}

	
}

