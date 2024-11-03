use std::fmt::{self, Formatter, Debug};

use bitflags::bitflags;
use lzma_rs::lzma_decompress;

use crate::errors::ReplayDataError;

/// Game mode of the replay.
#[derive(Debug, Default)]
pub enum GameMode {
    #[default]
    /// The default osu! game mode.
    Osu,
    /// The Taiko game mode.
    Taiko,
    /// The Catch the Beat game mode.
    CatchTheBeat,
    /// The Mania game mode.
    Mania,
}

impl TryFrom<u8> for GameMode {
    type Error = ReplayDataError<'static>;

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
    /// Flags for the mods used in the replay.
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

/// Struct representing a single action in the replay.
#[derive(Debug, Default)]
pub struct ReplayData {
    /// The time the action was performed.
    pub time: i64,
    /// The x-coordinate of the action.
    pub x: f32,
    /// The y-coordinate of the action.
    pub y: f32,
    /// The keys pressed during the action.
    pub keys: u32,
}

/// Struct representing a replay file.
/// 
/// Use [Self::parse] to parse a replay.
#[derive(Debug, Default)]
pub struct Replay {
    /// The game mode of the replay.
    pub game_mode: GameMode,
    /// The used osu! version to create the replay.
    pub version: u32,
    /// The MD5 hash of the beatmap.
    pub beatmap_md5: String,
    /// The name of the player.
    pub player_name: String,
    /// The MD5 hash of the replay.
    pub replay_md5: String,
    /// Number of 300s
    pub n300: u16,
    /// Number of 100s in standard, 150s in Taiko, 100s in CTB, 100s in mania.
    pub n100: u16,
    /// Number of 50s in standard, small fruit in CTB, 50s in mania.
    pub n50: u16,
    /// Number of Gekis in standard, Max 300s in mania.
    pub n_geki: u16,
    /// Number of Katus in standard, 200s in mania.
    pub n_katu: u16,
    /// Number of misses.
    pub n_miss: u16,
    /// Total score displayed on the score report.
    pub total_score: u32,
    /// Greatest combo displayed on the score report.
    pub greatest_combo: u16,
    /// Perfect/full combo
    pub perfect: u8,
    /// Bitwise representation of the mods used.
    pub mods: u32,
    /// Life bar graph
    pub life_bar: String,
    /// Time of the replay (Windows ticks)
    pub time_stamp: i64,
    /// Compressed replay data
    pub compressed_data: Vec<u8>,
    /// Online score ID
    pub online_score_id: i64,
}

impl fmt::Display for Replay {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
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
    fn decompress_lzma(self) -> Result<String, ReplayDataError<'static>> {
        let mut decompressed_data = Vec::new();
        lzma_decompress(&mut self.compressed_data.as_slice(), &mut decompressed_data)?;
        let decompressed_data =
            String::from_utf8(decompressed_data).map_err(|_| ReplayDataError::InvalidUtfError)?;

        Ok(decompressed_data)
    }

    /// Get a vector of [`ReplayData`](struct.ReplayData.html) from the compressed replay data.
    /// # Example
    /// ```
    /// use osu_replay_parser::{Replay, ReplayData};
    /// use std::fs;
    /// 
    /// let input = fs::read("assets/replay.osr").expect("Error reading file");
    /// let replay = Replay::parse(&input).expect("Error parsing replay");
    /// let actions = replay.get_actions().expect("Error getting actions");
    /// 
    pub fn get_actions(self) -> Result<Vec<ReplayData>, ReplayDataError<'static>> {
        let decompressed_data = self.decompress_lzma()?;
        let replay_data: Result<Vec<ReplayData>, ReplayDataError<'_>> = decompressed_data
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
