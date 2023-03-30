mod math;

use crate::math::round_to_precision;
use lib::{AgentRunStats, BEN_NAME, CHAD_NAME, DEFAULT_STARTING_CASH, OUTPUT_FILE_NAME};
use ndarray::{Array, Ix1};
use plotters::prelude::*;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

const NETWORTH_PLOT_FILE_NAME: &str = "generated/networth.png";
const TRADE_COUNT_PLOT_FILE_NAME: &str = "generated/trade_count.png";

pub fn main() {
    let data = load_data();
    draw_networth_histogram(&data);
    draw_trade_counts_histogram(&data);
    stdout_stats(&data);
}

fn load_data() -> HashMap<String, Vec<AgentRunStats>> {
    let file = File::open(OUTPUT_FILE_NAME).unwrap();
    let reader = BufReader::new(file);
    return serde_json::from_reader(reader).unwrap();
}

fn draw_networth_histogram(data: &HashMap<String, Vec<AgentRunStats>>) -> () {
    let draw_area = BitMapBackend::new(NETWORTH_PLOT_FILE_NAME, (1280, 960)).into_drawing_area();
    draw_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&draw_area)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .margin(20)
        .caption("Networth Distribution", ("sans-serif", 50.0))
        .build_cartesian_2d(0u32..40000u32, 0u32..800u32)
        .unwrap();

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&BLACK.mix(0.3))
        .y_desc("Count")
        .y_label_style(("sans-serif", 20))
        .x_desc("Networth")
        .x_label_style(("sans-serif", 20))
        .axis_desc_style(("sans-serif", 30))
        .draw()
        .unwrap();

    for (key, value) in data {
        chart
            .draw_series(
                Histogram::vertical(&chart)
                    .margin(12)
                    .style(get_color(key).filled())
                    .data(
                        value
                            .iter()
                            .map(|x| (round_to_precision(x.net_worth as u32, 1000), 1)),
                    ),
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
        .border_style(&BLACK)
        .legend_area_size(30)
        .background_style(&WHITE.mix(0.8))
        .label_font(("sans-serif", 20))
        .draw()
        .unwrap();

    draw_area.present().unwrap();
}

fn draw_trade_counts_histogram(data: &HashMap<String, Vec<AgentRunStats>>) -> () {
    let draw_area = BitMapBackend::new(TRADE_COUNT_PLOT_FILE_NAME, (1280, 960)).into_drawing_area();
    draw_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&draw_area)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .margin(20)
        .caption("Trade Counts Distribution", ("sans-serif", 50.0))
        .build_cartesian_2d(0u32..250u32, 0u32..700u32)
        .unwrap();

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&BLACK.mix(0.3))
        .y_desc("Count")
        .y_label_style(("sans-serif", 20))
        .x_desc("Trade Counts")
        .x_label_style(("sans-serif", 20))
        .axis_desc_style(("sans-serif", 30))
        .draw()
        .unwrap();

    for (key, value) in data {
        chart
            .draw_series(
                Histogram::vertical(&chart)
                    .margin(5)
                    .style(get_color(key).filled())
                    .data(value.iter().map(|x| (x.trade_count as u32, 1))),
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
        .border_style(&BLACK)
        .legend_area_size(30)
        .background_style(&WHITE.mix(0.8))
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
    for (agent, run_stats) in data.iter() {
        let (trade_count_array, net_worth_array) = vec_to_arrays(run_stats);

        let (net_mean, net_std) = calc_stats(&net_worth_array);
        let (trade_count_mean, trade_count_std) = calc_stats(&trade_count_array);
        let delta = (net_mean - DEFAULT_STARTING_CASH) / DEFAULT_STARTING_CASH * 100.0;

        println!(
            "{} statistics:\n\
            Net Worth: {:.2} ± {:.2}\n\
            Trade Count: {:.2} ± {:.2}\n\
            % delta: {:.2}\n",
            agent, net_mean, net_std, trade_count_mean, trade_count_std, delta
        );
    }
}

fn vec_to_arrays(run_stats: &Vec<AgentRunStats>) -> (Array<f64, Ix1>, Array<f64, Ix1>) {
    let trade_counts: Vec<f64> = run_stats.iter().map(|x| x.trade_count as f64).collect();
    let net_worths: Vec<f64> = run_stats.iter().map(|x| x.net_worth).collect();

    let trade_count_array = Array::from(trade_counts);
    let net_worth_array = Array::from(net_worths);

    (trade_count_array, net_worth_array)
}

fn calc_stats(array: &Array<f64, Ix1>) -> (f64, f64) {
    let mean = array.mean().unwrap();
    let std = array.std(0.0);
    (mean, std)
}
