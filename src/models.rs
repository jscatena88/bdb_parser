use serde::{Deserialize, Serialize};

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

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct LatLong {
    pub lat: f64,
    pub long: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct BoundingBox {
    pub min: LatLong,
    pub max: LatLong,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Line {
    pub point_1: LatLong,
    pub point_2: LatLong,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum StartFinish {
    Circuit(Line),
    PointToPoint { start: Line, finish: Line },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackChunk {
    pub track_name: String,
    pub bounding_box: BoundingBox,
    pub start_finish: StartFinish,
    pub combo: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileHeader {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub data: [u8; 8],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Region {
    pub bounding_box: BoundingBox,
    pub tracks: Vec<TrackChunk>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Footer {
    pub data: [u8; 4],
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackDatabase {
    pub hdr: FileHeader,
    pub regions: Vec<Region>,
    pub footer: Footer,
}
