use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use signal::Trap; // Import the signal trap

fn main() {
    // Create a channel to communicate between threads
    let (tx, rx) = mpsc::channel();

    // Set up the signal handler
    let trap = Trap::trap();

    // Start a new thread that will wait for the signal
    thread::spawn(move || {
        for _ in trap {
            // Send a message to the main thread when SIGINT is received
            tx.send(()).unwrap();
        }
    });

    println!("Press Ctrl+C to exit...");

    // Main loop
    loop {
        // Do some work
        println!("Working...");
        thread::sleep(Duration::from_secs(1));

        // Check if we received a signal
        if let Ok(_) = rx.try_recv() {
            println!("Keyboard interrupt received. Exiting gracefully...");
            break;
        }
    }

    println!("Exited cleanly.");
}
