//! This is a trait to calculate greeks for European options.
//! # How to use
//! ```
//! mod structs;
//! mod greeks;
//! use crate::structs::*;
//! use crate::greeks::Greeks;
//! fn main() {
//! let option = OptionTick::builder().strike(250.).asset_price(100.).risk_free_rate(0.001)
//!                    .implied_volatility(10.).expiry(30./365.).option_type(OptionType::Call).build();
//! assert_float_relative_eq!(option.delta(), 0.8673, 0.001);
//! assert_float_relative_eq!(option.gamma(), 0.0007483, 0.00001);
//! assert_float_relative_eq!(option.theta(), -374.164, 0.001);
//! assert_float_relative_eq!(option.rho(), 0.818, 0.001);
//! assert_float_relative_eq!(option.vega(), 6.151, 0.001);
//! }
//! ```
//! # Formula
//! See EuropeanGreeks trait page.


use probability::prelude::*;
use crate::*;
use crate::black_scholes::*;

#[cfg_attr(doc, katexit::katexit)]
/// This is the trait for calculating European Greeks.
/// ## Variables
/// * S_t: spot price of asset
/// * K: strike price
/// * $\tau$: Remaining time to maturity (normalized in years, e.g. $\tau$ = 2 means 2years)
/// * r: risk free rate
/// * q: dividend yield
/// * $\sigma$: implied volatility
pub trait EuropeanGreeks: black_scholes::BlackScholes{

	
	/// Returns the delta of the option
	/// # Formula
	/// $$
	/// 	\Delta_c = e^{-q\tau }\Phi(d_1)
	/// $$
	/// $$
	/// 	\Delta_p = e^{-q\tau }\Phi(-d_1)
	/// $$
	fn delta(&self) -> FloatType;

	/// Returns the gamma of the option
	/// # Formula
	/// $$
	/// 	\Gamma = \frac{e^{-q\tau }\phi(d_1)}{S_t\sigma\sqrt{\tau}}
	/// $$
	fn gamma(&self) -> FloatType;

	/// Returns the theta of the option
	/// # Formula
	/// $$
	/// 	\Theta_c = -\frac{S_t\sigma e^{-q\tau }\phi(d_1)}{2\sqrt{\tau}} - rKe^{-r\tau }\Phi(d_1) + qS_te^{-q\tau }\Phi(d_1)
	/// $$
	/// $$
	/// 	\Theta_p = -\frac{S_t\sigma e^{-q\tau }\phi(d_1)}{2\sqrt{\tau}} + rKe^{-r\tau }\Phi(-d_1) - qS_te^{-q\tau }\Phi(-d_1)
	/// $$
	fn theta(&self) -> FloatType;

	/// Returns the rho of the option
	/// # Formula
	/// $$
	/// 	\rho_c = K\tau e^{-r\tau }\Phi(d_2)
	/// $$
	/// $$
	/// 	\rho_p = -K\tau e^{-r\tau }\Phi(-d_2)
	/// $$
	fn rho(&self) -> FloatType;

	/// Returns the vega of the option
	/// # Formula
	/// $$
	/// 	\kappa = S_t\sqrt{\tau}e^{-q\tau }\phi(d_1)
	/// $$
	fn vega(&self) -> FloatType;

	/// Returns the epsilon of the option
	/// # Formula
	/// $$
	/// 	\epsilon_c = - S_t \tau e^{-q\tau }\Phi(d_2)
	/// $$
	/// $$
	///     \epsilon_p = S_t \tau e^{-q\tau }\Phi(-d_2)
	/// $$
	fn epsilon(&self) -> FloatType;

	/// Returns the vanna of the option
	/// # Formula
	/// $$
	///     -e^{-q\tau} \phi(d_1) \frac{d_2}{d\sigma}
	/// $$
	fn vanna(&self) -> FloatType;

	/// Returns the charm of the option
	/// # Formula
	/// $$
	///     c : qe^{-q\tau}\Phi(d_1) - e^{-q\tau} \phi(d_1) \frac{2(r-q)\tau - d_2\sigma\sqrt{\tau}}{2\tau\sigma\sqrt{\tau}}
	/// $$
	/// $$
	///     p : -qe^{-q\tau}\Phi(-d_1) - e^{-q\tau} \phi(d_1) \frac{2(r-q)\tau - d_2\sigma\sqrt{\tau}}{2\tau\sigma\sqrt{\tau}}
	/// $$
	fn charm(&self) -> FloatType;

