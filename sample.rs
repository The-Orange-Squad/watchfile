use std::thread;

mod watchfile;

fn main() {
    let filepath = "smpl.txt";
    let check_interval = 1; // Check every second

    let tracker = watchfile::FileTracker::new(filepath.to_string(), check_interval);

    let track_future = tracker.track();

    // Run another thread to perform other tasks while the file is being tracked
    thread::spawn(move || {
        println!("Performing other tasks...");
    });

    if track_future.await {
        println!("File has been modified!");
    } else {
        println!("File tracking canceled.");
    }

    tracker.cancel();
}
