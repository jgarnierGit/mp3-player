use std::borrow::{Borrow, BorrowMut};
use std::path::Path;

use crate::pixel_buf::PixelBuf;
use audio_player::MetadataParserBuilder;
use minifb::{Key, Window, WindowOptions};
use plotters::backend::{BGRXPixel, PixelFormat};
use plotters::chart::ChartState;
use plotters::coord::types::RangedCoordf64;
use plotters::coord::Shift;
use plotters::drawing::IntoDrawingArea;
use plotters::prelude::*;
use plotters::series::LineSeries;
use plotters::style::{BLUE, RED};

const WIDTH: usize = 1024;
const HEIGHT: usize = 768;

pub fn analyze_samples(
    path: &Path,
    music_name: &str,
    beats: &Vec<f64>,
) -> Option<(Box<Vec<f32>>, Vec<f64>)> {
    let music_path_buf = path.join(music_name);
    let music_path = music_path_buf.as_path();
    let metadata_parser = MetadataParserBuilder::build();
    let frame_rate_tag = String::from("frameRate");
    let channel_tag = String::from("channels");
    let frame_rate = metadata_parser.get_metadata_string(music_path, &frame_rate_tag);
    let channel_count = metadata_parser.get_metadata_string(music_path, &channel_tag);

    if let (Ok(rate), Ok(channel_c)) = (frame_rate, channel_count) {
        println!(
            "audio has framerate of {}, for channels count of {}",
            rate, channel_c
        );

        if let Some(samples) = metadata_parser.get_file_samples(music_path) {
            println!("samples to print {}", samples.len());
            let inverse_sample_rate = 1.0 / rate.parse::<f64>().unwrap();
            let channel_nb = channel_c.parse::<u32>().unwrap();
            println!(
                "samples length {} at frame rate is {}",
                (samples.len() as f64) * inverse_sample_rate / channel_nb as f64,
                inverse_sample_rate
            );
            let beats_formatted: Vec<f64> = beats
                .iter()
                .map(|sec| (*sec / inverse_sample_rate))
                .collect();
            println!(
                "before:{:?},{:?}; after: {:?},{:?}",
                beats.first(),
                beats.last(),
                beats_formatted.first(),
                beats_formatted.last()
            );
            let samples_formatted: Box<Vec<f32>> = Box::new(
                samples
                    .iter()
                    .enumerate()
                    .filter(|(i, v)| *i as f64 % channel_nb as f64 == 0.0)
                    .map(|(i, v)| *v)
                    .collect(),
            );
            println!(
                "sample format : {}, {}",
                samples.len(),
                samples_formatted.len()
            );
            return Some((samples_formatted, beats_formatted));
        }
    }
    None
}

pub fn draw_static_into_window(
    _path: &Path,
    _music_name: &str,
    audio_samples: &Box<Vec<f32>>,
    beats: &Vec<f64>,
) -> Result<(), Box<dyn std::error::Error>> {
    let (mut window, mut pixel_buf, _chart_state) = setup_window();

    println!("done into drawing");
    let root_drawing_area = get_drawing_area(pixel_buf.borrow_mut());
    generate_static_spectrum(&root_drawing_area, audio_samples, beats)?;
    root_drawing_area
        .present()
        .expect("Unable to write result to buffer ??");
    drop(root_drawing_area);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window
            .update_with_buffer(pixel_buf.borrow(), WIDTH, HEIGHT)
            .unwrap();
    }
    Ok(())
}

fn setup_window() -> (
    Window,
    PixelBuf,
    ChartState<Cartesian2d<RangedCoordf64, RangedCoordf64>>,
) {
    let mut pixel_buf = PixelBuf(vec![0_u32; WIDTH * HEIGHT]);
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let x_min: f64 = 0.0;
    let x_max: f64 = 1.0;
    let y_min: f64 = 0.0;
    let y_max: f64 = 1.0;

    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));
    let root_drawing_area = get_drawing_area(pixel_buf.borrow_mut());
    // renders a first frame in order to get an initial chartState for dynamic rendering reset

    let mut chart = ChartBuilder::on(&root_drawing_area)
        .caption("Bitmap Example", ("sans-serif", 30))
        .margin(10)
        .set_label_area_size(LabelAreaPosition::Left, 40)
        .set_label_area_size(LabelAreaPosition::Bottom, 40)
        .build_cartesian_2d(x_min..x_max, y_min..y_max)
        .expect("Could not build chart");
    chart
        .configure_mesh()
        //.disable_mesh()
        .draw()
        .expect("Could not draw initial chart");
    let chart_state = chart.into_chart_state();
    drop(root_drawing_area);
    (window, pixel_buf, chart_state)
}

