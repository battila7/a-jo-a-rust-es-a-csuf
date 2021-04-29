extern crate futures; // 0.3.14
extern crate thiserror; // 1.0.24
extern crate tokio; // 1.5.0

use thiserror::Error;

#[derive(Default)]
struct Greeting {
    hello: String
}

#[derive(Debug, Error)]
enum FileSystemError {
    #[error("Hili, hali, halihó! Nincs meg a fájl :(")]
    FileNotFound
}

async fn show_off_select() {
    tokio::select! {
        _ = futures::future::pending() => {
            // Csinálunk valamit, 
        },
        _ = futures::future::pending() => {
            // És itt is csinálunk valamit.
        },
    }
}
