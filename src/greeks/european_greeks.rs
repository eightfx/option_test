//! This is a trait to calculate greeks for European options.
//! # How to use
//! ```
//! mod structs;
//! mod greeks;
//! use crate::structs::*;
//! use crate::greeks::Greeks;
//! fn main() {
//! dbg!(greeks::greeks::delta(&250.0, &100.0, &(60.*60.*24.*30.), &0., &10., &OptionType::Call, &OptionStyle::European, &0.001,&0.));
//! dbg!(greeks::greeks::gamma(&250.0, &100.0, &(60.*60.*24.*30.), &0., &10., &OptionType::Call, &OptionStyle::European, &0.001,&0.));
//! dbg!(greeks::greeks::vomma(&250.0, &100.0, &(60.*60.*24.*30.), &0., &10., &OptionType::Call, &OptionStyle::European, &0.001,&0.));
//! dbg!(greeks::greeks::speed(&250.0, &100.0, &(60.*60.*24.*30.), &0., &10., &OptionType::Call, &OptionStyle::European, &0.001,&0.));
//! dbg!(greeks::greeks::charm(&250.0, &100.0, &(60.*60.*24.*30.), &0., &10., &OptionType::Call, &OptionStyle::European, &0.001,&0.));
//! dbg!(greeks::greeks::dual_gamma(&250.0, &100.0, &(60.*60.*24.*30.), &0., &10., &OptionType::Call, &OptionStyle::European, &0.001,&0.));
//! dbg!(greeks::greeks::dual_delta(&250.0, &100.0, &(60.*60.*24.*30.), &0., &10., &OptionType::Call, &OptionStyle::European, &0.001,&0.));
//! dbg!(greeks::greeks::ultima(&250.0, &100.0, &(60.*60.*24.*30.), &0., &10., &OptionType::Call, &OptionStyle::European, &0.001,&0.));
//! }
//! ```
//! # Formula
//! See European trait page.


use probability::prelude::*;
use crate::*;

