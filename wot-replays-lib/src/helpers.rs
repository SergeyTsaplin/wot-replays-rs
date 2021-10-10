use crate::models;
use std::convert::TryFrom;
use std::fs::File;

pub fn read_raw(
    mut stream: &mut impl std::io::Read,
    data_only: bool,
) -> Result<models::RawReplay, Box<dyn std::error::Error>> {
    if data_only {
        models::RawReplay::read_data_only(&mut stream)
    } else {
        models::RawReplay::read(&mut stream)
    }
}

pub fn read_raw_from_file(
    file_name: &str,
    data_only: bool,
) -> Result<models::RawReplay, Box<dyn std::error::Error>> {
    let mut stream = File::open(file_name)?;
    read_raw(&mut stream, data_only)
}

pub fn read_and_parse(
    mut stream: &mut impl std::io::Read,
) -> Result<models::Replay, Box<dyn std::error::Error>> {
    let raw_replay = read_raw(&mut stream, true)?;
    Ok(models::Replay::try_from(&raw_replay)?)
}

pub fn read_and_parse_from_file(
    file_name: &str,
) -> Result<models::Replay, Box<dyn std::error::Error>> {
    let mut stream = File::open(file_name)?;
    read_and_parse(&mut stream)
}
