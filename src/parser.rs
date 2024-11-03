use crate::errors::{from_context, ReplayDataError};
use crate::replay::{GameMode, Replay};

use nom::bytes::complete::take;

use nom::error::{context, ParseError, VerboseError};
// Naming conventions taken from the osu! wiki
use nom::number::complete::{le_i64, le_u16 as short, le_u32 as integer, le_u8 as byte};
use nom::{Finish, IResult};

type ParseResult<I, O> = IResult<I, O, VerboseError<I>>;

fn uleb128(input: &[u8]) -> ParseResult<&[u8], u32> {
    let mut result = 0;
    let mut shift = 0;

    for (bytes, byte) in input.iter().enumerate() {
        let byte = *byte as u32;
        result |= (byte & 0x7F) << shift;
        shift += 7;

        if byte & 0x80 == 0 {
            return Ok((&input[bytes + 1..], result));
        }
    }

    Err(nom::Err::Error(VerboseError::from_error_kind(
        input,
        nom::error::ErrorKind::Eof,
    )))
}

fn utf8_string(input: &[u8]) -> ParseResult<&[u8], &str> {
    let str = std::str::from_utf8(input)
        .map_err(|_| nom::Err::Error(from_context(input, "Error converting bytes to UTF-8")))?;

    Ok((b"", str))
}

fn osr_string(input: &[u8]) -> ParseResult<&[u8], &str> {
    let (input, is_present) = byte(input)?;
    if is_present == 0x00 {
        return Ok((input, ""));
    }

    let (input, length) = context("Error parsing ULEB128 for String length", uleb128)(input)?;
    let (input, value) = context("Expected more bytes for string", take(length))(input)?;
    let (_, string) = utf8_string(value)?;

    Ok((input, string))
}

fn game_mode(input: &[u8]) -> ParseResult<&[u8], GameMode> {
    let (input, game_mode_int) = byte(input)?;

    let game_mode = match game_mode_int {
        0 => GameMode::Osu,
        1 => GameMode::Taiko,
        2 => GameMode::CatchTheBeat,
        3 => GameMode::Mania,
        _ => return Err(nom::Err::Error(from_context(input, "Invalid Game Mode"))),
    };

    Ok((input, game_mode))
}

fn replay_parser(input: &[u8]) -> ParseResult<&[u8], Replay> {
    let (input, game_mode) = context("Error parsing game mode", game_mode)(input)?;
    let (input, version) = context("Error parsing game version", integer)(input)?;
    let (input, beatmap_md5) = context("Error parsing beatmap MD5", osr_string)(input)?;
    let (input, player_name) = context("Error parsing player name", osr_string)(input)?;
    let (input, replay_md5) = context("Error parsing replay MD5", osr_string)(input)?;
    let (input, n300) = context("Error parsing 300s count", short)(input)?;
    let (input, n100) = context("Error parsing 100s count", short)(input)?;
    let (input, n50) = context("Error parsing 50s count", short)(input)?;
    let (input, n_geki) = context("Error parsing gekis count", short)(input)?;
    let (input, n_katu) = context("Error parsing katus count", short)(input)?;
    let (input, n_miss) = context("Error parsing misses count", short)(input)?;
    let (input, total_score) = context("Error parsing total score", integer)(input)?;
    let (input, greatest_combo) = context("Error parsing greatest combo", short)(input)?;
    let (input, perfect) = context("Error parsing perfect", byte)(input)?;
    let (input, mods) = context("Error parsing mods", integer)(input)?;
    let (input, life_bar) = context("Error parsing life bar", osr_string)(input)?;
    let (input, time_stamp) = context("Error parsing time stamp", le_i64)(input)?;
    let (input, compressed_length) = context("Error parsing compressed length", integer)(input)?;
    let (input, compressed_data) =
        context("Error parsing compressed data", take(compressed_length))(input)?;
    let (input, online_score_id) = context("Error parsing online score ID", le_i64)(input)?;

    let replay = Replay {
        game_mode,
        version,
        beatmap_md5: beatmap_md5.to_string(),
        player_name: player_name.to_string(),
        replay_md5: replay_md5.to_string(),
        n300,
        n100,
        n50,
        n_geki,
        n_katu,
        n_miss,
        total_score,
        greatest_combo,
        perfect,
        mods,
        life_bar: life_bar.to_string(),
        time_stamp,
        compressed_data: compressed_data.to_vec(),
        online_score_id,
    };

    Ok((input, replay))
}

impl Replay {
    /// Parse an osu! replay file into a `Replay` struct.
    /// # Example
    /// ```
    /// use osu_replay_parser::{Replay};
    /// use std::fs;
    ///
    ///
    /// // This is a sample replay file
    /// let input = fs::read("assets/replay.osr").expect("Error reading file");
    /// let replay = Replay::parse(&input).expect("Error parsing replay");
    ///
    /// ```
    /// # Errors
    /// Returns a `ReplayDataError` if the replay file is invalid or cannot be parsed.
    ///
    pub fn parse(input: &[u8]) -> Result<Self, ReplayDataError<'_>> {
        let (_, replay) = context("Error parsing replay file", replay_parser)(input).finish()?;

        Ok(replay)
    }
}
