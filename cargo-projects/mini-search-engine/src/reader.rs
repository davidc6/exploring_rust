use std::{
    borrow::Cow,
    fs::read_to_string,
    path::PathBuf,
    sync::mpsc::{channel, Receiver},
    thread::{self, JoinHandle},
};

use crate::{content::Content, filename::Filename};

/// Attempts to read all the files in a given directory and
/// send the content of each file on the channel.
///
/// Exits if a file cannot be read. If all operations are successful,
/// returns a tuple - a Receiver and JoinHandle.
pub fn file_reader(
    files: Vec<PathBuf>,
) -> (
    Receiver<(Filename, Content)>,
    JoinHandle<std::io::Result<()>>,
) {
    let (tx, rx) = channel();

    let handle = thread::spawn(move || {
        for file in files {
            let filename = if let Some(filename) = file.file_name() {
                filename.to_string_lossy()
            } else {
                Cow::from("")
            }
            .into_owned();

            let content = (Filename(filename), Content(read_to_string(file)?));

            let Ok(()) = tx.send(content) else {
                break;
            };
        }

        Ok(())
    });

    (rx, handle)
}
