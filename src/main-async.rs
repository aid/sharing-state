use std::sync::Arc;

// Define the shared state
struct SharedState {
    system_alive: bool,
    timestamp: std::time::SystemTime,
}

fn query_system_alive() -> bool {
    rand::random::<bool>()
}

/// We need to ensure the main() function is both marked as tokio::main
/// and that it is defined as an async function.
#[tokio::main]
async fn main() {
    // Create the shared state wrapped in an Arc and RwLock
    let shared_state = Arc::new(tokio::sync::RwLock::new(SharedState {
        system_alive: true,
        timestamp: std::time::SystemTime::now(),
    }));

    // Spawn one writer task
    let writer_state = shared_state.clone();
    tokio::spawn(async move {
        loop {
            // Define a constrained scope to hold the lock whilst we make changes to the state
            // If we keep the lock in the same scope ({}) as the wait, this thread
            // effectively maintains the lock continually preventing the readers
            // from accessing the lock.
            {
                // In tokio async world, we need to await this RwLock;
                // it doesn't return a Result so we don't need to worry about
                // unwrap() or similar.
                let mut state = writer_state.write().await;
                state.system_alive = query_system_alive();
                state.timestamp = std::time::SystemTime::now();
            }
            tokio::time::sleep(std::time::Duration::from_secs(10)).await;
        }
    });

    // Spawn two reader tasks
    for _ in 0..2 {
        let reader_state = shared_state.clone();
        tokio::spawn(async move {
            loop {
                // Define a constrained scope to hold the lock whilst read the state
                // If we keep the lock in the same scope ({}) as the wait, this thread
                // effectively maintains the lock continually preventing the writer
                // from accessing the lock. (Although multiple readers could have the
                // lock simutaneously.)
                {
                    // In tokio async world, we need to await this RwLock;
                    // it doesn't return a Result so we don't need to worry about
                    // unwrap() or similar.
                    let state = reader_state.read().await;
                    println!(
                        "Async system alive: {}, Timestamp: {:?}",
                        state.system_alive, state.timestamp
                    );
                }
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        });
    }
    // We pause the main thread for 30s so we can see the above threads in
    // action.  We need this else the program will exit immediately.  Nor
    // can we wait on the above threads as they continue infinitely.
    tokio::time::sleep(std::time::Duration::from_secs(30)).await;
}
