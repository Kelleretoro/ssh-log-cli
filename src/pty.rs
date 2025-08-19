use crate::data::{DataDecoder, DataSource};
use crate::metadata::Metadata;
use std::io::{Read, Write};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PTYParserError {
    #[error("read error")]
    ReadError(std::io::Error),
    #[error("write error")]
    WriteError(std::io::Error),
    #[error("data error")]
    DataError(crate::data::DataError),
}

pub fn generate_replay<R: Read, W1: Write, W2: Write>(
    _metadata: &Metadata,
    mut decoder: DataDecoder<R>,
    mut data_writer: W1,
    mut times_writer: W2,
) -> Result<(), PTYParserError> {
    let mut last_time = std::time::Duration::from_secs(0);

    loop {
        let data_packet = match decoder.next().map_err(PTYParserError::DataError)? {
            Some(packet) => packet,
            None => break,
        };

        // Only process origin (server) data for PTY replay
        if matches!(data_packet.source, DataSource::Origin) {
            // Calculate time delta since last packet
            let time_delta = data_packet.elapsed.saturating_sub(last_time);
            last_time = data_packet.elapsed;

            // Write timing data in scriptreplay format (seconds.microseconds data_length)
            writeln!(
                times_writer,
                "{:.6} {}",
                time_delta.as_secs_f64(),
                data_packet.data.len()
            )
            .map_err(|e| PTYParserError::WriteError(e))?;

            // Write the terminal data
            data_writer
                .write_all(&data_packet.data)
                .map_err(|e| PTYParserError::WriteError(e))?;
        }
    }

    Ok(())
}
