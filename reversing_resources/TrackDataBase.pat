import std.array;

enum ChunkId: u8{
    TOP = 0xA1,
    MID = 0xA2,
    TRACK = 0xA3,
    TRACK_NAME = 0xA4,
    TRACK_5 = 0xA5,
    TRACK_6 = 0xA6,
    TRACK_7 = 0xA7,
    FOOTER = 0xEE,
};

struct TrackData {
    ChunkId marker;
    u8 len;
    padding[2];
    match (marker) {
        (ChunkId::TRACK_NAME):  char track_name[len-4] [[color("00FF00")]];
        (_): u8 bonus_data[len-4] [[color("FF0000")]];
    }
};

struct TrackChunk {
    ChunkId id;
    u16 len;
    padding[1];
    s32 track_data[4];
    std::ByteSizedArray<TrackData, len-20> data;
};

struct FileHeader {
    ChunkId id;
    u16 len;
    padding[1];
    u8 data[12];
};

struct MidChunk {
    ChunkId id;
    u16 len;
    padding[1];
    s32 mid_data[4];
    std::ByteSizedArray<TrackChunk, len-20> tracks;
};

struct File {
    FileHeader hdr;
    MidChunk mids[65];
    u8 footer[8];
    
};


File f1 @ 0x0;