use std::process::Command;

fn main() {
    let status = Command::new("typeshare")
        .args(&[
            "./src/models/",
            "--lang=typescript",
            "--output-file",
            "./frontend/src/lib/types/generated.ts",
        ])
        .status()
        .expect("Failed to run typeshare");

    if !status.success() {
        std::process::exit(1);
    }
}