#[cfg_attr(doc, katexit::katexit)]
/// This is the trait for calculating European Greeks.
/// ## Variables
/// * S_t: spot price of asset
/// * K: strike price
/// * $\tau$: Remaining time to maturity (normalized in years, e.g. $\tau$ = 2 means 2years)
/// * r: risk free rate
/// * q: dividend yield
/// * $\sigma$: implied volatility
pub trait EuropeanGreeks{
	/// Returns the d1 
	/// # Formula
	/// $$
	/// 	d_{1}={\frac {\ln(S/K)+\left(r-q+{\frac {1}{2}}\sigma ^{2}\right)\tau }{\sigma {\sqrt {\tau }}}}
	/// $$
	fn d1(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) ->FloatType;
	/// Returns the d1 
	/// # Formula
	/// $$
	/// 	d_{2}={\frac {\ln(S/K)+\left(r-q-{\frac {1}{2}}\sigma ^{2}\right)\tau }{\sigma {\sqrt {\tau }}}}=d_{1}-\sigma {\sqrt {\tau }}
	/// $$
	fn d2(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) ->FloatType;

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
	
	
	/// Returns the delta of the option
	/// # Formula
	/// $$
	/// 	\Delta_c = e^{-q\tau }\Phi(d_1)
	/// $$
	/// $$
	/// 	\Delta_p = e^{-q\tau }\Phi(-d_1)
	/// $$
	fn delta(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType;

	/// Returns the gamma of the option
	/// # Formula
	/// $$
	/// 	\Gamma = \frac{e^{-q\tau }\phi(d_1)}{S_t\sigma\sqrt{\tau}}
	/// $$
	fn gamma(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType;

	/// Returns the theta of the option
	/// # Formula
	/// $$
	/// 	\Theta_c = -\frac{S_t\sigma e^{-q\tau }\phi(d_1)}{2\sqrt{\tau}} - rKe^{-r\tau }\Phi(d_1) + qS_te^{-q\tau }\Phi(d_1)
	/// $$
	/// $$
	/// 	\Theta_p = -\frac{S_t\sigma e^{-q\tau }\phi(d_1)}{2\sqrt{\tau}} + rKe^{-r\tau }\Phi(-d_1) - qS_te^{-q\tau }\Phi(-d_1)
	/// $$
	fn theta(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType;

	/// Returns the rho of the option
	/// # Formula
	/// $$
	/// 	\rho_c = K\tau e^{-r\tau }\Phi(d_2)
	/// $$
	/// $$
	/// 	\rho_p = -K\tau e^{-r\tau }\Phi(-d_2)
	/// $$
	fn rho(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType;

	/// Returns the vega of the option
	/// # Formula
	/// $$
	/// 	\kappa = S_t\sqrt{\tau}e^{-q\tau }\phi(d_1)
	/// $$
	fn vega(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType;

	/// Returns the epsilon of the option
	/// # Formula
	/// $$
	/// 	\epsilon_c = - S_t \tau e^{-q\tau }\Phi(d_2)
	/// $$
	/// $$
	///     \epsilon_p = S_t \tau e^{-q\tau }\Phi(-d_2)
	/// $$
	fn epsilon(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType;

	/// Returns the vanna of the option
	/// # Formula
	/// $$
	///     -e^{-q\tau} \phi(d_1) \frac{d_2}{d\sigma}
	/// $$
	fn vanna(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType;

	/// Returns the charm of the option
	/// # Formula
	/// $$
	///     c : qe^{-q\tau}\Phi(d_1) - e^{-q\tau} \phi(d_1) \frac{2(r-q)\tau - d_2\sigma\sqrt{\tau}}{2\tau\sigma\sqrt{\tau}}
	/// $$
	/// $$
	///     p : -qe^{-q\tau}\Phi(-d_1) - e^{-q\tau} \phi(d_1) \frac{2(r-q)\tau - d_2\sigma\sqrt{\tau}}{2\tau\sigma\sqrt{\tau}}
	/// $$
	fn charm(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType;

	/// Returns the vomma of the option
	/// # Formula
	/// $$
	///     S_te^{-q\tau}\phi(d_1)\sqrt{\tau} \frac{d_1*d_2}{\sigma}
	/// $$
	fn vomma(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType;

	/// Returns the veta of the option
	/// # Formula
	/// $$
	///-Se^{-q\tau }\phi (d_{1}){\sqrt {\tau }}\left[q+{\frac {\left(r-q\right)d_{1}}{\sigma {\sqrt {\tau }}}}-{\frac {1+d_{1}d_{2}}{2\tau }}\right]
	/// $$
	fn veta(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType;

	/// Returns the speed of the option
	/// # Formula
	/// $$
	///   -e^{-q\tau }{\frac {\phi (d_{1})}{S^{2}\sigma {\sqrt {\tau }}}}\left({\frac {d_{1}}{\sigma {\sqrt {\tau }}}}+1\right)=-{\frac {\Gamma }{S}}\left({\frac {d_{1}}{\sigma {\sqrt {\tau }}}}+1\right)
	/// $$
	fn speed(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType;

	/// Returns the zomma of the option
	/// # Formula
	/// $$
	///   e^{-q\tau }{\frac {\phi (d_{1})\left(d_{1}d_{2}-1\right)}{S\sigma ^{2}{\sqrt {\tau }}}}=\Gamma {\frac {d_{1}d_{2}-1}{\sigma }}
	/// $$
	fn zomma(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType;

	/// Returns the color of the option
	/// # Formula
	/// $$
	///    -e^{-q \tau} \frac{\phi(d_1)}{2S\tau \sigma \sqrt{\tau}} \left[2q\tau + 1 + \frac{2(r-q) \tau - d_2 \sigma \sqrt{\tau}}{\sigma \sqrt{\tau}}d_1 \right]
	/// $$
	fn color(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType;

	/// Returns the ultima of the option
	/// # Formula
	/// $$
	///   {\frac {-{\mathcal {V}}}{\sigma ^{2}}}\left[d_{1}d_{2}(1-d_{1}d_{2})+d_{1}^{2}+d_{2}^{2}\right]
	/// $$
	fn ultima(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType;

	/// Returns the dual_delta of the option
	/// # Formula
	/// $$
	/// c:  -e^{-r \tau} \Phi(d_2)   \\\\
	/// p:  e^{-r \tau} \Phi(-d_2)
	/// $$
	fn dual_delta(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType;

	/// Returns the dual_gamma of the option
	/// # Formula
	/// $$
	///  e^{-r \tau} \frac{\phi(d_2)}{K\sigma\sqrt{\tau}} 
	/// $$
	fn dual_gamma(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType;

	fn get_expiry(ts_expiration:&FloatType, ts_now:&FloatType) -> FloatType{
		(ts_expiration - ts_now) / 31536000.
	}
}

impl EuropeanGreeks for greeks::greeks{
	fn d1(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType {

		let t = Self::get_expiry(ts_expiration, ts_now);
		((asset_price / strike).log(std::f64::consts::E) + (risk_free_rate  - dividend_yield + 0.5 * implied_volatility * implied_volatility) * t) / (implied_volatility * t.sqrt()) 

	}
	fn d2(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType {

		let t = Self::get_expiry(ts_expiration, ts_now);
		((asset_price / strike).log(std::f64::consts::E) + (risk_free_rate - dividend_yield  - 0.5 * implied_volatility * implied_volatility) * t) / (implied_volatility * t.sqrt())

	}
	
	fn phi(x:&FloatType) ->FloatType {
		(-0.5*x*x).exp() / (2.0 * std::f64::consts::PI).sqrt()
	}

	fn Phi(x:&FloatType) ->FloatType {
		let g = Gaussian::new(0.0, 1.0);
		g.distribution(*x)
	}
	
	fn delta(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType {
		
		let d1 = <Self as EuropeanGreeks>::d1(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, risk_free_rate,dividend_yield);
		match option_type {
			OptionType::Call => (-dividend_yield * risk_free_rate).exp() * <Self as EuropeanGreeks>::Phi(&d1),
			OptionType::Put => -(-dividend_yield * risk_free_rate).exp() * <Self as EuropeanGreeks>::Phi(&(-d1))
		}
	}

	fn gamma(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType {

		let d1 = <Self as EuropeanGreeks>::d1(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, risk_free_rate,dividend_yield);
		let t = Self::get_expiry(ts_expiration, ts_now);

		(-dividend_yield * risk_free_rate).exp() * <Self as EuropeanGreeks>::phi(&d1) / (asset_price * implied_volatility * t.sqrt())
	}

	fn theta(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType {

		let d1 = <Self as EuropeanGreeks>::d1(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, risk_free_rate,dividend_yield);
		let d2 = <Self as EuropeanGreeks>::d2(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, risk_free_rate,dividend_yield);

		let t = Self::get_expiry(ts_expiration, ts_now);
		match option_type{
			OptionType::Call => -(-dividend_yield * risk_free_rate).exp() * asset_price * <Self as EuropeanGreeks>::phi(&d1) * implied_volatility / (2.0 * t.sqrt()) - risk_free_rate * strike * (-risk_free_rate * t).exp() * <Self as EuropeanGreeks>::Phi(&d2) + dividend_yield * asset_price * (-dividend_yield * t).exp() * <Self as EuropeanGreeks>::Phi(&d1),
			OptionType::Put => -(-dividend_yield * risk_free_rate).exp() * asset_price * <Self as EuropeanGreeks>::phi(&d1) * implied_volatility / (2.0 * t.sqrt()) + risk_free_rate * strike * (-risk_free_rate * t).exp() * <Self as EuropeanGreeks>::Phi(&(-d2)) - dividend_yield * asset_price * (-dividend_yield * t).exp() * <Self as EuropeanGreeks>::Phi(&(-d1))
		}

	}

	fn rho(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType {
		let d2 = <Self as EuropeanGreeks>::d2(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, risk_free_rate,dividend_yield);


		let t = Self::get_expiry(ts_expiration, ts_now);

		match option_type{
			OptionType::Call => t * strike * (-risk_free_rate * t).exp() * <Self as EuropeanGreeks>::Phi(&d2),
			OptionType::Put => -t * strike * (-risk_free_rate * t).exp() * <Self as EuropeanGreeks>::Phi(&(-d2))

		}

	}

	fn vega(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType {
		let d1 = <Self as EuropeanGreeks>::d1(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, risk_free_rate,dividend_yield);


		let t = Self::get_expiry(ts_expiration, ts_now);
		
		(-dividend_yield * risk_free_rate).exp() * asset_price * <Self as EuropeanGreeks>::phi(&d1) * t.sqrt()
	}

	fn veta(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType {
		let d1 = <Self as EuropeanGreeks>::d1(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, risk_free_rate,dividend_yield);
		let d2 = <Self as EuropeanGreeks>::d2(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, risk_free_rate,dividend_yield);

		let t = Self::get_expiry(ts_expiration, ts_now);
		-asset_price * (-dividend_yield * t).exp() * <Self as EuropeanGreeks>::Phi(&d1) * t.sqrt() * (dividend_yield + (risk_free_rate - dividend_yield)*d1/(implied_volatility * t.sqrt()) - (1.+d1*d2) / (2.*t))
	}

	fn vanna(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType {

		let d1 = <Self as EuropeanGreeks>::d1(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, risk_free_rate,dividend_yield);
		let d2 = <Self as EuropeanGreeks>::d2(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, risk_free_rate,dividend_yield);

		-(-dividend_yield * risk_free_rate).exp() * <Self as EuropeanGreeks>::phi(&d1) * d2 / implied_volatility
	}

	fn charm(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType {
		let d1 = <Self as EuropeanGreeks>::d1(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, risk_free_rate,dividend_yield);
		let d2 = <Self as EuropeanGreeks>::d2(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, risk_free_rate,dividend_yield);

		let t = Self::get_expiry(ts_expiration, ts_now);
		match option_type{
			OptionType::Call => dividend_yield * (-dividend_yield * t).exp() * <Self as EuropeanGreeks>::Phi(&d1) - (-dividend_yield * risk_free_rate).exp() * <Self as EuropeanGreeks>::phi(&d1) * (2.*(risk_free_rate - dividend_yield)*t - d2 * implied_volatility * t.sqrt()) / (2. * t * implied_volatility * t.sqrt()),
			OptionType::Put => -dividend_yield * (-dividend_yield * t).exp() * <Self as EuropeanGreeks>::Phi(&(-d1)) - (-dividend_yield * risk_free_rate).exp() * <Self as EuropeanGreeks>::phi(&(-d1)) * (2.*(risk_free_rate - dividend_yield)*t - d2 * implied_volatility * t.sqrt()) / (2. * t * implied_volatility * t.sqrt())
		}
	}

	fn vomma(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType {
		let d1 = <Self as EuropeanGreeks>::d1(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, risk_free_rate,dividend_yield);
		let d2 = <Self as EuropeanGreeks>::d2(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, risk_free_rate,dividend_yield);

		let t = Self::get_expiry(ts_expiration, ts_now);
		(-dividend_yield * risk_free_rate).exp() * asset_price * <Self as EuropeanGreeks>::phi(&d1) * t.sqrt() * d1 * d2 / implied_volatility
	}

	fn speed(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType {
		let d1 = <Self as EuropeanGreeks>::d1(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, risk_free_rate,dividend_yield);

		let t = Self::get_expiry(ts_expiration, ts_now);
		let gamma = <Self as EuropeanGreeks>::gamma(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, risk_free_rate,dividend_yield);

		- gamma / asset_price * (d1 / (implied_volatility * t.sqrt()) + 1.)
	}

	fn zomma(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType {
		let d1 = <Self as EuropeanGreeks>::d1(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, risk_free_rate,dividend_yield);
		let d2 = <Self as EuropeanGreeks>::d2(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, risk_free_rate,dividend_yield);

		let gamma = <Self as EuropeanGreeks>::gamma(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, risk_free_rate,dividend_yield);

		gamma * (d1*d2 - 1.) / implied_volatility
	}

	fn color(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType {

		let d1 = <Self as EuropeanGreeks>::d1(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, risk_free_rate,dividend_yield);
		let d2 = <Self as EuropeanGreeks>::d2(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, risk_free_rate,dividend_yield);

		let t = Self::get_expiry(ts_expiration, ts_now);

		- (-dividend_yield * risk_free_rate).exp() * <Self as EuropeanGreeks>::phi(&d1) / (2. * asset_price * implied_volatility * t * t.sqrt()) * (2. * risk_free_rate * t + 1. + d1 * (2. * (risk_free_rate - dividend_yield) * t - d2 * implied_volatility * t.sqrt()) / (implied_volatility * t.sqrt()))
	}

	fn ultima(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType {
		let d1 = <Self as EuropeanGreeks>::d1(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, risk_free_rate,dividend_yield);
		let d2 = <Self as EuropeanGreeks>::d2(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, risk_free_rate,dividend_yield);

		let vega = <Self as EuropeanGreeks>::vega(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, risk_free_rate,dividend_yield);

		- vega / (implied_volatility * implied_volatility) * (d1*d2*(1. - d1*d2) + d1*d1 + d2*d2)

	}

	fn epsilon(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType {
		let d1 = <Self as EuropeanGreeks>::d1(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, risk_free_rate,dividend_yield);

		let t = Self::get_expiry(ts_expiration, ts_now);

		match option_type {
			OptionType::Call => - asset_price * t * (-dividend_yield * t).exp() * <Self as EuropeanGreeks>::Phi(&d1),
			OptionType::Put => asset_price * t * (-dividend_yield * t).exp() * <Self as EuropeanGreeks>::Phi(&(-d1))
		}

	}

	fn dual_delta(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType {

		let d2 = <Self as EuropeanGreeks>::d2(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, risk_free_rate,dividend_yield);
		let t = Self::get_expiry(ts_expiration, ts_now);

		match option_type {
			OptionType::Call => - (-risk_free_rate* t).exp() * <Self as EuropeanGreeks>::Phi(&d2),
			OptionType::Put => (-risk_free_rate* t).exp() * <Self as EuropeanGreeks>::Phi(&(-d2))
		}

	}

	fn dual_gamma(strike:&FloatType, asset_price:&FloatType, ts_expiration:&FloatType, ts_now:&FloatType, implied_volatility:&FloatType, option_type:&OptionType, risk_free_rate:&FloatType, dividend_yield:&FloatType) -> FloatType {
		
		let d2 = <Self as EuropeanGreeks>::d2(strike, asset_price, ts_expiration, ts_now, implied_volatility, option_type, risk_free_rate,dividend_yield);
		let t = Self::get_expiry(ts_expiration, ts_now);

		(-risk_free_rate * t).exp() * <Self as EuropeanGreeks>::phi(&d2) / (strike * implied_volatility * t.sqrt())

	}
	
}


#[cfg(test)]
mod tests{
	use crate::greeks::*;

	#[test]
	fn greeks_call(){
		
		assert_float_relative_eq!(greeks::delta(&250.0, &100.0, &(60.*60.*24.*30.), &0., &10., &OptionType::Call, &OptionStyle::European, &0.001,&0.), 0.8673, 0.001);
		assert_float_relative_eq!(greeks::gamma(&250.0, &100.0, &(60.*60.*24.*30.), &0., &10., &OptionType::Call, &OptionStyle::European, &0.001,&0.), 0.0007483, 0.00001);
		assert_float_relative_eq!(greeks::theta(&250.0, &100.0, &(60.*60.*24.*30.), &0., &10., &OptionType::Call, &OptionStyle::European, &0.001,&0.), -374.164, 0.001);
		assert_float_relative_eq!(greeks::rho(&250.0, &100.0, &(60.*60.*24.*30.), &0., &10., &OptionType::Call, &OptionStyle::European, &0.001,&0.), 0.818, 0.001);
		assert_float_relative_eq!(greeks::vega(&250.0, &100.0, &(60.*60.*24.*30.), &0., &10., &OptionType::Call, &OptionStyle::European, &0.001,&0.), 6.151, 0.001);
		
	}

	#[test]
	fn greeks_put(){
		assert_float_relative_eq!(greeks::delta(&250.0, &100.0, &(60.*60.*24.*30.), &0., &10., &OptionType::Put, &OptionStyle::European, &0.001,&0.), -0.132666, 0.001);
		assert_float_relative_eq!(greeks::gamma(&250.0, &100.0, &(60.*60.*24.*30.), &0., &10., &OptionType::Put, &OptionStyle::European, &0.001,&0.), 0.0007483, 0.00001);
		assert_float_relative_eq!(greeks::theta(&250.0, &100.0, &(60.*60.*24.*30.), &0., &10., &OptionType::Put, &OptionStyle::European, &0.001,&0.), -373.9, 0.001);
		assert_float_relative_eq!(greeks::rho(&250.0, &100.0, &(60.*60.*24.*30.), &0., &10., &OptionType::Put, &OptionStyle::European, &0.001,&0.),-19.7285, 0.001);
		assert_float_relative_eq!(greeks::vega(&250.0, &100.0, &(60.*60.*24.*30.), &0., &10., &OptionType::Put, &OptionStyle::European, &0.001,&0.), 6.151, 0.001);
		
		
	}
	
} 
