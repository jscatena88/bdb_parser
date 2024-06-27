//! This module contains the parsing functions for the file format.

use nom::{
    bytes::complete::{tag, take},
    combinator::{map, verify},
    error::ErrorKind,
    multi::{count, many0},
    number::complete::{le_i32, le_u16, le_u8},
    sequence::tuple,
    IResult,
};

use crate::models::*;

pub enum TrackData {
    TrackName(String),
    StartLine(Line),
    ComboFlag,
    FinishLine(Line),
}

pub fn parse_chunk_id(input: &[u8]) -> IResult<&[u8], ChunkIdentifier> {
    let (input, val) = le_u8(input)?;
    Ok((
        input,
        ChunkIdentifier::try_from(val)
            .map_err(|_| nom::Err::Error(nom::error::Error::new(input, ErrorKind::Verify)))?,
    ))
}

pub fn parse_lat_long_raw(input: &[u8]) -> IResult<&[u8], LatLong> {
    map(tuple((le_i32, le_i32)), |(lat, long)| LatLong {
        lat: (lat as f64) / (100000.0 * 60.0),
        long: (long as f64) / (100000.0 * 60.0),
    })(input)
}

pub fn parse_bounding_box(input: &[u8]) -> IResult<&[u8], BoundingBox> {
    map(
        tuple((parse_lat_long_raw, parse_lat_long_raw)),
        |(min, max)| BoundingBox {
            corner_1: min,
            corner_2: max,
        },
    )(input)
}

pub fn parse_line(input: &[u8]) -> IResult<&[u8], Line> {
    map(
        tuple((parse_lat_long_raw, parse_lat_long_raw)),
        |(min, max)| Line {
            point_1: min,
            point_2: max,
        },
    )(input)
}

pub fn parse_track_data(input: &[u8]) -> IResult<&[u8], TrackData> {
    let (input, chunk_id) = parse_chunk_id(input)?;
    let (input, len) = le_u8(input)?;
    let (input, _) = tag([0, 0])(input)?; // padding

    match chunk_id {
        ChunkIdentifier::TrackName => {
            let (input, name) = map(take(len as usize - 4), |s: &[u8]| {
                String::from_utf8_lossy(s).into_owned()
            })(input)?;
            Ok((input, TrackData::TrackName(name)))
        }
        ChunkIdentifier::StartLine => {
            let (input, start) = parse_line(input)?;
            Ok((input, TrackData::StartLine(start)))
        }
        ChunkIdentifier::ComboFlag => {
            let (input, _combo) = le_u8(input)?;
            Ok((input, TrackData::ComboFlag))
        }
        ChunkIdentifier::FinishLine => {
            let (input, finish) = parse_line(input)?;
            Ok((input, TrackData::FinishLine(finish)))
        }
        _ => panic!("Unknown chunk id: {:?}", chunk_id),
    }
}

pub fn parse_track_chunk(input: &[u8]) -> IResult<&[u8], TrackChunk> {
    let (input, _) = verify(parse_chunk_id, |&id| id == ChunkIdentifier::Track)(input)?;
    let (input, len) = le_u16(input)?;
    let (input, _) = tag([0])(input)?; // padding
    let (input, bounding_box) = parse_bounding_box(input)?;

    let data_len = len as usize - 20; // 20 is the size of the track chunk header
    let (input, data_bytes) = take(data_len)(input)?;
    let (_, data) = many0(parse_track_data)(data_bytes)?;

    let name = data.iter().find_map(|d| match d {
        TrackData::TrackName(name) => Some(name.clone()),
        _ => None,
    });

    let combo = data.iter().any(|d| match d {
        TrackData::ComboFlag => true,
        _ => false,
    });

    let start = data.iter().find_map(|d| match d {
        TrackData::StartLine(data) => Some(data.clone()),
        _ => None,
    });

    let finish = data.iter().find_map(|d| match d {
        TrackData::FinishLine(data) => Some(data.clone()),
        _ => None,
    });

    let start_finish = match (start, finish) {
        (Some(start), Some(finish)) => Some(StartFinish::PointToPoint { start, finish }),
        (Some(start), None) => Some(StartFinish::Circuit(start)),
        _ => None,
    };

    Ok((
        input,
        TrackChunk {
            bounding_box,
            start_finish: start_finish.unwrap(),
            track_name: name.unwrap(),
            combo,
        },
    ))
}

pub fn parse_region_chunk(input: &[u8]) -> IResult<&[u8], Region> {
    let (input, _) = verify(parse_chunk_id, |&id| id == ChunkIdentifier::Region)(input)?;
    let (input, total_len) = le_u16(input)?;
    let (input, _) = tag([0])(input)?; // padding
    let (input, bounding_box) = parse_bounding_box(input)?;

    // Calculate the length of track chunks data
    let tracks_data_len = total_len as usize - 20; // 20 is the size of the mid chunk header

    // Take the track chunks data
    let (input, tracks_data) = take(tracks_data_len)(input)?;

    // Parse the track chunks from the taken data
    let (_, tracks) = many0(parse_track_chunk)(tracks_data)?;

    Ok((
        input,
        Region {
            bounding_box,
            tracks,
        },
    ))
}

pub fn parse_footer(input: &[u8]) -> IResult<&[u8], Footer> {
    let (input, _) = verify(parse_chunk_id, |&id| id == ChunkIdentifier::FileFooter)(input)?;
    let (input, _total_len) = le_u16(input)?;
    let (input, _) = tag([0])(input)?; // padding

    let (input, data) = take(4usize)(input)?;

    Ok((
        input,
        Footer {
            data: [data[0], data[1], data[2], data[3]],
        },
    ))
}

pub fn parse_file_header(input: &[u8]) -> IResult<&[u8], FileHeader> {
    let (input, _) = verify(parse_chunk_id, |&id| id == ChunkIdentifier::FileHeader)(input)?;
    let (input, _) = le_u16(input)?;
    let (input, _) = tag([0])(input)?; // padding

    let (input, year) = le_u16(input)?;
    let (input, month) = le_u8(input)?;
    let (input, day) = le_u8(input)?;

    let (input, data) = take(8usize)(input)?;

    Ok((
        input,
        FileHeader {
            year,
            month,
            day,
            data: [
                data[0], data[1], data[2], data[3], data[4], data[5], data[6], data[7],
            ],
        },
    ))
}

/// This is the top level parsing function that accepts the entire file and returns a `TrackDatabase`.
pub fn parse_track_database(input: &[u8]) -> IResult<&[u8], TrackDatabase> {
    let (input, hdr) = parse_file_header(input)?;
    let (input, mids) = count(parse_region_chunk, 65)(input)?;
    let (input, footer) = parse_footer(input)?;

    Ok((
        input,
        TrackDatabase {
            hdr,
            regions: mids,
            footer,
        },
    ))
}
