use std::env;
use std::fs;

use image::ImageFormat;
use image::io::Reader as ImageReader;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|a| a == "--help" || a == "-h") {
        eprintln!(
            "Usage: all_to_png [--dry-run|-n]\n  --dry-run, -n   Show what would be converted without writing files"
        );
        return;
    }
    let dry_run = args.iter().any(|a| a == "--dry-run" || a == "-n");

    let mut converted = 0usize;
    let mut skipped_png = 0usize;

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
            if ext.eq_ignore_ascii_case("png") {
                skipped_png += 1;
                continue;
            }
        }

        // Try to open and decode the file as an image
        match ImageReader::open(&path) {
            Ok(reader) => match reader.with_guessed_format() {
                Ok(reader2) => match reader2.decode() {
                    Ok(img) => {
                        let out_path = path.with_extension("png");
                        if dry_run {
                            println!(
                                "Would convert: {} -> {}",
                                path.display(),
                                out_path.display()
                            );
                            converted += 1;
                        } else {
                            match img.save_with_format(&out_path, ImageFormat::Png) {
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
        "Summary: converted {}, skipped existing png {}.",
        converted, skipped_png
    );
}
