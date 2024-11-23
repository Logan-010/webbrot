use crate::colormaps;

#[derive(Clone, PartialEq)]
pub struct Options {
    pub dimensions: [u32; 2],
    pub image_center: Option<[f64; 2]>,
    pub view_size: Option<[f64; 2]>,
    pub step_limits: [u32; 2],
    pub colormap: Option<Colormap>,
    pub rng_seed: Option<u64>,
    pub bailout_num: f64,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            dimensions: [960, 540],
            image_center: None,
            view_size: None,
            step_limits: [super::MIN_STEPS, super::MAX_STEPS],
            colormap: None,
            rng_seed: None,
            bailout_num: super::BAILOUT_NUM,
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Colormap {
    Acton,
    Bamako,
    Batlow,
    Berlin,
    Bilbao,
    Broc,
    Broco,
    Buda,
    Cork,
    Corko,
    Davos,
    Devon,
    Grayc,
    Hawaii,
    Imola,
    Lajolla,
    Lapaz,
    Lisbon,
    Nuuk,
    Oleron,
    Oslo,
    Roma,
    Romao,
    Tofino,
    Tokyo,
    Turku,
    Vik,
    Viko,
}

pub const COLORMAP_CHOICES: &[Colormap] = &[
    Colormap::Acton,
    Colormap::Bamako,
    Colormap::Batlow,
    Colormap::Berlin,
    Colormap::Bilbao,
    Colormap::Broc,
    Colormap::Broco,
    Colormap::Buda,
    Colormap::Cork,
    Colormap::Corko,
    Colormap::Davos,
    Colormap::Devon,
    Colormap::Grayc,
    Colormap::Hawaii,
    Colormap::Imola,
    Colormap::Lajolla,
    Colormap::Lapaz,
    Colormap::Lisbon,
    Colormap::Nuuk,
    Colormap::Oleron,
    Colormap::Oslo,
    Colormap::Roma,
    Colormap::Romao,
    Colormap::Tofino,
    Colormap::Tokyo,
    Colormap::Turku,
    Colormap::Vik,
    Colormap::Viko,
];

impl Colormap {
    pub fn to_colormap(self) -> &'static [u8] {
        match self {
            Colormap::Acton => colormaps::ACTON,
            Colormap::Bamako => colormaps::BAMAKO,
            Colormap::Batlow => colormaps::BATLOW,
            Colormap::Berlin => colormaps::BERLIN,
            Colormap::Bilbao => colormaps::BILBAO,
            Colormap::Broc => colormaps::BROC,
            Colormap::Broco => colormaps::BROCO,
            Colormap::Buda => colormaps::BUDA,
            Colormap::Cork => colormaps::CORK,
            Colormap::Corko => colormaps::CORKO,
            Colormap::Davos => colormaps::DAVOS,
            Colormap::Devon => colormaps::DEVON,
            Colormap::Grayc => colormaps::GRAYC,
            Colormap::Hawaii => colormaps::HAWAII,
            Colormap::Imola => colormaps::IMOLA,
            Colormap::Lajolla => colormaps::LAJOLLA,
            Colormap::Lapaz => colormaps::LAPAZ,
            Colormap::Lisbon => colormaps::LISBON,
            Colormap::Nuuk => colormaps::NUUK,
            Colormap::Oleron => colormaps::OLERON,
            Colormap::Oslo => colormaps::OSLO,
            Colormap::Roma => colormaps::ROMA,
            Colormap::Romao => colormaps::ROMAO,
            Colormap::Tofino => colormaps::TOFINO,
            Colormap::Tokyo => colormaps::TOKYO,
            Colormap::Turku => colormaps::TURKU,
            Colormap::Vik => colormaps::VIK,
            Colormap::Viko => colormaps::VIKO,
        }
    }
}