fn get_drawing_area(pixel_buf: &mut [u8]) -> DrawingArea<BitMapBackend<BGRXPixel>, Shift> {
    BitMapBackend::<BGRXPixel>::with_buffer_and_format(
        pixel_buf.borrow_mut(),
        (WIDTH as u32, HEIGHT as u32),
    )
    .unwrap()
    .into_drawing_area()
}

pub fn draw_into_png(
    path: &Path,
    music_name: &str,
    audio_samples: &Box<Vec<f32>>,
    beats: &Vec<f64>,
) -> Result<(), Box<dyn std::error::Error>> {
    let img = path.join(music_name.to_owned() + ".png");
    println!("path is {:?}", img);
    let point_diam: u32 = 1;
    let smallest_step = 1.0 / find_smallest_step(beats).unwrap();
    //let last = beats.last().unwrap().round() + 10.0;
    let step_length = (beats.len() as f64) * smallest_step.round();
    let root = BitMapBackend::new(img.to_str().unwrap(), (2048, 768)).into_drawing_area();
    generate_static_spectrum(&root, audio_samples, beats)?;
    // To avoid the IO failure being ignored silently, we manually call the present function
    root.present().expect("Unable to write result to file, please make sure 'plotters-doc-data' dir exists under current dir");
    println!("Result has been saved to {}", img.to_str().unwrap());

    Ok(())
}

fn generate_static_spectrum<T>(
    root: &DrawingArea<BitMapBackend<T>, Shift>,
    audio_samples: &Box<Vec<f32>>,
    beats: &Vec<f64>,
) -> Result<(), String>
where
    T: PixelFormat,
{
    root.fill(&WHITE).expect("Could not reset background color");
    let x_max = audio_samples.len() as f64;
    let x_min = 0.0;
    if let (Some(y_min), Some(y_max)) = (
        audio_samples.iter().map(|v| *v).reduce(f32::min),
        audio_samples.iter().map(|v| *v).reduce(f32::max),
    ) {
        println!("min: {}, max: {}", y_min, y_max);
        let mut chart = ChartBuilder::on(&root)
            .caption("Bitmap Example", ("sans-serif", 30))
            .margin(10)
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .build_cartesian_2d(x_min..x_max, y_min..y_max)
            .expect("Could not build chart");
        let chart_state = chart
            .configure_mesh()
            .disable_mesh()
            .draw()
            .expect("Could not draw initial chart");

        chart
            .draw_series(LineSeries::new(
                audio_samples
                    .iter()
                    .enumerate()
                    .map(|(x, y)| (x as f64, *y)),
                &RED,
            ))
            .expect("could not draw spectrum series");
        //
        chart
            .draw_series(
                beats.iter().map(|t| {
                    Polygon::new([(*t, y_max), (*t, y_min)], BLUE.stroke_width(1).filled())
                }),
            )
            .expect("could not draw Beat series");
        return Ok(chart_state);
    }
    Err(String::from("cannot compute x_min, y_min"))
}

fn find_smallest_step<T>(samples: &[T]) -> Option<T>
where
    T: core::cmp::PartialOrd<T> + std::ops::Sub<Output = T> + std::fmt::Display + std::marker::Copy,
{
    let mut a: Option<&T> = None;
    let mut b: Option<&T> = None;
    let mut min_interv: Option<T> = None;
    for elem in samples {
        b = Some(elem);
        if let (Some(val_a), Some(val_b)) = (a, b) {
            if let Some(interv) = min_interv {
                if interv > *val_b - *val_a {
                    min_interv = Some(*val_b - *val_a);
                }
            } else {
                min_interv = Some(*val_b - *val_a);
            }
        }
        a = Some(elem);
    }
    match min_interv {
        Some(e) => println!("final min interval is {}", e),
        None => (),
    }
    min_interv
}
