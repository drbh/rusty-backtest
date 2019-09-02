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
