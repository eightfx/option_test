//! Greeks Exposure refers to the sum of the product of each strike's greeks  and open interest in the Option Chain.
//! It represents a measure of the market risk and economic impact of an option position.
//!
//! Greeks Exposure can be calculated using the following formula:
//!
//!     Greeks Exposure = Sum of (Strike * Open Interest * Each Greek * (-1 if Put))
//!
//!
//! # Example
//! A prime example of Greek exposure is also called gamma exposure (GEX), which represents a market maker's gamma risk in their position. By monitoring their Greeks Exposure, market makers can manage the risk associated with their option positions.

use crate::greeks::EuropeanGreeks;
use paste::paste;
use anyhow::{Result, ensure};
use crate::structs::*;
use crate::black_scholes::*;

macro_rules! exposure_trait {
	($($greeks_name:ident),*) => {
		pub trait GreeksExposure  {
			$(
				paste!{
					fn [<$greeks_name _exposure>](&self) -> Result<FloatType>;
				}
			)*
		}
	};
}

macro_rules! exposure_impl{
	($($greeks_name:ident),*) => {
		impl GreeksExposure for OptionChain<OptionTick>{
			$(
				paste!{
					fn [<$greeks_name _exposure>](&self) -> Result<FloatType> {
						let mut sum:FloatType = 0.;
						for data in self.data.iter(){
							let option_tick = data.to_owned();
							let additional_data = option_tick.additional_data.clone();

							ensure!(additional_data.is_some(), "No additional data is set. Set a value in the additional_data field of the OptionTick.");
							let additional_data = additional_data.unwrap();

							ensure!(additional_data.open_interest.is_some(), "No open interest is set. Set a value in the open_interest field of the additional_data.");
							let oi = additional_data.open_interest.unwrap();

							match data.option_value{
								OptionValue::Price(_) =>  {
									match data.option_type{
										OptionType::Put => sum -= oi * option_tick.get_implied_volatility().$greeks_name(),
										OptionType::Call => sum += oi * option_tick.get_implied_volatility().$greeks_name()
									}
								}

								OptionValue::ImpliedVolatility(_) => {
									match data.option_type{
										OptionType::Put => sum -= oi * option_tick.$greeks_name(),
										OptionType::Call => sum += oi * option_tick.$greeks_name()
									}
								}

							}
						}
						Ok(sum)
					
					}
				}

			)*
			}
	};

	
}

exposure_trait!(delta, gamma, theta, rho, vega, epsilon, vanna, charm, vomma, veta, speed, zomma, color, ultima, dual_delta, dual_gamma);
exposure_impl!(delta, gamma, theta, rho, vega, epsilon, vanna, charm, vomma, veta, speed, zomma, color, ultima, dual_delta, dual_gamma);

