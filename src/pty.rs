use std::io::{Read, Write};
use std::time::Duration;
use thiserror::Error;

use crate::data::{DataDecoder, DataSource};
use crate::metadata::Metadata;

#[derive(Error, Debug)]
pub enum PTYParserError {
    #[error("data error")]
    DataError(crate::data::DataError),
    #[error("write error")]
    WriteError(std::io::Error),
    #[error("invalid PTY data")]
    InvalidPTYData,
}

pub fn generate_replay<R: Read, W1: Write, W2: Write>(
    _metadata: &Metadata,
    mut decoder: DataDecoder<R>,
    mut data_writer: W1,
    mut times_writer: W2,
) -> Result<(), PTYParserError> {
    let mut start_time: Option<Duration> = None;

    loop {
        let packet = match decoder.next().map_err(PTYParserError::DataError)? {
            Some(packet) => packet,
            None => break,
        };

        // Initialize start time with first packet
        if start_time.is_none() {
            start_time = Some(packet.elapsed);
        }

        let elapsed_since_start = packet
            .elapsed
            .checked_sub(start_time.unwrap())
            .unwrap_or(Duration::ZERO);

        // Only process client data for PTY replay
        if matches!(packet.source, DataSource::Client) {
            // Write data to data file
            data_writer
                .write_all(&packet.data)
                .map_err(PTYParserError::WriteError)?;

            // Write timing information (format: seconds.microseconds data_length)
            let seconds = elapsed_since_start.as_secs();
            let micros = elapsed_since_start.subsec_micros();
            let timing_line = format!("{}.{:06} {}\n", seconds, micros, packet.data.len());
            times_writer
                .write_all(timing_line.as_bytes())
                .map_err(PTYParserError::WriteError)?;
        }
    }

    Ok(())
}
