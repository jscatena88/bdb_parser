# Racetrack Database File Format Documentation

This document describes the file format used to store racetrack location and information data.

## File Structure Overview

The file format is organized into a series of "chunks". A chunk is a self-contained unit of data with a specific structure:

1. A single-byte identifier that indicates the type of data in the chunk
2. A two-byte length field (in little-endian format) that specifies the total length of the chunk in bytes, including the identifier and length fields
3. One byte of padding (always set to 0)
4. The actual data content of the chunk

The file consists of the following main sections, each represented by one or more chunks:

1. File Header (single chunk)
2. Variable number of Region Chunks
3. File Footer (single chunk)

## Chunk Identifiers

Each chunk in the file is identified by a single byte:

| Identifier | Meaning     |
|------------|-------------|
| 0xA1       | File Header |
| 0xA2       | Region      |
| 0xA3       | Track       |
| 0xA4       | Track Name  |
| 0xA5       | Start Line  |
| 0xA6       | Finish Line |
| 0xA7       | Combo Flag  |
| 0xEE       | File Footer |

## File Header Chunk

```
                     1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+---------------+---------------+---------------+---------------+
|     0xA1      | File Length (2 bytes, little-endian) |   0x00 |
+---------------+---------------+---------------+---------------+
|           Year (2 bytes, little-endian)       |     Month     |
+---------------+---------------+---------------+---------------+
|      Day      |                 Unknown Data (8 bytes)        |
+---------------+---------------+---------------+---------------+
|                       Unknown Data (cont.)                    |
+---------------+---------------+---------------+---------------+
|Unk.Data(cont.)|
+---------------+
```

The File Header chunk contains:
- Chunk identifier (0xA1)
- Length of the entire file (2 bytes, little-endian)
- 1 byte of padding (0)
- Year (2 bytes, little-endian)
- Month (1 byte)
- Day (1 byte)
- 8 bytes of unknown data

Since the length field of this chunk is the length of the entire file it can actually be thought of as the entire file is part of this chunk's data. So it could be more accurate to think of this as a "top-level of the file" chunk, that contains all the data in the file, as opposed to a header sitting on top of the file. It makes me wonder if this format was made to embed in a larger format or protocol 

## Region Chunk

```
                     1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+---------------+---------------+---------------+---------------+
|     0xA2      |   Length (2 bytes, little-endian)   |   0x00  |
+---------------+---------------+---------------+---------------+
|                    Bounding Box (16 bytes)                    |
+---------------+---------------+---------------+---------------+
|                    Bounding Box (cont.)                       |
+---------------+---------------+---------------+---------------+
|                    Bounding Box (cont.)                       |
+---------------+---------------+---------------+---------------+
|                    Bounding Box (cont.)                       |
+---------------+---------------+---------------+---------------+
|                  Track Chunks (Variable Size)                 |
+---------------+---------------+---------------+---------------+
```

The Region Chunk contains:
- Chunk identifier (0xA2)
- Length of the region chunk (2 bytes, little-endian)
- 1 byte of padding (0)
- Bounding Box (16 bytes, two LatLon pairs)
- Multiple Track Chunks

## Track Chunk

```
                     1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+---------------+---------------+---------------+---------------+
|     0xA3      |   Length (2 bytes, little-endian)   |   0x00  |
+---------------+---------------+---------------+---------------+
|                    Bounding Box (16 bytes)                    |
+---------------+---------------+---------------+---------------+
|                    Bounding Box (cont.)                       |
+---------------+---------------+---------------+---------------+
|                    Bounding Box (cont.)                       |
+---------------+---------------+---------------+---------------+
|                    Bounding Box (cont.)                       |
+---------------+---------------+---------------+---------------+
|                Track Data Chunks (Variable Size)              |
+---------------+---------------+---------------+---------------+
```

The Track Chunk contains:
- Chunk identifier (0xA3)
- Length of the track chunk (2 bytes, little-endian)
- 1 byte of padding (0)
- Bounding Box (16 bytes, two LatLon pairs)
- Track Data (variable length, consists of multiple sub-chunks)

## Track Data Chunks

Track Data consists of multiple Chunks, each following this format:

```
                     1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+---------------+---------------+---------------+---------------+
| Chunk ID (1B) |     Length (2 bytes, little-endian)  |  0x00  |
+---------------+---------------+---------------+---------------+
|                         Chunk Data                            |
+---------------+---------------+---------------+---------------+
```

### Track Data Types

1. **Track Name (0xA4)**
   - Always Present
   - Contains the name of the track as a UTF-8 encoded string.
   - The length field indicates the total length of the chunk, including the 4-byte header.
   - Actual string length = chunk length - 4 bytes.


2. **Start Line (0xA5)**
   - Always Present
   - Represents the start line of the track.
   - Contains two LatLon pairs (16 bytes total) defining the start line.

3. **Combo Flag (0xA7)**
   - Not Always Present (if it is present it seems to always be 1/True)
   - Indicates whether the track is a "combo" track.
   - Contains a single byte (0 or 1) after the chunk header.

4. **Finish Line (0xA6)**
   - Not Always Present (Only present if the track start and finish line are not the same, i.e. a point to point race not a circuit)
   - Represents the finish line of the track.
   - Contains two LatLon pairs (16 bytes total) defining the finish line.

## LatLon Format

Latitude and Longitude are stored as 32-bit integers (little-endian). Their are calculated as the GPS minutes * 10000

They can be converted to decimal degrees with the following:
```
degrees = (integer_value as f64) / (100000.0 * 60.0)
```

## File Footer Chunk

```
                     1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+---------------+---------------+---------------+---------------+
|     0xEE      |   Length (2 bytes, little-endian)   |   0x00  |
+---------------+---------------+---------------+---------------+
|                       Unknown Data (4 bytes)                  |
+---------------+---------------+---------------+---------------+
```

The File Footer chunk contains:
- Chunk identifier (0xEE)
- Length of the footer (2 bytes, little-endian)
- 1 byte of padding (0)
- 4 bytes of unknown data

## Notes

- All multi-byte integers are stored in little-endian format.
- Padding bytes are set to 0.
- The file can contain multiple Region Chunks, each containing multiple Track Chunks.
