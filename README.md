
# Table of Contents

1.  [Introduction](#orgeec97b4)
2.  [Features](#org9b327de)
3.  [Usage](#org37ab3c0)
    1.  [OptionTick](#orgd948abe)
    2.  [StrikeBoard](#org6985364)
    3.  [OptionChain](#orgeb26343)
    4.  [OptionBoard](#orgaea53a8)
    5.  [TimeSeries](#org9395335)


<a id="orgeec97b4"></a>

# Introduction

This is a tool for analyzing derivative options, which is open-source software.
It provides various functions such as fetching best bid/ask/mid prices, option chains, and calculating Black Scholes, Greeks, and their exposures.
The tool is implemented in Rust language.


<a id="org9b327de"></a>

# Features

The following are the features of this tool:

-   Fetching strike board's best bid, best ask, mid prices
-   Fetching option chain's out-of-the-money (otm), at-the-money (atm), call, put, 25delta, and 50delta options
-   Calculating iv <-> premium through Black Scholes
-   Calculating 16 types of Greeks
-   Calculating 16 types of Greeks exposure (e.g., gamma exposure)
-   Performing time series data analysis


<a id="org37ab3c0"></a>

# Usage


<a id="orgd948abe"></a>

## OptionTick

OptionTick is a struct that represents option tick data.
It implements the builder pattern, which can be created as shown below:

    use optiors::prelude::*;
    use chrono::prelude::*;
    
    let mut tick = OptionTick::builder().strike(100.0).asset_price(99.).maturity(maturity)
        .option_type(OptionType::Call).option_value(OptionValue::Price(3.)).side(OptionSide::Bid)
        .additional_data(AdditionalOptionData::builder().volume(20.).build())
        .build();

OptionTick can calculate iv and premium through Black Scholes, as well as various Greeks.

    tick = tick.get_implied_volatility();
    
    dbg!(tick.iv());
    dbg!(tick.delta());
    dbg!(tick.dual_delta());
    dbg!(tick.ultima());


<a id="org6985364"></a>

## StrikeBoard

StrikeBoard is a struct that represents bid/ask board information for a specific maturity and strike.

    let mut sb = StrikeBoard::new();
    sb.push(OptionTick::builder().strike(27800.).asset_price(27602.).risk_free_rate(0.0015).option_value(OptionValue::Price(200.)).expiry(0.06575).option_type(OptionType::Call).side(OptionSide::Bid).build());
    sb.push(OptionTick::builder().strike(27800.).asset_price(27602.).risk_free_rate(0.0015).option_value(OptionValue::Price(230.)).expiry(0.06575).option_type(OptionType::Call).side(OptionSide::Bid).build());
    sb.push(OptionTick::builder().strike(27800.).asset_price(27602.).risk_free_rate(0.0015).option_value(OptionValue::Price(250.)).expiry(0.06575).option_type(OptionType::Call).side(OptionSide::Ask).build());
    sb.push(OptionTick::builder().strike(27800.).asset_price(27602.).risk_free_rate(0.0015).option_value(OptionValue::Price(270.)).expiry(0.06575).option_type(OptionType::Call).side(OptionSide::Ask).build());
    
    // The following methods return OptionTick
    dbg!(sb.best_bid());
    dbg!(sb.best_ask());
    dbg!(sb.mid()); // mid price
    dbg!(sb.mid_weighted()); // volume-weighted mid price


<a id="orgeb26343"></a>

## OptionChain

OptionChain<T> represents an option chain for a specific maturity, where T is either OptionTick or StrikeBoard.
You can store StrikeBoard, which includes strike board information, or simplified tick data that does not include the strike board in OptionChain.

    let mut oc:OptionChain<OptionTick> = OptionChain::builder().maturity(chrono::Utc::now()).build();
    oc.data.push(OptionTick::builder().strike(27700.).asset_price(27802.).risk_free_rate(0.0015).option_value(OptionValue::Price(200.)).expiry(0.06575).option_type(OptionType::Call).build());
    oc.data.push(OptionTick::builder().strike(27800.).asset_price(27802.).risk_free_rate(0.0015).option_value(OptionValue::Price(200.)).expiry(0.06575).option_type(OptionType::Call).build());
    oc.data.push(OptionTick::builder().strike(27900.).asset_price(27802.).risk_free_rate(0.0015).option_value(OptionValue::Price(200.)).expiry(0.06575).option_type(OptionType::Call).build());
    oc.data.push(OptionTick::builder().strike(28000.).asset_price(27802.).risk_free_rate(0.0015).option_value(OptionValue::Price(200.)).expiry(0.06575).option_type(OptionType::Call).build());
    
    // The following returns Self
    dbg!(oc.otm()); 
    
    // The following returns T
    dbg!(oc.atm());
    dbg!(oc.call_25delta()); // tick for a call option with a delta of 25%
    dbg!(oc.put_50delta()); // tick for a put option with a delta of 50%
    
    // The following performs the following operations:
    // Calculate the mid price from the StrikeBoard: OptionChain<StrikeBoard> -> OptionChain<OptionTick>
    // Get the ATM tick: OptionChain<OptionTick> -> OptionTick
    // Calculate iv: OptionTick -> OptionTick
    // Calculate vega: OptionTick -> f64
    dbg!(oc.map(StrikeBoard::mid).unwrap().atm().get_implied_volatility().vega())


<a id="orgaea53a8"></a>

## OptionBoard

OptionBoard<T> collects all OptionChains for different maturities, where T is either OptionTick or StrikeBoard.
It can efficiently insert OptionTick.

    let mut ob = OptionBoard::<StrikeBoard>::new();
    let tick = OptionTick::builder().strike(100.0).asset_price(99.).maturity(maturity)
        .option_type(OptionType::Call).option_value(OptionValue::Price(3.)).side(OptionSide::Bid)
        .additional_data(AdditionalOptionData::builder().volume(20.).build())
        .build();
    ob.upsert(tick);
    let tick = OptionTick::builder().strike(200.0).asset_price(99.).maturity(maturity)
        .option_type(OptionType::Call).option_value(OptionValue::Price(3.)).side(OptionSide::Bid)
        .additional_data(AdditionalOptionData::builder().volume(20.).build())
        .build();
    ob.upsert(tick);
    let tick = OptionTick::builder().strike(200.0).asset_price(99.).maturity(maturity)
        .option_type(OptionType::Call).option_value(OptionValue::Price(5.)).side(OptionSide::Ask)
        .additional_data(AdditionalOptionData::builder().volume(20.).build())
        .build();
    ob.upsert(tick);
    let tick = OptionTick::builder().strike(200.0).asset_price(99.).maturity(Utc::now() + chrono::Duration::days(60))
        .option_type(OptionType::Call).option_value(OptionValue::Price(5.)).side(OptionSide::Ask)
        .additional_data(AdditionalOptionData::builder().volume(20.).build())
        .build();
    ob.upsert(tick);
    dbg!(ob);

Doing so yields the following output

    [src/main.rs:70] ob = OptionBoard(
        [
            OptionChain(
                [
                    StrikeBoard(
                        [
                            OptionTick {
                                strike: 100.0,
                                maturity: 2023-03-25T13:04:11.172354Z,
                                asset_price: 99.0,
                                risk_free_rate: 0.001,
                                dividend_yield: 0.0,
                                option_type: Call,
                                option_value: Price(
                                    3.0,
                                ),
                                side: Some(
                                    Bid,
                                ),
                                additional_data: Some(
                                    AdditionalOptionData {
                                        open_interest: None,
                                        volume: Some(
                                            20.0,
                                        ),
                                    },
                                ),
                            },
                        ],
                    ),
                    StrikeBoard(
                        [
                            OptionTick {
                                strike: 200.0,
                                maturity: 2023-03-25T13:04:11.172354Z,
                                asset_price: 99.0,
                                risk_free_rate: 0.001,
                                dividend_yield: 0.0,
                                option_type: Call,
                                option_value: Price(
                                    3.0,
                                ),
                                side: Some(
                                    Bid,
                                ),
                                additional_data: Some(
                                    AdditionalOptionData {
                                        open_interest: None,
                                        volume: Some(
                                            20.0,
                                        ),
                                    },
                                ),
    
    .......

Thus, upsert automatically creates StrikeBoard, OptionChain, etc., overwriting data if it already exists or adding it if it does not.
Thanks to this function, creating an OptionBoard is very easy.
Just upsert all OptionTicks when given option data.


<a id="org9395335"></a>

## TimeSeries

TimeSeries<T> is time series data of T.
T can be anything.
TimeSeries provides a map function, for example, given the function T -> U, which converts data to an index
TimeSeries<T> -> TimeSeries<U>, and so on.

    // 特定の満期、特定のストライクのbid ask板情報の時系列データを作成
    let mut ts:TimeSeries<StrikeBoard> = TimeSeries::new();
    ts.push(sb.clone());
    ts.push(sb.clone());
    ts.push(sb.clone());
    
    // 板の仲値を計算し、そのIVを計算し、グリークスの時系列データを作成
    // 時系列の中身はStrikeBoard -> OptionTick -> FloatTypeに推移する
    let ts1:TimeSeries<FloatType> = ts.map(StrikeBoard::mid).map(OptionTick::get_implied_volatility).map(OptionTick::vega);
    let ts2:TimeSeries<FloatType> = ts.map(StrikeBoard::best_bid).map(OptionTick::get_implied_volatility).map(OptionTick::vanna);

This allows for easy handling of time series data.

