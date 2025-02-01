use std::process::Command;

fn main() {
    // Build frontend
    Command::new("cmd")
        .args(&["/C", "cd frontend && pnpm run build"])
        .status()
        .expect("Failed to build frontend");
}