	/// Returns the vomma of the option
	/// # Formula
	/// $$
	///     S_te^{-q\tau}\phi(d_1)\sqrt{\tau} \frac{d_1*d_2}{\sigma}
	/// $$
	fn vomma(&self) -> FloatType;

	/// Returns the veta of the option
	/// # Formula
	/// $$
	///-Se^{-q\tau }\phi (d_{1}){\sqrt {\tau }}\left[q+{\frac {\left(r-q\right)d_{1}}{\sigma {\sqrt {\tau }}}}-{\frac {1+d_{1}d_{2}}{2\tau }}\right]
	/// $$
	fn veta(&self) -> FloatType;

	/// Returns the speed of the option
	/// # Formula
	/// $$
	///   -e^{-q\tau }{\frac {\phi (d_{1})}{S^{2}\sigma {\sqrt {\tau }}}}\left({\frac {d_{1}}{\sigma {\sqrt {\tau }}}}+1\right)=-{\frac {\Gamma }{S}}\left({\frac {d_{1}}{\sigma {\sqrt {\tau }}}}+1\right)
	/// $$
	fn speed(&self) -> FloatType;

	/// Returns the zomma of the option
	/// # Formula
	/// $$
	///   e^{-q\tau }{\frac {\phi (d_{1})\left(d_{1}d_{2}-1\right)}{S\sigma ^{2}{\sqrt {\tau }}}}=\Gamma {\frac {d_{1}d_{2}-1}{\sigma }}
	/// $$
	fn zomma(&self) -> FloatType;

	/// Returns the color of the option
	/// # Formula
	/// $$
	///    -e^{-q \tau} \frac{\phi(d_1)}{2S\tau \sigma \sqrt{\tau}} \left[2q\tau + 1 + \frac{2(r-q) \tau - d_2 \sigma \sqrt{\tau}}{\sigma \sqrt{\tau}}d_1 \right]
	/// $$
	fn color(&self) -> FloatType;

	/// Returns the ultima of the option
	/// # Formula
	/// $$
	///   {\frac {-{\mathcal {V}}}{\sigma ^{2}}}\left[d_{1}d_{2}(1-d_{1}d_{2})+d_{1}^{2}+d_{2}^{2}\right]
	/// $$
	fn ultima(&self) -> FloatType;

	/// Returns the dual_delta of the option
	/// # Formula
	/// $$
	/// c:  -e^{-r \tau} \Phi(d_2)   \\\\
	/// p:  e^{-r \tau} \Phi(-d_2)
	/// $$
	fn dual_delta(&self) -> FloatType;

	/// Returns the dual_gamma of the option
	/// # Formula
	/// $$
	///  e^{-r \tau} \frac{\phi(d_2)}{K\sigma\sqrt{\tau}} 
	/// $$
	fn dual_gamma(&self) -> FloatType;

}

impl EuropeanGreeks for structs::OptionTick{

	fn delta(&self) -> FloatType {
	
		let d1 = self.d1();
		match self.option_type {
			OptionType::Call => (-self.dividend_yield * self.risk_free_rate).exp() * Self::Phi(&d1),
			OptionType::Put => -(-self.dividend_yield * self.risk_free_rate).exp() * Self::Phi(&(-d1))
		}
	}

	fn gamma(&self) -> FloatType {

		let d1 = self.d1();
		let implied_volatility :FloatType;
		match self.option_value{
			OptionValue::Price(_) => {implied_volatility = FloatType::NAN;},
			OptionValue::ImpliedVolatility(iv) =>{implied_volatility = iv;}
		}

		(-self.dividend_yield * self.risk_free_rate).exp() * Self::phi(&d1) / (self.asset_price * implied_volatility * self.expiry.sqrt())
	}

	fn theta(&self) -> FloatType {

		let d1 = self.d1();
		let d2 = self.d2();
		let implied_volatility :FloatType;
		match self.option_value{
			OptionValue::Price(_) => {implied_volatility = FloatType::NAN;},
			OptionValue::ImpliedVolatility(iv) =>{implied_volatility = iv;}
		}


		match self.option_type{
			OptionType::Call => -(-self.dividend_yield * self.risk_free_rate).exp() * self.asset_price * Self::phi(&d1) * implied_volatility / (2.0 * self.expiry.sqrt()) - self.risk_free_rate * self.strike * (-self.risk_free_rate * self.expiry).exp() * Self::Phi(&d2) + self.dividend_yield * self.asset_price * (-self.dividend_yield * self.expiry).exp() * Self::Phi(&d1),
			OptionType::Put => -(-self.dividend_yield * self.risk_free_rate).exp() * self.asset_price * Self::phi(&d1) * implied_volatility / (2.0 * self.expiry.sqrt()) + self.risk_free_rate * self.strike * (-self.risk_free_rate * self.expiry).exp() * Self::Phi(&(-d2)) - self.dividend_yield * self.asset_price * (-self.dividend_yield * self.expiry).exp() * Self::Phi(&(-d1))
		}

	}

