use std::{path::PathBuf, time::Duration};
use tokio::{fs::File, io::AsyncReadExt, sync::watch, time::sleep};

pub async fn read_file(filepath: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(filepath).await?;
    let mut buf_str = String::new();
    file.read_to_string(&mut buf_str).await?;

    Ok(buf_str)
}

pub async fn watch_for_file_changes(tx: watch::Sender<bool>) {
    // Get file path
    let path_buf = PathBuf::from("file.txt");

    // Last modified file value
    let mut file_last_modified = None;

    // Infinite loop allows us to extract file time when the file was modified
    loop {
        // Gef file metadata
        let Ok(metadata) = path_buf.metadata() else {
            return;
        };

        // Get file modified data and compare against the previous/stored value
        if let Ok(modified) = metadata.modified() {
            let new_modified = Some(modified);
            if file_last_modified != new_modified {
                file_last_modified = new_modified;
                let _ = tx.send(true);
            }
        }

        // Sleep for 100 milliseconds and try
        sleep(Duration::from_millis(100)).await;
    }
}
