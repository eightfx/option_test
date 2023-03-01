use super::structs::{
    FloatType, OptionBase, OptionBoard, OptionChain, OptionSide, OptionTick, OptionType,
    OptionValue, StrikeBoard,
};
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};

/// Trait to retrieve common information
/// For example, since OptionChain is a set of OptionTicks with the same maturity, this trait can be used to retrieve maturity information.
/// If it tries to retrieve information that is not common information, it returns None.
pub trait ExtractCommonInfo {
    fn strike(&self) -> Result<FloatType> {
        Err(anyhow!("This function is not available for this struct. Because the value you are calling is not a common value."))
    }
    fn maturity(&self) -> Result<DateTime<Utc>> {
        Err(anyhow!("This function is not available for this struct. Because the value you are calling is not a common value."))
    }
    fn asset_price(&self) -> Result<FloatType> {
        Err(anyhow!("This function is not available for this struct. Because the value you are calling is not a common value."))
    }
    fn risk_free_rate(&self) -> Result<FloatType> {
        Err(anyhow!("This function is not available for this struct. Because the value you are calling is not a common value."))
    }
    fn dividend_yield(&self) -> Result<FloatType> {
        Err(anyhow!("This function is not available for this struct. Because the value you are calling is not a common value."))
    }
    fn option_type(&self) -> Result<OptionType> {
        Err(anyhow!("This function is not available for this struct. Because the value you are calling is not a common value."))
    }
    fn option_value(&self) -> Result<OptionValue> {
        Err(anyhow!("This function is not available for this struct. Because the value you are calling is not a common value."))
    }
    fn side(&self) -> Result<OptionSide> {
        Err(anyhow!("This function is not available for this struct. Because the value you are calling is not a common value."))
    }
}
impl ExtractCommonInfo for OptionTick {
    fn strike(&self) -> Result<FloatType> {
        Ok(self.strike)
    }
    fn maturity(&self) -> Result<DateTime<Utc>> {
        Ok(self.maturity)
    }
    fn asset_price(&self) -> Result<FloatType> {
        Ok(self.asset_price)
    }
    fn risk_free_rate(&self) -> Result<FloatType> {
        Ok(self.risk_free_rate)
    }
    fn dividend_yield(&self) -> Result<FloatType> {
        Ok(self.dividend_yield)
    }
    fn option_type(&self) -> Result<OptionType> {
        Ok(self.option_type.clone())
    }
    fn option_value(&self) -> Result<OptionValue> {
        Ok(self.option_value.clone())
    }
}

impl ExtractCommonInfo for StrikeBoard {
    fn strike(&self) -> Result<FloatType> {
        Ok(self.0[0].strike().unwrap())
    }
    fn maturity(&self) -> Result<DateTime<Utc>> {
        Ok(self.0[0].maturity().unwrap())
    }
    fn asset_price(&self) -> Result<FloatType> {
        Ok(self.0[0].asset_price().unwrap())
    }
    fn risk_free_rate(&self) -> Result<FloatType> {
        Ok(self.0[0].risk_free_rate().unwrap())
    }
    fn dividend_yield(&self) -> Result<FloatType> {
        Ok(self.0[0].dividend_yield().unwrap())
    }
    fn option_type(&self) -> Result<OptionType> {
        Ok(self.0[0].option_type().unwrap())
    }
}

impl<T: OptionBase + ExtractCommonInfo> ExtractCommonInfo for OptionChain<T> {
    fn asset_price(&self) -> Result<FloatType> {
        Ok(self.0[0].asset_price().unwrap())
    }

    fn maturity(&self) -> Result<DateTime<Utc>> {
        Ok(self.0[0].maturity().unwrap())
    }
    fn risk_free_rate(&self) -> Result<FloatType> {
        Ok(self.0[0].risk_free_rate().unwrap())
    }
    fn dividend_yield(&self) -> Result<FloatType> {
        Ok(self.0[0].dividend_yield().unwrap())
    }
}
impl<T: OptionBase + ExtractCommonInfo> ExtractCommonInfo for OptionBoard<T> {
    fn maturity(&self) -> Result<DateTime<Utc>> {
        Ok(self.0[0].maturity().unwrap())
    }
    fn risk_free_rate(&self) -> Result<FloatType> {
        Ok(self.0[0].risk_free_rate().unwrap())
    }
    fn dividend_yield(&self) -> Result<FloatType> {
        Ok(self.0[0].dividend_yield().unwrap())
    }
}
