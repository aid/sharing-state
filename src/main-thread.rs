use std::sync::{Arc, RwLock};

// Define the shared state
struct SharedState {
    system_alive: bool,
    timestamp: std::time::SystemTime,
}

fn query_system_alive() -> bool {
    rand::random::<bool>()
}

fn main() {
    // Create the shared state wrapped in an Arc and RwLock
    let shared_state = Arc::new(RwLock::new(SharedState {
        system_alive: true,
        timestamp: std::time::SystemTime::now(),
    }));

    // Spawn one writer task
    let writer_state = shared_state.clone();

    // Note that std::thread uses closures (||) whilst tokio
    // just uses plain blocks ({}).
    std::thread::spawn(move || {
        loop {
            // Define a constrained scope to hold the lock whilst we make changes to the state
            // If we keep the lock in the same scope ({}) as the wait, this thread
            // effectively maintains the lock continually preventing the readers
            // from accessing the lock.
            {
                let mut state = writer_state.write().unwrap();
                state.system_alive = query_system_alive();
                state.timestamp = std::time::SystemTime::now();
            }
            std::thread::sleep(std::time::Duration::from_secs(10));
        }
    });

    // Spawn two reader tasks
    for _ in 0..2 {
        let reader_state = shared_state.clone();
        std::thread::spawn(move || {
            loop {
                // Define a constrained scope to hold the lock whilst read the state
                // If we keep the lock in the same scope ({}) as the wait, this thread
                // effectively maintains the lock continually preventing the writer
                // from accessing the lock. (Although multiple readers could have the
                // lock simutaneously.)
                {
                    let state = reader_state.read().unwrap();
                    println!(
                        "Thread system alive: {}, Timestamp: {:?}",
                        state.system_alive, state.timestamp
                    );
                }
                std::thread::sleep(std::time::Duration::from_secs(1));
            }
        });
    }

    // We pause the main thread for 30s so we can see the above threads in
    // action.  We need this else the program will exit immediately.  Nor
    // can we wait on the above threads as they continue infinitely.
    std::thread::sleep(std::time::Duration::from_secs(30));
}
