use std::env;
use std::fs;
use std::thread;

use image::ImageFormat;
use image::io::Reader as ImageReader;

fn save_help_file() {
    let help = include_str!("../README.md");
    fs::write("./help.txt", help).expect("Failed to write help file");
}

fn main() {
    let _exit_delay = ExitIn3Seconds;
    // get the name of the executable and provide a runtime "is_executable" check
    let current_exe_name: Option<String> = env::current_exe()
        .ok()
        .and_then(|p| p.file_name().and_then(|s| s.to_str().map(|s| s.to_owned())));
    eprintln!("Current exe name: {:?}", current_exe_name);
    // Default target is PNG; override if executable name contains a format hint
    let mut target_ext = "png";
    let mut target_format = ImageFormat::Png;

    if let Some(ref name) = current_exe_name {
        let lower = name.to_lowercase();
        if lower.contains("help") || lower.contains("h") {
            save_help_file();
            println!("Help file 'help.txt' has been created in the current directory.");
            return;
        }

        if lower.contains("gif") {
            target_ext = "gif";
            target_format = ImageFormat::Gif;
        } else if lower.contains("jpeg") || lower.contains("jpg") {
            if lower.contains("jpg") {
                target_ext = "jpg";
            } else {
                target_ext = "jpeg";
            }

            target_format = ImageFormat::Jpeg;
        } else if lower.contains("png") {
            target_ext = "png";
            target_format = ImageFormat::Png;
        } else {
            save_help_file();
            println!("Help file 'help.txt' has been created in the current directory.");
            return;
        }
    }
    println!("Convert to {}", target_ext);

    let args: Vec<String> = env::args().collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "Usage: **exec** [--dry-run|-n]\n  --dry-run, -n   Show what would be converted without writing files"
        );
        return;
    }
    let dry_run = args.iter().any(|a| a == "--dry-run" || a == "-n");

    let mut converted = 0usize;
    let mut skipped_file = 0usize;

    let dir = ".";
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(err) => {
            eprintln!("Failed to read current directory: {}", err);
            return;
        }
    };

    for entry in entries {
        let entry = match entry {
            Ok(en) => en,
            Err(err) => {
                eprintln!("Failed to iterate entry: {}", err);
                continue;
            }
        };

        let path = entry.path();
        if !path.is_file() {
            continue;
        }

        if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
            if ext.eq_ignore_ascii_case(target_ext) {
                skipped_file += 1;
                continue;
            }
        }

        // Try to open and decode the file as an image
        match ImageReader::open(&path) {
            Ok(reader) => match reader.with_guessed_format() {
                Ok(reader2) => match reader2.decode() {
                    Ok(img) => {
                        let out_path = path.with_extension(target_ext);
                        if dry_run {
                            println!(
                                "Would convert: {} -> {}",
                                path.display(),
                                out_path.display()
                            );
                            converted += 1;
                        } else {
                            match img.save_with_format(&out_path, target_format) {
                                Ok(_) => {
                                    println!(
                                        "Converted: {} -> {}",
                                        path.display(),
                                        out_path.display()
                                    );
                                    converted += 1;
                                }
                                Err(err) => {
                                    eprintln!("Failed to save {}: {}", out_path.display(), err)
                                }
                            }
                        }
                    }
                    Err(_err) => {
                        // pass
                        // eprintln!("Failed to decode {}: {}", path.display(), err);
                    }
                },
                Err(_err) => {
                    // pass
                    // eprintln!("Failed to guess format for {}: {}", path.display(), err);
                }
            },
            Err(err) => eprintln!("Failed to open {}: {}", path.display(), err),
        }
    }

    println!(
        "Summary: converted {}, skipped existing {} {}.",
        converted, target_ext, skipped_file
    );
}

struct ExitIn3Seconds;
impl Drop for ExitIn3Seconds {
    fn drop(&mut self) {
        println!("Exiting in 3 seconds...");

        thread::sleep(std::time::Duration::from_secs(3));
    }
}