	fn rho(&self) -> FloatType {
		let d2 = self.d2();

		match self.option_type{
			OptionType::Call => self.expiry * self.strike * (-self.risk_free_rate * self.expiry).exp() * Self::Phi(&d2),
			OptionType::Put => -self.expiry * self.strike * (-self.risk_free_rate * self.expiry).exp() * Self::Phi(&(-d2))

		}

	}

	fn vega(&self) -> FloatType {
		let d1 = self.d1();


		(-self.dividend_yield * self.risk_free_rate).exp() * self.asset_price * Self::phi(&d1) * self.expiry.sqrt()
	}

	fn veta(&self) -> FloatType {
		let d1 = self.d1();
		let d2 = self.d2();
		let implied_volatility :FloatType;
		match self.option_value{
			OptionValue::Price(_) => {implied_volatility = FloatType::NAN;},
			OptionValue::ImpliedVolatility(iv) =>{implied_volatility = iv;}
		}


		-self.asset_price * (-self.dividend_yield * self.expiry).exp() * Self::Phi(&d1) * self.expiry.sqrt() * (self.dividend_yield + (self.risk_free_rate - self.dividend_yield)*d1/(implied_volatility * self.expiry.sqrt()) - (1.+d1*d2) / (2.*self.expiry))
	}

	fn vanna(&self) -> FloatType {

		let d1 = self.d1();
		let d2 = self.d2();
		let implied_volatility :FloatType;
		match self.option_value{
			OptionValue::Price(_) => {implied_volatility = FloatType::NAN;},
			OptionValue::ImpliedVolatility(iv) =>{implied_volatility = iv;}
		}


		-(-self.dividend_yield * self.risk_free_rate).exp() * Self::phi(&d1) * d2 / implied_volatility
	}

	fn charm(&self) -> FloatType {
		let d1 = self.d1();
		let d2 = self.d2();
		let implied_volatility :FloatType;
		match self.option_value{
			OptionValue::Price(_) => {implied_volatility = FloatType::NAN;},
			OptionValue::ImpliedVolatility(iv) =>{implied_volatility = iv;}
		}


		match self.option_type{
			OptionType::Call => self.dividend_yield * (-self.dividend_yield * self.expiry).exp() * Self::Phi(&d1) - (-self.dividend_yield * self.risk_free_rate).exp() * Self::phi(&d1) * (2.*(self.risk_free_rate - self.dividend_yield)*self.expiry - d2 * implied_volatility * self.expiry.sqrt()) / (2. * self.expiry * implied_volatility * self.expiry.sqrt()),
			OptionType::Put => -self.dividend_yield * (-self.dividend_yield * self.expiry).exp() * Self::Phi(&(-d1)) - (-self.dividend_yield * self.risk_free_rate).exp() * Self::phi(&(-d1)) * (2.*(self.risk_free_rate - self.dividend_yield)*self.expiry - d2 * implied_volatility * self.expiry.sqrt()) / (2. * self.expiry * implied_volatility * self.expiry.sqrt())
		}
	}

	fn vomma(&self) -> FloatType {
		let d1 = self.d1();
		let d2 = self.d2();
		let implied_volatility :FloatType;
		match self.option_value{
			OptionValue::Price(_) => {implied_volatility = FloatType::NAN;},
			OptionValue::ImpliedVolatility(iv) =>{implied_volatility = iv;}
		}


		(-self.dividend_yield * self.risk_free_rate).exp() * self.asset_price * Self::phi(&d1) * self.expiry.sqrt() * d1 * d2 / implied_volatility
	}

	fn speed(&self) -> FloatType {
		let d1 = self.d1();
		let implied_volatility :FloatType;
		match self.option_value{
			OptionValue::Price(_) => {implied_volatility = FloatType::NAN;},
			OptionValue::ImpliedVolatility(iv) =>{implied_volatility = iv;}
		}


		let gamma = self.gamma();

		- gamma / self.asset_price * (d1 / (implied_volatility * self.expiry.sqrt()) + 1.)
	}

