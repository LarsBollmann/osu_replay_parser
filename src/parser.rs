use crate::errors::ReplayDataError;
use crate::replay::{GameMode, Replay};

use nom::bytes::complete::take;

// Naming conventions taken from the osu! wiki
use nom::number::complete::{le_i64, le_u16 as short, le_u32 as integer, le_u8 as byte};
use nom::{Finish, IResult};

type ParseResult<I, O> = IResult<I, O, ReplayDataError>;

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

    Err(ReplayDataError::InvalidValueError.into())
}

fn osr_string(input: &[u8]) -> ParseResult<&[u8], &str> {
    let (input, is_present) = byte(input)?;
    if is_present == 0x00 {
        return Ok((input, ""));
    }

    let (input, length) = uleb128(input)?;
    let (input, value) = take(length)(input)?;
    let string = std::str::from_utf8(value).map_err(|_| ReplayDataError::ParseStringError)?;

    Ok((input, string))
}

fn replay_parser(input: &[u8]) -> ParseResult<&[u8], Replay> {
    let (input, game_mode_int) = byte(input)?;
    let game_mode = GameMode::try_from(game_mode_int)?;
    let (input, version) = integer(input)?;
    let (input, beatmap_md5) = osr_string(input)?;
    let (input, player_name) = osr_string(input)?;
    let (input, replay_md5) = osr_string(input)?;
    let (input, n300) = short(input)?;
    let (input, n100) = short(input)?;
    let (input, n50) = short(input)?;
    let (input, n_geki) = short(input)?;
    let (input, n_katu) = short(input)?;
    let (input, n_miss) = short(input)?;
    let (input, total_score) = integer(input)?;
    let (input, greatest_combo) = short(input)?;
    let (input, perfect) = byte(input)?;
    let (input, mods) = integer(input)?;
    let (input, life_bar) = osr_string(input)?;
    let (input, time_stamp) = le_i64(input)?;
    let (input, compressed_length) = integer(input)?;
    let (input, compressed_data) = take(compressed_length)(input)?;
    let (input, online_score_id) = le_i64(input)?;

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

pub fn parse_replay(input: impl AsRef<[u8]>) -> Result<Replay, ReplayDataError> {
    let input = input.as_ref();
    let (_, replay) = replay_parser(input).finish()?;

    Ok(replay)
}
