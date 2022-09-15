// File exporter
#![forbid(unsafe_code)]
#![forbid(missing_docs)]
use crate::errors::ExporterError;
use crate::exporter::Exporter;
use log::debug;
use std::fmt;
use std::io::{
    self,
    Write,
};
use std::path::PathBuf;
use tempfile::NamedTempFile;

#[derive(Clone, Debug)]
pub enum FileExporterOutput {
    File(PathBuf),
    Stdout,
}

impl fmt::Display for FileExporterOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::File(path) => {
                let path = path.to_str().expect("path to str");
                write!(f, "{}", path)
            },
            Self::Stdout => write!(f, "-"),
        }
    }
}

pub struct FileExporter {
    dest: FileExporterOutput,
}

impl FileExporter {
    pub fn new(output: FileExporterOutput) -> Self {
        match &output {
            FileExporterOutput::File(path) => {
                // This was already checked during command line parsing.
                let path = path.to_str().expect("path to str");
                debug!("New FileExporter outputting to {}", path);
            },
            FileExporterOutput::Stdout => {
                debug!("New FileExporter outputting to stdout");
            },
        };

        Self {
            dest: output,
        }
    }

    // Handles choosing the correct output type based on path
    fn write(&self, metrics: Vec<u8>) -> Result<(), ExporterError> {
        match &self.dest {
            FileExporterOutput::Stdout => {
                debug!("Writing metrics to stdout");

                io::stdout().write_all(&metrics)?;
            },
            FileExporterOutput::File(path) => {
                debug!("Writing metrics to {:?}", path);

                // We already vetted the parent in the CLI validator, so unwrap
                // here should be fine.
                let parent = path.parent().expect("path to have a parent");

                // We do this since we need the temporary file to be on the
                // same filesystem as the final persisted file.
                let mut file = NamedTempFile::new_in(&parent)?;
                let metrics = String::from_utf8(metrics)?;
                write!(file, "{}", metrics)?;
                file.persist(&path)?;
            },
        }

        Ok(())
    }

    pub fn export(self) -> Result<(), ExporterError> {
        debug!("Exporting metrics to file");

        // Get an exporter and export the metrics.
        let exporter = Exporter::new();
        let metrics  = exporter.export()?;

        // Write metrics
        self.write(metrics)?;

        Ok(())
    }
}
