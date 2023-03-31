mod math;

use crate::math::{calc_stats, calculate_networth_probability, round_to_precision};
use lib::{
    AgentRunStats, BEN_NAME, CHAD_NAME, DEFAULT_STARTING_CASH, NETWORTH_PLOT_FILE_NAME,
    OUTPUT_FILE_NAME, TRADE_COUNT_PLOT_FILE_NAME,
};
use ndarray::{Array, Ix1};
use plotters::prelude::*;
use prettytable::{format, row, Cell, Row, Table};

use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

pub fn main() {
    let data = load_data();
    draw_histogram(
        &data,
        NETWORTH_PLOT_FILE_NAME,
        (0, 40000),
        (0, 800),
        "Networth Distribution",
        "Networth",
        "Count",
        12,
        |x| round_to_precision(x.net_worth as u32, 1000),
    );
    draw_histogram(
        &data,
        TRADE_COUNT_PLOT_FILE_NAME,
        (0, 250),
        (0, 700),
        "Trade Counts Distribution",
        "Trade Counts",
        "Count",
        5,
        |x| x.trade_count as u32,
    );
    stdout_stats(&data);
}

fn load_data() -> HashMap<String, Vec<AgentRunStats>> {
    let file = File::open(OUTPUT_FILE_NAME).unwrap();
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap()
}

#[allow(clippy::too_many_arguments)]
fn draw_histogram<F>(
    data: &HashMap<String, Vec<AgentRunStats>>,
    plot_file_name: &str,
    x_axis_range: (u32, u32),
    y_axis_range: (u32, u32),
    caption: &str,
    x_axis_label: &str,
    y_axis_label: &str,
    bar_width: u32,
    value_mapper: F,
) where
    F: Fn(&AgentRunStats) -> u32,
{
    let draw_area = BitMapBackend::new(plot_file_name, (1280, 960)).into_drawing_area();
    draw_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&draw_area)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .margin(20)
        .caption(caption, ("sans-serif", 50.0))
        .build_cartesian_2d(
            x_axis_range.0..x_axis_range.1,
            y_axis_range.0..y_axis_range.1,
        )
        .unwrap();

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(BLACK.mix(0.3))
        .y_desc(y_axis_label)
        .y_label_style(("sans-serif", 20))
        .x_desc(x_axis_label)
        .x_label_style(("sans-serif", 20))
        .axis_desc_style(("sans-serif", 30))
        .draw()
        .unwrap();

    for (key, value) in data {
        chart
            .draw_series(
                Histogram::vertical(&chart)
                    .margin(bar_width)
                    .style(get_color(key).filled())
                    .data(value.iter().map(|x| (value_mapper(x), 1))),
            )
            .unwrap()
            .label(key)
            .legend(move |(x, y)| {
                PathElement::new(
                    vec![(x, y), (x + 20, y)],
                    ShapeStyle {
                        color: get_color(key),
                        filled: false,
                        stroke_width: 4,
                    },
                )
            });
    }

    chart
        .configure_series_labels()
        .border_style(BLACK)
        .legend_area_size(30)
        .background_style(WHITE.mix(0.8))
        .label_font(("sans-serif", 20))
        .draw()
        .unwrap();

    draw_area.present().unwrap();
}

fn get_color(key: &String) -> RGBAColor {
    if CHAD_NAME.to_string().eq(key) {
        return RED.mix(0.3);
    } else if BEN_NAME.to_string().eq(key) {
        return GREEN.mix(0.3);
    }
    panic!("Unknown color.")
}

fn stdout_stats(data: &HashMap<String, Vec<AgentRunStats>>) {
    let mut table = Table::new();
    table.set_format(*format::consts::FORMAT_DEFAULT);
    let mut header = Row::new(vec![Cell::new("Simulation Statistics")]);
    for agent in data.keys() {
        header.add_cell(Cell::new(agent));
    }
    table.add_row(header);
    let mut net_mean_row = row!["Net avg."];
    let mut trade_count_mean_row = row!["Trade count avg"];
    let mut avg_profit_row = row!["Avg. profit"];
    let mut random_profit_prob_row = row!["Profit p"];
    let mut random_2x_prob_row = row![">2x p"];
    let mut random_3x_prob_row = row![">3x p"];
    let mut random_5x_prob_row = row![">5x p"];

    for run_stats in data.values() {
        let (trade_count_array, net_worth_array) = vec_to_arrays(run_stats);
        let (net_mean, net_std) = calc_stats(&net_worth_array);
        let (trade_count_mean, trade_count_std) = calc_stats(&trade_count_array);

        net_mean_row.add_cell(Cell::new(&format!("{:.2} ± {:.2}", net_mean, net_std)));
        trade_count_mean_row.add_cell(Cell::new(&format!(
            "{:.2} ± {:.2}",
            trade_count_mean, trade_count_std
        )));
        avg_profit_row.add_cell(Cell::new(&format!(
            "{:.2}%",
            (net_mean - DEFAULT_STARTING_CASH) / DEFAULT_STARTING_CASH * 100.0
        )));
        random_profit_prob_row.add_cell(Cell::new(&format!(
            "{:.2}%",
            calculate_networth_probability(&net_worth_array, 1.0)
        )));
        random_2x_prob_row.add_cell(Cell::new(&format!(
            "{:.2}%",
            calculate_networth_probability(&net_worth_array, 2.0)
        )));
        random_3x_prob_row.add_cell(Cell::new(&format!(
            "{:.2}%",
            calculate_networth_probability(&net_worth_array, 3.0)
        )));
        random_5x_prob_row.add_cell(Cell::new(&format!(
            "{:.2}%",
            calculate_networth_probability(&net_worth_array, 5.0)
        )));
    }
    table.add_row(net_mean_row);
    table.add_row(trade_count_mean_row);
    table.add_row(avg_profit_row);
    table.add_row(random_profit_prob_row);
    table.add_row(random_2x_prob_row);
    table.add_row(random_3x_prob_row);
    table.add_row(random_5x_prob_row);
    table.printstd();
}

fn vec_to_arrays(run_stats: &[AgentRunStats]) -> (Array<f64, Ix1>, Array<f64, Ix1>) {
    let trade_counts: Vec<f64> = run_stats.iter().map(|x| x.trade_count as f64).collect();
    let net_worths: Vec<f64> = run_stats.iter().map(|x| x.net_worth).collect();

    let trade_count_array = Array::from(trade_counts);
    let net_worth_array = Array::from(net_worths);

    (trade_count_array, net_worth_array)
}
