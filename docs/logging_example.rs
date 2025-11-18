// Simple log rotation example (fake example for testing RAG system)

fn rotate_logs() {
    let max_size = 10 * 1024 * 1024; // 10MB
    let path = "app.log";

    if let Ok(metadata) = std::fs::metadata(path) {
        if metadata.len() > max_size {
            let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
            let new_name = format!("app_{}.log", timestamp);

            // Rename old log
            let _ = std::fs::rename(path, &new_name);

            // Create new log file
            let _ = std::fs::File::create(path);
        }
    }
}

fn main() {
    rotate_logs();
}
