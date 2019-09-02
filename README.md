## Rusty Backtest

Backtesting made easy!

Rusty Backtest follows a simple concept. The enter - exit testing strategy. 

This is where there is some set of logic returns a boolean (True/False) action to enter and to exit the market. This could be a buy-sell pair or a sell-buy pair.

A verbal example of a exit enter test:
```
enter the market when the RSI is above 70

exit the market if your up 30% or if its been 1 month or if the current price is 50% down.
```

You can chain logical test together on both the entry and exit points.

## Enter/Exit Logic Breakdown

#### Entering market rules

In this case we passed the backtest an array 2, 7. Think of it as a table with 7 rows and 2 columns. 

The first column is the price and is required for the backtest. It is the "market" prices that the backtest uses to track price.

The second column and onwards are custom data values that can be used to infor the enter exit logic.

In this case the second column is some calculated metric - lets say this is the DSI (david's special indicator). It could be the RSI or EMA or other well known technical indicator.

```rust
let mydata = vec![
    vec![1.0, 5.3, 0.5, 5.3, 1.0, 5.3, 1.0],
    vec![0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0],
];
```

Now that we have this information, we want to only enter the market when the DSI is above `0.5`. Looking at the data above - we can see this only happends once (good indicator). Note: here we access the second column the "DSI" with `market_info.data[1]`

```rust
fn enter_market_function(market_info: EnterMarketInfo) -> bool {
    // if second row is above or equal to the limit we care about
    if market_info.data[1][market_info.index as usize] >= 0.5 {
        return true;
    }
    return false;
}
```

This function is run on every "row" of data and allows the user to set the strategy of their choice. If the function returns true *and the portfolio has money* then the backtest will enter the market. If the function returns false, then no action will be taken.

#### Exiting market rules

Now we have a way to enter the market. We need a way to exit it! Here we can set some logic to exit the market. Here we just want to exit if the trade is more then 2 days old. This is a silly strategy but you can chain any boolean statements together to make a much more complex - successful strategy.

```rust
fn exit_market_function(market_info: ExitMarketInfo) -> bool {
    // if trade in for certain time
    if market_info.index - market_info.index_in >= 2 {
        return true;
    }
    return false;
}
```


## Full Example

We pull together the above enter and exit logic and run a full test. We can see that this test is very small - but also runs very fast `57 nano seconds`. 

eg. running this same program on about 1500 daily equity prices ran  `<2ms`

Copy and run the following example using
```bash
cargo run --release
```

```rust
use std::time::Instant;

extern crate rusty_backtest;
use crate::rusty_backtest::backtest;
use crate::rusty_backtest::EnterMarketInfo;
use crate::rusty_backtest::ExitMarketInfo;
use crate::rusty_backtest::TradeInputResults;

fn enter_market_function(market_info: EnterMarketInfo) -> bool {
    // if second row indicates
    if market_info.data[1][market_info.index as usize] >= 0.5 {
        return true;
    }
    return false;
}

fn exit_market_function(market_info: ExitMarketInfo) -> bool {
    // if trade in for certain time
    if market_info.index - market_info.index_in >= 2 {
        return true;
    }
    return false;
}

fn main() {
    let _start = Instant::now();

    let mydata = vec![
        vec![1.0, 5.3, 0.5, 5.3, 1.0, 5.3, 1.0],
        vec![0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0],
    ];

	// backtest arguments
	// 	values
	// 	holding
	// 	default_amt
	// 	enter_market_function
	// 	exit_market_function
    let _returns = backtest(
        mydata,
        100,
        5,
        &enter_market_function,
        &exit_market_function,
    );

    let _out = TradeInputResults { returns: _returns };
    let duration = _start.elapsed();

    println!("{:#?}", _out);
    println!("{:#?}", duration);
}

// CONSOLE OUTPUT

//	 [1.0, 5.3, 0.5, 5.3, 1.0, 5.3, 1.0]
//	 TradeInputResults {
//	     returns: BacktestResults {
//	         calculated_returns: 0.5,
//	         tradesin: [],
//	         tradesout: [
//	             TradeActionOut {
//	                 index_in: 2,
//	                 price_in: 0.5,
//	                 amt: 5,
//	                 index_out: 4,
//	                 price_out: 1.0,
//	                 diff: 0.5,
//	             },
//	         ],
//	     },
//	 }
//	 57.715µs
```