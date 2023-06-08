use prettytable::{Cell, Row, Table};

use crate::request::Exchanges;

pub fn table_exchanges(data: Exchanges) {
    let mut table = Table::new();
    table.add_row(Row::new(vec![Cell::new("Currency"), Cell::new("Rate")]));

    table.add_row(Row::new(vec![
        Cell::new(&data.base),
        Cell::new(&data.amount.to_string()),
    ]));

    for (currency, rate) in &data.rates {
        table.add_row(Row::new(vec![
            Cell::new(currency),
            Cell::new(&format!("{:.2}", rate)),
        ]));
    }

    table.printstd();
}

fn convert_currency(amount: f64, exchange_rate: f64) -> String {
    format!("{:.2}", amount * exchange_rate)
}

pub fn table_convertion(data: Exchanges, amount: f64) {
    let mut table = Table::new();
    table.add_row(Row::new(vec![
        Cell::new("Currency"),
        Cell::new("Rate"),
        Cell::new("Conversion"),
    ]));

    table.add_row(Row::new(vec![
        Cell::new(&data.base),
        Cell::new(&data.amount.to_string()),
        Cell::new(&amount.to_string()),
    ]));

    for (currency, rate) in &data.rates {
        table.add_row(Row::new(vec![
            Cell::new(currency),
            Cell::new(&format!("{:.2}", rate)),
            Cell::new(&convert_currency(amount, *rate)),
        ]));
    }

    table.printstd();
}
