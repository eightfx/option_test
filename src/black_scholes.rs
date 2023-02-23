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
use crate::structs::*;

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
	#[allow(non_snake_case)]
	fn Phi(x:&FloatType) ->FloatType;

	/// Returns a new OptionTick instance with the theoretical price calculated from the implied volatility using the Black-Scholes formula.
	/// # Formula
	/// $$
	/// 	C(S_t,K,\tau,r,q,\sigma) = S_t e^{-q\tau} \Phi(d_1) - K e^{-r\tau} \Phi(d_2)
	/// $$
	/// $$
	/// 	P(S_t,K,\tau,r,q,\sigma) = K e^{-r\tau} \Phi(-d_2) - S_t e^{-q\tau} \Phi(-d_1)
	/// $$
	///
	/// # Process flow
	///
	/// 1. Check if the option value is an implied volatility. If not, return a clone of self.
	/// 2. Calculate d1 and d2 using the implied volatility and other parameters of self.
	/// 3. Calculate the price using the Black-Scholes formula for call or put depending on the option type of self.
	/// 4. Clone self and set the option value to Price(price).
	/// 5. Return the modified option instance.
	///
	/// # Notes
	///
	/// * This function assumes that self has valid values for symbol, strike, expiry, option_type and implied_volatility fields.
	/// * This function uses Self::Phi to calculate the cumulative distribution function of a standard normal distribution.
	fn get_theoretical_price(&self) -> Self;

	/// Returns a new OptionTick instance with the implied volatility calculated from the option price using Newton's method.
	///
	/// # Arguments
	///
	/// * `sigma_est`: A float value that represents the initial estimate of the implied volatility.
	/// * `epsilon`: A float value that represents the tolerance for the convergence criterion.
	///
	/// # Process flow
	///
	/// 1. Check if the option value is a price. If not, return a clone of self.
	/// 2. Clone self and assign sigma_est to sigma.
	/// 3. Calculate the difference between the option price and the Black-Scholes formula using sigma as the implied volatility.
	/// 4. Check if the option type is a call or a put.
	/// 5. If it is a call, use Newton's method to iteratively update sigma until the difference is less than epsilon or a maximum number of iterations is reached.
	/// 6. If it is a put, implement a similar logic as for call (TODO).
	/// 7. Assign sigma to new_sigma and set the option value to ImpliedVolatility(new_sigma).
	/// 8. Return the modified option instance.
	///
	/// # Notes
	///
	/// * This function assumes that self has valid values for symbol, strike, expiry and option_type fields.
	/// * This function uses Gaussian::new(0.0, 1.0) to create a standard normal distribution for calculating vega (the derivative of price with respect to volatility).
	fn get_implied_volatility(&self)->Self;
	fn _difference(option:&Self, implied_volatility:FloatType)->FloatType;
	
}

impl BlackScholes for OptionTick{
	fn d1(&self) -> FloatType {
		let tau = self.tau();
		match self.option_value{
			OptionValue::Price(_) => {FloatType::NAN},
			OptionValue::ImpliedVolatility(implied_volatility) =>{
				((self.asset_price / self.strike).log(std::f64::consts::E) + (self.risk_free_rate  - self.dividend_yield + 0.5 * implied_volatility * implied_volatility) * tau) / (implied_volatility * tau.sqrt() )
			}
		}

	}
	fn d2(&self) -> FloatType {
		let tau = self.tau();
		match self.option_value{
			OptionValue::Price(_) => {FloatType::NAN},
			OptionValue::ImpliedVolatility(implied_volatility) =>{
				((self.asset_price / self.strike).log(std::f64::consts::E) + (self.risk_free_rate - self.dividend_yield  - 0.5 * implied_volatility * implied_volatility) * tau) / (implied_volatility * tau.sqrt())
			}
		}

	}
	
	fn phi(x:&FloatType) ->FloatType {
		(-0.5*x*x).exp() / (2.0 * std::f64::consts::PI).sqrt()
	}

	fn Phi(x:&FloatType) ->FloatType {
		let g = Gaussian::new(0.0, 1.0);
		g.distribution(*x)
	}

	fn get_theoretical_price(&self) -> Self{
		match self.option_value{
			OptionValue::Price(_) =>{self.clone()},
			OptionValue::ImpliedVolatility(_) =>{
				let d1 = self.d1();
				let d2 = self.d2();
				let tau = self.tau();

				let price = match self.option_type {
					OptionType::Call => (-self.dividend_yield * tau).exp() * self.asset_price * Self::Phi(&d1) - self.strike * (-self.risk_free_rate * tau).exp() * Self::Phi(&d2),
					OptionType::Put => self.strike * (-self.risk_free_rate * tau).exp() * Self::Phi(&(-d2)) - (-self.dividend_yield * tau).exp() * self.asset_price * Self::Phi(&(-d1)),
				};
				let mut new_option = self.clone();
				new_option.option_value = OptionValue::Price(price);
				new_option
			}
		}
	}

	fn get_implied_volatility(&self) -> Self{
		let sigma_est = 10.;
		let epsilon = 0.0001;
		let tau = self.tau();

		match self.option_value{

			OptionValue::Price(_) =>{
				let mut option = self.clone();
				let mut sigma = sigma_est;
				let mut diff = Self::_difference(&option,sigma);
				let max_iter = 5000;
				let mut iter = 0;

					
				while diff.abs() > epsilon && iter < max_iter {

					let mut option_with_iv = option.clone();
					option_with_iv.option_value = OptionValue::ImpliedVolatility(sigma);
					let d1 = option_with_iv.d1();
					let g = Gaussian::new(0.0, 1.0);
					let vega = self.asset_price * tau.sqrt() * g.distribution(d1);
					sigma = sigma - diff / vega;
					diff = Self::_difference(&option, sigma.clone());
					iter += 1;
				}
				let new_sigma = sigma;
			option.option_value = OptionValue::ImpliedVolatility(new_sigma);
				option
			}
			OptionValue::ImpliedVolatility(_) =>{
				self.clone()
			}
		}
	}

	fn _difference(option:&Self, implied_volatlity:FloatType) -> FloatType{
		// Change only IV
		let mut option_ = option.clone();
		option_.option_value = OptionValue::ImpliedVolatility(implied_volatlity);

		// Theoretical price calculated from iv - Current premium
		option_.get_theoretical_price().get_value() - option.get_value()
	}

	
}

