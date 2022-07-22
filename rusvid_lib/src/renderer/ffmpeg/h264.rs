use std::ffi::OsString;

use crate::renderer::CliArgument;

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum H264Preset {
    Ultrafast,
    Superfast,
    Veryfast,
    Faster,
    Fast,
    Medium,
    Slow,
    Slower,
    Veryslow,
    Placebo,
}

impl Default for H264Preset {
    fn default() -> Self {
        Self::Medium
    }
}

impl ToString for H264Preset {
    fn to_string(&self) -> String {
        match self {
            H264Preset::Ultrafast => "ultrafast".to_string(),
            H264Preset::Superfast => "superfast".to_string(),
            H264Preset::Veryfast => "veryfast".to_string(),
            H264Preset::Faster => "faster".to_string(),
            H264Preset::Fast => "fast".to_string(),
            H264Preset::Medium => "medium".to_string(),
            H264Preset::Slow => "slow".to_string(),
            H264Preset::Slower => "slower".to_string(),
            H264Preset::Veryslow => "veryslow".to_string(),
            H264Preset::Placebo => "placebo".to_string(),
        }
    }
}

impl CliArgument for H264Preset {
    #[inline(always)]
    fn build_cli_argument(&self) -> Vec<OsString> {
        vec![
            OsString::from("-preset"),
            OsString::from(self.to_string().as_str()),
        ]
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
pub enum H264Tune {
    Film,
    Animation,
    Grain,
    Stillimage,
    Fastdecode,
    Zerolatency,
}

impl ToString for H264Tune {
    fn to_string(&self) -> String {
        match self {
            H264Tune::Film => "film".to_string(),
            H264Tune::Animation => "animation".to_string(),
            H264Tune::Grain => "grain".to_string(),
            H264Tune::Stillimage => "stillimage".to_string(),
            H264Tune::Fastdecode => "fastdecode".to_string(),
            H264Tune::Zerolatency => "zerolatency".to_string(),
        }
    }
}

impl CliArgument for H264Tune {
    #[inline(always)]
    fn build_cli_argument(&self) -> Vec<OsString> {
        vec![
            OsString::from("-tune"),
            OsString::from(self.to_string().as_str()),
        ]
    }
}

#[derive(Debug, Copy, Clone, Default)]
pub struct H264Settings {
    constant_rate_factor: Option<usize>,
    preset: Option<H264Preset>,
    tune: Option<H264Tune>,
}

impl CliArgument for H264Settings {
    #[inline(always)]
    fn build_cli_argument(&self) -> Vec<OsString> {
        let mut arguments = Vec::new();

        if let Some(crf) = &self.constant_rate_factor {
            arguments.extend_from_slice(&[
                OsString::from("-crf"),
                OsString::from(crf.to_string().as_str()),
            ])
        }

        if let Some(preset) = &self.preset {
            arguments.extend_from_slice(&preset.build_cli_argument());
        }

        if let Some(tune) = &self.tune {
            arguments.extend_from_slice(&tune.build_cli_argument())
        }

        arguments
    }
}
