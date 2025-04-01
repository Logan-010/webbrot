use base64::{
    alphabet::STANDARD,
    engine::{GeneralPurpose, GeneralPurposeConfig},
    Engine,
};
use futures::{channel::mpsc, SinkExt, StreamExt};
use image::{ImageFormat, Rgba, RgbaImage};
use leptos::task;
use num::complex::Complex64;
use std::io::Cursor;

pub mod colormaps;
pub mod options;

pub const MIN_STEPS: u32 = 150;
pub const MAX_STEPS: u32 = 1024;
pub const BAILOUT_NUM: f64 = 15.0;

#[derive(Clone, Copy)]
pub struct MandelbrotConfig {
    pub min_steps: u32,
    pub max_steps: u32,
    pub bailout_num: f64,
}

impl Default for MandelbrotConfig {
    fn default() -> Self {
        Self {
            min_steps: MIN_STEPS,
            max_steps: MAX_STEPS,
            bailout_num: 1.0 * 10.0f64.powf(BAILOUT_NUM),
        }
    }
}

pub fn rand_range(min: f64, max: f64) -> f64 {
    let u = fastrand::f64();
    lerp(min, max, u)
}

pub fn lerp(a: f64, b: f64, u: f64) -> f64 {
    (a) * (1.0 - (u)) + (b) * (u)
}

fn abs_square(input: Complex64) -> f64 {
    input.re.powf(2.0) + input.im.powf(2.0)
}

pub fn mandelbrot(input: (f64, f64), cfg: &MandelbrotConfig) -> u32 {
    let c0 = Complex64::new(input.0, input.1);
    let mut c = c0;
    let mut dc = Complex64::new(1.0, 0.0);
    let mut dc_sum = Complex64::new(0.0, 0.0);

    for n in 1..cfg.max_steps {
        c = c.powf(2.0) + c0;
        dc = 2.0 * dc * c + 1.0;
        dc_sum += dc;

        if abs_square(dc_sum) >= cfg.bailout_num {
            return n;
        }
    }

    0
}

pub fn choose_center(x: &mut f64, y: &mut f64, cfg: &MandelbrotConfig) -> u32 {
    let mut steps = 0;
    while !(cfg.min_steps..cfg.max_steps).contains(&steps) {
        *x = rand_range(-1.5, 1.0);
        *y = rand_range(0.0, 1.0);
        steps = mandelbrot((*x, *y), cfg);
    }
    steps
}

async fn gen_image(options: options::Options) -> Vec<u8> {
    let cfg = MandelbrotConfig {
        min_steps: options.step_limits[0],
        max_steps: options.step_limits[1],
        bailout_num: 1.0 * 10.0f64.powf(options.bailout_num),
    };

    let seed = options.rng_seed.unwrap_or_else(fastrand::get_seed);

    fastrand::seed(seed);

    let (width, height) = (options.dimensions[0], options.dimensions[1]);

    let palette = if let Some(colormap) = options.colormap {
        colormap.to_colormap()
    } else {
        let choice = fastrand::usize(0..options::COLORMAP_CHOICES.len() - 1);

        options::COLORMAP_CHOICES[choice].to_colormap()
    };

    let steps;

    fastrand::seed(seed);

    let center = match options.image_center {
        Some(v) => {
            let point = (v[0], v[1]);
            steps = mandelbrot(point, &cfg);
            point
        }
        None => {
            let (mut x, mut y) = (0.0, 0.0);
            steps = choose_center(&mut x, &mut y, &cfg);
            (x, y)
        }
    };

    fastrand::seed(seed);

    let dx;
    let dy;

    if let Some(size) = options.view_size {
        dx = size[0] / 2.0;
        dy = size[1] / 2.0;
    } else {
        dx = (steps as f64).powf(rand_range(-2.5, -1.0));
        dy = dx * height as f64 / width as f64;
    }

    let (xmin, xmax) = (center.0 - dx, center.0 + dx);
    let (ymin, ymax) = (center.1 - dy, center.1 + dy);

    let mut image = RgbaImage::new(width, height);

    let (tx, rx) = mpsc::channel(100);

    let pixels: Vec<(u32, u32)> = (0..height)
        .flat_map(|y| (0..width).map(|x| (x, y)).collect::<Vec<(u32, u32)>>())
        .collect();

    tracing::info!("Starting generation");
    for pixel_group in pixels.chunks(options.chunk_size) {
        let mut group = vec![(0, 0); pixel_group.len()];
        group.clone_from_slice(pixel_group);
        let mut task_tx = tx.clone();
        task::spawn(async move {
            for pixel in group {
                let scaled = (
                    lerp(xmin, xmax, pixel.0 as f64 / (width as f64 - 1.0)),
                    lerp(ymin, ymax, pixel.1 as f64 / (height as f64 - 1.0)),
                );

                let iteration = mandelbrot(scaled, &cfg);

                let index = (3 * iteration as usize).clamp(0, palette.len() - 3);

                let sample = &palette[index..];

                task_tx
                    .send((
                        pixel.0,
                        pixel.1,
                        Rgba::from([sample[0], sample[1], sample[2], 0xFF]),
                    ))
                    .await
                    .unwrap();
            }
        })
    }
    tracing::info!("Tasks spawned");

    let mut values = rx.take((width * height) as usize);
    while let Some((x, y, color)) = values.next().await {
        image.put_pixel(x, y, color);
    }

    tracing::info!("Pixels recieved");

    let mut bytes: Vec<u8> = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
        .unwrap();

    tracing::info!("Wrote data to image");

    bytes
}

pub async fn encode_image(options: options::Options) -> String {
    let bytes = gen_image(options).await;
    GeneralPurpose::new(&STANDARD, GeneralPurposeConfig::new()).encode(&bytes)
}
