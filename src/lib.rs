fn first<T>(v: &Vec<T>) -> Option<&T> {
    v.first()
}

#[derive(Debug, Clone)]
pub struct TradeActionOut {
    pub index_in: i32,
    pub price_in: f64,
    pub amt: i32,
    pub index_out: i32,
    pub price_out: f64,
    pub diff: f64,
}

#[derive(Debug)]
pub struct TradeActionIn {
    pub index_in: i32,
    pub price_in: f64,
    pub amt: i32,
}

#[derive(Debug)]
pub struct TradeInputResults {
    pub returns: BacktestResults,
}

#[derive(Debug)]
pub struct BacktestResults {
    pub calculated_returns: f64,
    pub tradesin: Vec<TradeActionIn>,
    pub tradesout: Vec<TradeActionOut>,
}

#[derive(Debug)]
pub struct EnterMarketInfo<'a> {
    pub index: i32,
    pub current_price: f64,
    pub holding: f64,
    pub inmarket: f64,
    pub data: &'a Vec<Vec<f64>>,
}

#[derive(Debug)]
pub struct ExitMarketInfo<'a> {
    pub index: i32,
    pub current_price: f64,
    pub index_in: i32,
    pub price_in: f64,
    pub holding: f64,
    pub inmarket: f64,
    pub data: &'a Vec<Vec<f64>>,
    pub diff: f64,
}

#[derive(Debug)]
pub struct Portfolio {
    pub holding: i32,
    pub inmarket: i32,
}

pub fn backtest(
    values: Vec<Vec<f64>>,
    holding: i32,
    default_amt: i32,
    enter_market_function: &dyn Fn(EnterMarketInfo) -> bool,
    exit_market_function: &dyn Fn(ExitMarketInfo) -> bool,
) -> BacktestResults {
    let mut calculated_returns: f64 = 0.0;

    let close_prices: Vec<f64> = first(&values).unwrap().to_vec();

    let mut portfolio = Portfolio {
        holding: holding,
        inmarket: 0,
    };

    let mut tradesin: Vec<TradeActionIn> = vec![];
    let mut tradesout: Vec<TradeActionOut> = vec![];

    println!("{:?}", close_prices);
    {
        for n in 0..close_prices.len() as i32 {
            let amt: i32 = default_amt;
            let have_money = portfolio.holding > amt;

            let to_make_enter_decision_on = EnterMarketInfo {
                index: n,
                current_price: close_prices[n as usize],
                holding: portfolio.holding.clone() as f64,
                inmarket: portfolio.inmarket.clone() as f64,
                data: &values,
            };

            if have_money && enter_market_function(to_make_enter_decision_on) {
                // WE ARE GOING TO ENTER THE MARKET AT THE CURRENT PRICE
                // AND WE WILL BUY OUR DEFAULT AMOUNT
                let action = TradeActionIn {
                    index_in: n,
                    price_in: close_prices[n as usize],
                    amt: amt,
                };
                tradesin.push(action);
                portfolio.holding = portfolio.holding - amt;
                portfolio.inmarket = portfolio.inmarket + amt;
            }

            tradesin.retain(|item| {
                let difference = close_prices[n as usize] - item.price_in;

                let to_make_exit_decision_on = ExitMarketInfo {
                    index: n,
                    index_in: item.index_in,
                    price_in: item.price_in,
                    current_price: close_prices[n as usize],
                    holding: portfolio.holding.clone() as f64,
                    inmarket: portfolio.inmarket.clone() as f64,
                    diff: difference,
                    data: &values,
                };

                if exit_market_function(to_make_exit_decision_on) {
                    let action_out = TradeActionOut {
                        index_in: item.index_in,
                        price_in: item.price_in,
                        amt: item.amt,
                        index_out: n,
                        price_out: close_prices[n as usize],
                        diff: difference,
                    };
                    tradesout.push(action_out);
                    portfolio.holding = portfolio.holding + amt;
                    portfolio.inmarket = portfolio.inmarket - amt;
                    return false;
                }
                return true;
            });
        }

        // add up all the diffs for a scoring metric
        for trade in tradesout.clone() {
            calculated_returns += trade.diff;
        }
    }

    BacktestResults {
        calculated_returns: calculated_returns,
        tradesin: tradesin,
        tradesout: tradesout,
    }
}
