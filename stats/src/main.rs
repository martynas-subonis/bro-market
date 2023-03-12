mod math;

use crate::math::round_up_to_1000;
use lib::{AgentRunStats, BEN_NAME, CHAD_NAME, OUTPUT_FILE_NAME};
use plotters::coord::ranged1d::SegmentedCoord;
use plotters::coord::types::RangedCoordu32;
use plotters::coord::Shift;
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
        .build_cartesian_2d((1000u32..35000u32).into_segmented(), 0u32..30u32)
        .unwrap();

    chart
        .configure_mesh()
        .disable_x_mesh()
        .bold_line_style(&BLACK.mix(0.3))
        .y_desc("Count")
        .x_desc("Networth")
        .axis_desc_style(("sans-serif", 30))
        .draw()
        .unwrap();

    let color_map = HashMap::from([
        (CHAD_NAME.to_string(), RED.mix(0.3).filled()),
        (BEN_NAME.to_string(), GREEN.mix(0.3).filled()),
    ]);

    for (key, value) in data {
        chart
            .draw_series(
                Histogram::vertical(&chart)
                    .margin(15)
                    .style(*color_map.get(&key).unwrap())
                    .data(
                        value
                            .iter()
                            .map(|x| (round_up_to_1000(x.net_worth as u32), 1)),
                    ),
            )
            .unwrap()
            .label("y = x^2")
            .legend(|(x,y)| PathElement::new(vec![(x,y), (x + 20,y)], &RED));
    }

    draw_area.present().unwrap();
}
