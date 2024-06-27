# Track Database (.BDB) File Format Documentation

## Overview

This document describes the binary format of the .BDB file, which contains a database of race tracks around the world. The format consists of a file header, multiple region chunks, and a footer.

## File Structure

1. File Header (16 bytes)
2. Region Chunks (variable size)
3. Footer (8 bytes)

## Detailed Structure

### File Header

Total size: 16 bytes

| Offset | Size (bytes) | Description     | Value/Type     |
|--------|--------------|-----------------|----------------|
| 0x00   | 1            | Chunk ID        | 0xA1           |
| 0x01   | 2            | Length          | uint16_le      |
| 0x03   | 1            | Padding         | 0x00           |
| 0x04   | 2            | Year            | uint16_le      |
| 0x06   | 1            | Month           | uint8          |
| 0x07   | 1            | Day             | uint8          |
| 0x08   | 8            | Unknown data    | byte[8]        |

### Region Chunk

Total size is variable.

| Offset | Size (bytes) | Description     | Value/Type     |
|--------|--------------|-----------------|----------------|
| 0x00   | 1            | Chunk ID        | 0xA2           |
| 0x01   | 2            | Length          | uint16_le      |
| 0x03   | 1            | Padding         | 0x00           |
| 0x04   | 16           | Bounding Box    | 2 x LatLong    |
| 0x14   | Variable     | Track Chunks    | Track Chunk[]  |

### Track Chunk

| Offset | Size (bytes) | Description     | Value/Type     |
|--------|--------------|-----------------|----------------|
| 0x00   | 1            | Chunk ID        | 0xA3           |
| 0x01   | 2            | Length          | uint16_le      |
| 0x03   | 1            | Padding         | 0x00           |
| 0x04   | 16           | Bounding Box    | 2 x LatLong    |
| 0x14   | Variable     | Track Data      | TrackData[]    |

#### Track Data

Can be one of four types:

1. Track Name (Chunk ID: 0xA4)
   - Length: 1 byte
   - Padding: 2 bytes
   - Name: UTF-8 encoded string (length - 4 bytes)

2. Start Line (Chunk ID: 0xA5)
   - Length: 1 byte (always 20)
   - Padding: 2 bytes
   - Line: 2 x LatLong (16 bytes)

3. Combo Flag (Chunk ID: 0xA7)
   - Length: 1 byte
   - Padding: 2 bytes
   - Combo: uint8 (1 byte)

4. Finish Line (Chunk ID: 0xA6)
   - Length: 1 byte (always 20)
   - Padding: 2 bytes
   - Line: 2 x LatLong (16 bytes)

### Footer

Total size: 8 bytes

| Offset | Size (bytes) | Description     | Value/Type     |
|--------|--------------|-----------------|----------------|
| 0x00   | 1            | Chunk ID        | 0xEE           |
| 0x01   | 2            | Length          | uint16_le      |
| 0x03   | 1            | Padding         | 0x00           |
| 0x04   | 4            | Unknown data    | byte[4]        |

## Coordinate System

Latitude and longitude are stored as int32 values. To convert to decimal degrees:

1. Divide the value by 100000
2. Divide the result by 60

## Track Types

Tracks can be of two types:
1. Circuit: Only has a start line, which also serves as the finish line.
2. Point-to-Point: Has separate start and finish lines.

## Notes

- All multi-byte integer values are stored in little-endian format.
- The file uses a chunk-based structure, with each chunk identified by a single-byte ID.
- Each region chunk contains a variable number of track chunks.
- The combo flag indicates if the track is a combination of multiple tracks.

## Gaps in Understanding

1. The purpose of the 8 bytes of data in the File Header is unknown.
3. The meaning and structure of the 4-byte footer data are unclear.

This document represents the current understanding of the .BDB file format and may be updated as more information becomes available.