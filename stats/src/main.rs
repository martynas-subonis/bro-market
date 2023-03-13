mod math;

use crate::math::round_to_precision;
use lib::{AgentRunStats, BEN_NAME, CHAD_NAME, OUTPUT_FILE_NAME};
use plotters::prelude::*;
use serde_json;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

const OUTPUT_PLOT_FILE_NAME: &str = "generated/stats.png";

pub fn main() {
    draw_histogram(load_data());
}

fn load_data() -> HashMap<String, Vec<AgentRunStats>> {
    let file = File::open(OUTPUT_FILE_NAME).unwrap();
    let reader = BufReader::new(file);
    return serde_json::from_reader(reader).unwrap();
}

fn draw_histogram(data: HashMap<String, Vec<AgentRunStats>>) -> () {
    let draw_area = BitMapBackend::new(OUTPUT_PLOT_FILE_NAME, (1280, 960)).into_drawing_area();
    draw_area.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&draw_area)
        .x_label_area_size(50)
        .y_label_area_size(50)
        .margin(5)
        .caption("Networth Distribution", ("sans-serif", 50.0))
        .build_cartesian_2d((0u32..75000u32).into_segmented(), 0u32..170u32)
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
        let key_cl = key.clone();
        let key_cl2 = key.clone();
        chart
            .draw_series(
                Histogram::vertical(&chart).margin(3).style(get_color(key).filled()).data(
                    value
                        .iter()
                        .map(|x| (round_to_precision(x.net_worth as u32, 500), 1)),
                ),
            )
            .unwrap()
            .label(key_cl)
            .legend(move |(x, y)| {
                PathElement::new(vec![(x, y), (x + 20, y)], ShapeStyle {
                    color: get_color(key_cl2.clone()),
                    filled: false,
                    stroke_width: 4,
                })
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

fn get_color(key: String) -> RGBAColor {
    if CHAD_NAME.to_string().eq(&key) {
        return RED.mix(0.3);
    } else if BEN_NAME.to_string().eq(&key) {
        return GREEN.mix(0.3);
    }
    panic!("Unknown color.")
}