	fn zomma(&self) -> FloatType {
		let d1 = self.d1();
		let d2 = self.d2();
		let implied_volatility :FloatType;
		match self.option_value{
			OptionValue::Price(_) => {implied_volatility = FloatType::NAN;},
			OptionValue::ImpliedVolatility(iv) =>{implied_volatility = iv;}
		}


		let gamma = self.gamma();

		gamma * (d1*d2 - 1.) / implied_volatility
	}

	fn color(&self) -> FloatType {

		let d1 = self.d1();
		let d2 = self.d2();
		let implied_volatility :FloatType;
		match self.option_value{
			OptionValue::Price(_) => {implied_volatility = FloatType::NAN;},
			OptionValue::ImpliedVolatility(iv) =>{implied_volatility = iv;}
		}



		- (-self.dividend_yield * self.risk_free_rate).exp() * Self::phi(&d1) / (2. * self.asset_price * implied_volatility * self.expiry * self.expiry.sqrt()) * (2. * self.risk_free_rate * self.expiry + 1. + d1 * (2. * (self.risk_free_rate - self.dividend_yield) * self.expiry - d2 * implied_volatility * self.expiry.sqrt()) / (implied_volatility * self.expiry.sqrt()))
	}

	fn ultima(&self) -> FloatType {
		let d1 = self.d1();
		let d2 = self.d2();
		let implied_volatility :FloatType;
		match self.option_value{
			OptionValue::Price(_) => {implied_volatility = FloatType::NAN;},
			OptionValue::ImpliedVolatility(iv) =>{implied_volatility = iv;}
		}


		let vega = self.vega();

		- vega / (implied_volatility * implied_volatility) * (d1*d2*(1. - d1*d2) + d1*d1 + d2*d2)

	}

	fn epsilon(&self) -> FloatType {
		let d1 = self.d1();


		match self.option_type {
			OptionType::Call => - self.asset_price * self.expiry * (-self.dividend_yield * self.expiry).exp() * Self::Phi(&d1),
			OptionType::Put => self.asset_price * self.expiry * (-self.dividend_yield * self.expiry).exp() * Self::Phi(&(-d1))
		}

	}

	fn dual_delta(&self) -> FloatType {

		let d2 = self.d2();

		match self.option_type {
			OptionType::Call => - (-self.risk_free_rate* self.expiry).exp() * Self::Phi(&d2),
			OptionType::Put => (-self.risk_free_rate* self.expiry).exp() * Self::Phi(&(-d2))
		}

	}

	fn dual_gamma(&self) -> FloatType {
		let implied_volatility :FloatType;
		match self.option_value{
			OptionValue::Price(_) => {implied_volatility = FloatType::NAN;},
			OptionValue::ImpliedVolatility(iv) =>{implied_volatility = iv;}
		}

		
		let d2 = self.d2();

		(-self.risk_free_rate * self.expiry).exp() * Self::phi(&d2) / (self.strike * implied_volatility * self.expiry.sqrt())

	}
	
}


#[cfg(test)]
mod tests{
	use crate::greeks::*;
	use assert_float_eq::*;

	#[test]
	fn greeks_call(){
		let option = OptionTick::builder().strike(250.).asset_price(100.).risk_free_rate(0.001).implied_volatility(10.).expiry(30./365.).option_type(OptionType::Call).build();
		
		assert_float_relative_eq!(option.delta(), 0.8673, 0.001);
		assert_float_relative_eq!(option.gamma(), 0.0007483, 0.00001);
		assert_float_relative_eq!(option.theta(), -374.164, 0.001);
		assert_float_relative_eq!(option.rho(), 0.818, 0.001);
		assert_float_relative_eq!(option.vega(), 6.151, 0.001);
		
	}

	#[test]
	fn greeks_put(){

		let option = OptionTick::builder().strike(250.).asset_price(100.).risk_free_rate(0.001).implied_volatility(10.).expiry(30./365.).option_type(OptionType::Put).build();
		assert_float_relative_eq!(option.delta(), -0.132666, 0.001);
		assert_float_relative_eq!(option.gamma(), 0.0007483, 0.00001);
		assert_float_relative_eq!(option.theta(), -373.9, 0.001);
		assert_float_relative_eq!(option.rho(),-19.7285, 0.001);
		assert_float_relative_eq!(option.vega(), 6.151, 0.001);
		
		
	}
	
} 
