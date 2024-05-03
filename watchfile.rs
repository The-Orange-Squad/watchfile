use std::fs::{metadata, FileTime};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;

struct FileTracker {
    filepath: String,
    check_interval: u64,
    stop_track: AtomicBool,
    last_mod_time: FileTime,
}

impl FileTracker {
    fn new(filepath: String, check_interval: u64) -> Self {
        let last_mod_time = metadata(filepath)
            .expect("Failed to get file metadata")
            .modified()
            .expect("Failed to get file modification time");

        Self {
            filepath,
            check_interval,
            stop_track: AtomicBool::new(false),
            last_mod_time,
        }
    }

    async fn track(&self) -> bool {
        loop {
            thread::sleep(Duration::from_secs(self.check_interval));

            let new_mod_time = metadata(self.filepath.clone())
                .expect("Failed to get file metadata")
                .modified()
                .expect("Failed to get file modification time");

            if new_mod_time != self.last_mod_time {
                self.last_mod_time = new_mod_time;
                return true;
            }

            if self.stop_track.load(Ordering::Relaxed) {
                break;
            }
        }

        false
    }

    fn cancel(&self) {
        self.stop_track.store(true, Ordering::Relaxed);
    }
}
