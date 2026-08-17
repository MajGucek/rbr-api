use std::{
    fs::{self, File},
    io::{
        self,
        BufReader,
        Read,
        Seek,
        SeekFrom,
    },
    path::{Path, PathBuf},
};

use super::Vector3;

const PHYSICS_CATEGORY: u32 = 0x03;
const DRIVELINE_TYPE: u32 = 0x14;
const SHAPE_COLLISION_MESHES_TYPE: u32 = 0x16;

const TRK_HEADER_SIZE: u32 = 8;
const DRIVELINE_POINT_SIZE: u64 = 32;

#[derive(Debug, Clone, PartialEq)]
pub enum TrkSegment {
    Driveline(Driveline),

    ShapeCollisionMeshes {
        byte_len: u32,
    },

    Unknown {
        category: u32,
        segment_type: u32,
        byte_len: u32,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Driveline {
    pub points: Vec<DrivelinePoint>,
}

impl Driveline {
    pub fn point_at_or_after(
        &self,
        location: f32,
    ) -> Option<DrivelinePoint> {
        self.points
            .iter()
            .find(|point| {
                point.location >= location
            })
            .copied()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DrivelinePoint {
    pub position: Vector3,
    pub direction: Vector3,

    
    pub location: f32,
}

pub struct TrkReader;

impl TrkReader {
    pub fn load(
        track_id: i32,
    ) -> io::Result<Vec<TrkSegment>> {
        let executable = std::env::current_exe()?;

        let rbr_directory =
            executable.parent().ok_or_else(|| {
                io::Error::new(
                    io::ErrorKind::NotFound,
                    "RBR executable has no parent directory",
                )
            })?;

        let maps_directory =
            rbr_directory.join("Maps");

        let track_path =
            find_track_file(&maps_directory, track_id)?;

        Self::load_file(track_path)
    }

    fn load_file(
        path: impl AsRef<Path>,
    ) -> io::Result<Vec<TrkSegment>> {
        let file = File::open(path)?;
        let file_length = file.metadata()?.len();

        let mut reader = BufReader::new(file);
        let mut offset = 0_u64;
        let mut segments = Vec::new();

        while offset < file_length {
            if file_length - offset < 16 {
                return Err(invalid_data(
                    "truncated TRK segment header",
                ));
            }

            let mut header = [0_u8; 16];
            reader.read_exact(&mut header)?;
            offset += 16;

            let header_size =
                u32_at(&header, 0);

            let category =
                u32_at(&header, 4);

            let segment_type =
                u32_at(&header, 8);

            let data_size =
                u32_at(&header, 12);

            if header_size != TRK_HEADER_SIZE {
                return Err(invalid_data(format!(
                    "invalid TRK header size \
                     {header_size}, expected \
                     {TRK_HEADER_SIZE}",
                )));
            }

            if u64::from(data_size)
                > file_length - offset
            {
                return Err(invalid_data(format!(
                    "TRK segment \
                     0x{segment_type:02X} is truncated",
                )));
            }

            let segment = match (
                category,
                segment_type,
            ) {
                (
                    PHYSICS_CATEGORY,
                    DRIVELINE_TYPE,
                ) => {
                    let driveline =
                        read_driveline(
                            &mut reader,
                            data_size,
                        )?;

                    TrkSegment::Driveline(driveline)
                }

                (
                    PHYSICS_CATEGORY,
                    SHAPE_COLLISION_MESHES_TYPE,
                ) => {
                    skip_bytes(
                        &mut reader,
                        data_size,
                    )?;

                    TrkSegment::ShapeCollisionMeshes {
                        byte_len: data_size,
                    }
                }

                _ => {
                    skip_bytes(
                        &mut reader,
                        data_size,
                    )?;

                    TrkSegment::Unknown {
                        category,
                        segment_type,
                        byte_len: data_size,
                    }
                }
            };

            offset += u64::from(data_size);
            segments.push(segment);
        }

        Ok(segments)
    }
}

fn read_driveline(
    reader: &mut impl Read,
    data_size: u32,
) -> io::Result<Driveline> {
    if data_size < 4 {
        return Err(invalid_data(
            "driveline segment has no point count",
        ));
    }

    let point_count = read_u32(reader)?;

    let expected_size = 4_u64
        + u64::from(point_count)
        * DRIVELINE_POINT_SIZE;

    if expected_size != u64::from(data_size) {
        return Err(invalid_data(format!(
            "invalid driveline size: \
             {point_count} points require \
             {expected_size} bytes, \
             segment contains {data_size}",
        )));
    }

    let point_count =
        usize::try_from(point_count).map_err(|_| {
            invalid_data(
                "driveline point count is too large",
            )
        })?;

    let mut points = Vec::new();

    points
        .try_reserve_exact(point_count)
        .map_err(|_| {
            invalid_data(
                "could not allocate driveline points",
            )
        })?;

    for _ in 0..point_count {
        let position = read_vector3(reader)?;
        let direction = read_vector3(reader)?;
        let location = read_f32(reader)?;

        // Two currently unknown u16 values.
        let mut unknown = [0_u8; 4];
        reader.read_exact(&mut unknown)?;

        points.push(DrivelinePoint {
            position,
            direction,
            location,
        });
    }

    Ok(Driveline { points })
}

fn read_vector3(
    reader: &mut impl Read,
) -> io::Result<Vector3> {
    Ok(Vector3 {
        x: read_f32(reader)?,
        y: read_f32(reader)?,
        z: read_f32(reader)?,
    })
}

fn read_f32(
    reader: &mut impl Read,
) -> io::Result<f32> {
    Ok(f32::from_bits(read_u32(reader)?))
}

fn read_u32(
    reader: &mut impl Read,
) -> io::Result<u32> {
    let mut bytes = [0_u8; 4];
    reader.read_exact(&mut bytes)?;

    Ok(u32::from_le_bytes(bytes))
}

fn skip_bytes(
    reader: &mut impl Seek,
    count: u32,
) -> io::Result<()> {
    reader.seek(SeekFrom::Current(
        i64::from(count),
    ))?;

    Ok(())
}

fn find_track_file(
    maps_directory: &Path,
    track_id: i32,
) -> io::Result<PathBuf> {
    let directory_prefix = format!("{track_id}-");
    let file_prefix =
        format!("track-{track_id}_").to_ascii_lowercase();

    let preferred_name =
        format!("track-{track_id}_O.trk");

    let mut candidates = Vec::new();

    for entry in fs::read_dir(maps_directory)? {
        let entry = entry?;

        if !entry.file_type()?.is_dir() {
            continue;
        }

        let directory_name = entry.file_name();
        let directory_name =
            directory_name.to_string_lossy();

        if !directory_name.starts_with(
            &directory_prefix,
        ) {
            continue;
        }

        for file in fs::read_dir(entry.path())? {
            let file = file?;

            if !file.file_type()?.is_file() {
                continue;
            }

            let path = file.path();
            let file_name = file.file_name();
            let file_name =
                file_name.to_string_lossy();

            let is_trk = path
                .extension()
                .and_then(|extension| {
                    extension.to_str()
                })
                .is_some_and(|extension| {
                    extension.eq_ignore_ascii_case(
                        "trk",
                    )
                });

            if is_trk
                && file_name
                .to_ascii_lowercase()
                .starts_with(&file_prefix)
            {
                candidates.push(path);
            }
        }
    }

    candidates.sort_by_key(|path| {
        let is_preferred = path
            .file_name()
            .and_then(|name| name.to_str())
            .is_some_and(|name| {
                name.eq_ignore_ascii_case(
                    &preferred_name,
                )
            });

        // `_M.trk` sorts before all other variants.
        !is_preferred
    });

    candidates.into_iter().next().ok_or_else(|| {
        io::Error::new(
            io::ErrorKind::NotFound,
            format!(
                "no file matching \
                 Maps\\{track_id}-*\\\
                 track-{track_id}_*.trk",
            ),
        )
    })
}

fn u32_at(
    bytes: &[u8],
    offset: usize,
) -> u32 {
    u32::from_le_bytes([
        bytes[offset],
        bytes[offset + 1],
        bytes[offset + 2],
        bytes[offset + 3],
    ])
}

fn invalid_data(
    message: impl Into<String>,
) -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidData,
        message.into(),
    )
}