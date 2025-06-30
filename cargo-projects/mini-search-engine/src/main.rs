use crate::index::file_indexing;
use crate::merger::merge_indices;
use crate::reader::file_reader;
use crate::writer::write_indices;
use std::fs::read_dir;
use std::io;
use std::path::PathBuf;

mod content;
mod filename;
mod index;
mod merger;
mod reader;
mod writer;

fn execute(files: Vec<PathBuf>) -> io::Result<()> {
    let (reader, reader_join) = file_reader(files);
    let (indexer, indexer_join) = file_indexing(reader);
    let (merger, merger_join) = merge_indices(indexer);
    let (writer, writer_join) = write_indices(merger);

    let reader_1 = reader_join.join().unwrap();
    indexer_join.join().unwrap();
    merger_join.join().unwrap();
    writer_join.join().unwrap();

    reader_1?;

    Ok(())
}

fn main() {
    let files_in_dir: Vec<PathBuf> = read_dir("./files")
        .unwrap()
        .filter_map(|dir| dir.ok())
        .map(|entry| entry.path())
        .collect();

    let _ = execute(files_in_dir);
}
