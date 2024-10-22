use std::fmt::{self, Formatter, Debug};

use bitflags::bitflags;
use lzma_rs::lzma_decompress;

use crate::errors::ReplayDataError;

#[derive(Debug, Default)]
pub enum GameMode {
    #[default]
    Osu,
    Taiko,
    CatchTheBeat,
    Mania,
}

impl TryFrom<u8> for GameMode {
    type Error = ReplayDataError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(GameMode::Osu),
            1 => Ok(GameMode::Taiko),
            2 => Ok(GameMode::CatchTheBeat),
            3 => Ok(GameMode::Mania),
            _ => Err(ReplayDataError::InvalidValueError),
        }
    }
}

bitflags! {
    pub struct Mods: u32 {
        const NONE = 0;
        const NO_FAIL = 1 << 0;
        const EASY = 1 << 1;
        const TOUCH_DEVICE = 1 << 2;
        const HIDDEN = 1 << 3;
        const HARD_ROCK = 1 << 4;
        const SUDDEN_DEATH = 1 << 5;
        const DOUBLE_TIME = 1 << 6;
        const RELAX = 1 << 7;
        const HALF_TIME = 1 << 8;
        const NIGHTCORE = 1 << 9;
        const FLASHLIGHT = 1 << 10;
        const AUTOPLAY = 1 << 11;
        const SPUN_OUT = 1 << 12;
        const RELAX2 = 1 << 13;
        const PERFECT = 1 << 14;
        const KEY4 = 1 << 15;
        const KEY5 = 1 << 16;
        const KEY6 = 1 << 17;
        const KEY7 = 1 << 18;
        const KEY8 = 1 << 19;
        const FADE_IN = 1 << 20;
        const RANDOM = 1 << 21;
        const LAST_MOD = 1 << 22;
        const TARGET_PRACTICE = 1 << 23;
        const KEY9 = 1 << 24;
        const COOP = 1 << 25;
        const KEY1 = 1 << 26;
        const KEY3 = 1 << 27;
        const KEY2 = 1 << 28;
        const SCORE_V2 = 1 << 29;
        const MIRROR = 1 << 30;
    }
}

#[derive(Debug, Default)]
pub struct ReplayData {
    pub time: i64,
    pub x: f32,
    pub y: f32,
    pub keys: u32,
}

#[derive(Debug, Default)]
pub struct Replay {
    pub game_mode: GameMode,
    pub version: u32,
    pub beatmap_md5: String,
    pub player_name: String,
    pub replay_md5: String,
    pub n300: u16,
    pub n100: u16,
    pub n50: u16,
    pub n_geki: u16,
    pub n_katu: u16,
    pub n_miss: u16,
    pub total_score: u32,
    pub greatest_combo: u16,
    pub perfect: u8,
    pub mods: u32,
    pub life_bar: String,
    pub time_stamp: i64,
    pub compressed_data: Vec<u8>,
    pub online_score_id: i64,
}

impl fmt::Display for Replay {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "Gamemode: {:?}", self.game_mode)?;
        writeln!(f, "Version: {}", self.version)?;
        writeln!(f, "Beatmap MD5: {}", self.beatmap_md5)?;
        writeln!(f, "Player Name: {}", self.player_name)?;
        writeln!(f, "Replay MD5: {}", self.replay_md5)?;
        writeln!(f, "300s: {}", self.n300)?;
        writeln!(f, "100s: {}", self.n100)?;
        writeln!(f, "50s: {}", self.n50)?;
        writeln!(f, "Gekis: {}", self.n_geki)?;
        writeln!(f, "Katus: {}", self.n_katu)?;
        writeln!(f, "Misses: {}", self.n_miss)?;
        writeln!(f, "Total Score: {}", self.total_score)?;
        writeln!(f, "Greatest Combo: {}", self.greatest_combo)?;
        writeln!(f, "Perfect: {}", self.perfect)?;
        writeln!(f, "Mods: {:b}", self.mods)?;
        writeln!(f, "Life Bar: {}", self.life_bar)?;
        writeln!(f, "Time Stamp: {}", self.time_stamp)?;
        writeln!(f, "Online Score ID: {}", self.online_score_id)?;
        writeln!(f, "Compressed Bytes: {}", self.compressed_data.len())?;

        Ok(())
    }
}

impl Replay {
    fn decompress_lzma(self) -> Result<String, ReplayDataError> {
        let mut decompressed_data = Vec::new();
        lzma_decompress(&mut self.compressed_data.as_slice(), &mut decompressed_data)
            .map_err(|_| ReplayDataError::LzmaError)?;

        let decompressed_data =
            String::from_utf8(decompressed_data).map_err(|_| ReplayDataError::ParseStringError)?;

        Ok(decompressed_data)
    }

    pub fn get_actions(self) -> Result<Vec<ReplayData>, ReplayDataError> {
        let decompressed_data = self.decompress_lzma()?;
        let replay_data: Result<Vec<ReplayData>, ReplayDataError> = decompressed_data
            .split_terminator(',')
            .map(|data| {
                let mut split = data.split('|');
                let time: i64 = split
                    .next()
                    .ok_or(ReplayDataError::MissingValueError)?
                    .parse()?;
                let x: f32 = split
                    .next()
                    .ok_or(ReplayDataError::MissingValueError)?
                    .parse()?;
                let y: f32 = split
                    .next()
                    .ok_or(ReplayDataError::MissingValueError)?
                    .parse()?;
                let keys: u32 = split
                    .next()
                    .ok_or(ReplayDataError::MissingValueError)?
                    .parse()?;

                Ok(ReplayData { time, x, y, keys })
            })
            .collect();

        replay_data
    }
}
