use num::complex::Complex64;

pub mod colormaps;
pub mod download;
pub mod options;
pub mod renderer;

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

pub(crate) struct View {
    pub bounds: [f64; 4],
    pub palette: &'static [u8],
    pub config: MandelbrotConfig,
}

pub(crate) fn prepare_view(options: &options::Options) -> Result<View, String> {
    if options.dimensions.iter().any(|&n| n < 2) {
        return Err("Width and height must be at least 2 pixels.".into());
    }
    if options.step_limits[1] < 2
        || (options.image_center.is_none()
            && (options.step_limits[0] == 0 || options.step_limits[0] >= options.step_limits[1]))
    {
        return Err(
            "For a random center, steps must satisfy 1 ≤ min < max; max must be at least 2.".into(),
        );
    }
    if !options.bailout_num.is_finite() || !(0.0..=38.0).contains(&options.bailout_num) {
        return Err("The bailout exponent must be between 0 and 38 for GPU rendering.".into());
    }
    if options
        .image_center
        .is_some_and(|c| c.iter().any(|n| !n.is_finite()))
        || options
            .view_size
            .is_some_and(|s| s.iter().any(|n| !n.is_finite() || *n <= 0.0))
    {
        return Err("The center must be finite and both view sizes must be positive.".into());
    }
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
            // Bound the search so impossible step ranges cannot hang the browser forever.
            let mut found = None;
            for _ in 0..10_000 {
                x = rand_range(-1.5, 1.0);
                y = rand_range(0.0, 1.0);
                let n = mandelbrot((x, y), &cfg);
                if (cfg.min_steps..cfg.max_steps).contains(&n) {
                    found = Some(n);
                    break;
                }
            }
            steps = found
                .ok_or("No suitable random center found. Try a different seed or step range.")?;
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
        if steps == 0 {
            return Err(
                "This center does not escape. Specify both view sizes to render it.".into(),
            );
        }
        dx = (steps as f64).powf(rand_range(-2.5, -1.0));
        dy = dx * height as f64 / width as f64;
    }

    let (xmin, xmax) = (center.0 - dx, center.0 + dx);
    let (ymin, ymax) = (center.1 - dy, center.1 + dy);

    Ok(View {
        bounds: [xmin, xmax, ymin, ymax],
        palette,
        config: cfg,
    })
}
