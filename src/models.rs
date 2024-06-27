//! This module contains the data structures and enums used to represent the parsed data.

use serde::{Deserialize, Serialize};

/// The identifier for a chunk in the file.
#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum ChunkIdentifier {
    FileHeader = 0xA1,
    Region = 0xA2,
    Track = 0xA3,
    TrackName = 0xA4,
    StartLine = 0xA5,
    FinishLine = 0xA6,
    ComboFlag = 0xA7,
    FileFooter = 0xEE,
}

impl TryFrom<u8> for ChunkIdentifier {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0xA1 => Ok(ChunkIdentifier::FileHeader),
            0xA2 => Ok(ChunkIdentifier::Region),
            0xA3 => Ok(ChunkIdentifier::Track),
            0xA4 => Ok(ChunkIdentifier::TrackName),
            0xA5 => Ok(ChunkIdentifier::StartLine),
            0xA6 => Ok(ChunkIdentifier::FinishLine),
            0xA7 => Ok(ChunkIdentifier::ComboFlag),
            0xEE => Ok(ChunkIdentifier::FileFooter),
            _ => Err(()),
        }
    }
}

/// The latitude and longitude of a point.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct LatLong {
    pub lat: f64,
    pub long: f64,
}

/// The bounding box of a region or track.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct BoundingBox {
    pub corner_1: LatLong,
    pub corner_2: LatLong,
}

/// A line segment between two points.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Line {
    pub point_1: LatLong,
    pub point_2: LatLong,
}

/// The start and finish of a track.
#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum StartFinish {
    /// A circuit track.
    Circuit(Line),
    /// A point-to-point track.
    PointToPoint { start: Line, finish: Line },
}

/// All the data for a single track.
#[derive(Debug, Serialize, Deserialize)]
pub struct TrackChunk {
    /// The name of the track.
    pub track_name: String,
    /// The bounding box of the track.
    pub bounding_box: BoundingBox,
    /// The start and finish of the track.
    pub start_finish: StartFinish,
    /// Whether the track is a combo track (Not sure exactly
    ///  what this is but there is a flag set for every track
    /// that has "combo" in the name.)
    pub combo: bool,
}

/// The header of the file.
#[derive(Debug, Serialize, Deserialize)]
pub struct FileHeader {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    /// Currently unknown data.
    pub data: [u8; 8],
}

/// A geographic region as indicated in the file
/// All tracks get grouped under a region. The region is defined by a bounding box.
#[derive(Debug, Serialize, Deserialize)]
pub struct Region {
    /// The bounding box of the region.
    pub bounding_box: BoundingBox,
    /// The tracks in the region.
    pub tracks: Vec<TrackChunk>,
}

/// The footer of the file.
#[derive(Debug, Serialize, Deserialize)]
pub struct Footer {
    /// Currently unknown data.
    pub data: [u8; 4],
}

/// The entire database of tracks as stored in the file.
#[derive(Debug, Serialize, Deserialize)]
pub struct TrackDatabase {
    /// The header of the file.
    pub hdr: FileHeader,
    /// The regions in the file.
    pub regions: Vec<Region>,
    /// The footer of the file.
    pub footer: Footer,
}
