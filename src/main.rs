#![windows_subsystem = "windows"]

use std::env;
use std::process;

use nokhwa::Camera;
use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::*;

const DEBUG: bool = false;

const EXIT_BASE: i32 = 100;

fn main() {
    let args = env::args()
        .skip(1)
        .collect::<Vec<String>>();

    if args.len() < 1 {
        debug_print("[!] you need to specify an output path");
        process::exit(EXIT_BASE + 1);
    }

    let output_path = &args[0];
    debug_print(format!("[*] output path: {}", output_path));

    let index = CameraIndex::Index(0);
    let requested = RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);

    let mut camera = match Camera::new(index, requested) {
        Ok(cam) => cam,
        Err(err) => {
            debug_print(format!("[!] failed to open camera: {}", err));
            process::exit(EXIT_BASE + 2);
        }
    };

    camera.open_stream().unwrap_or_else(|err| {
        debug_print(format!("[!] failed to open stream: {}", err));
        process::exit(EXIT_BASE + 3);
    });

    let frame = camera.frame().unwrap_or_else(|err| {
        debug_print(format!("[!] failed to capture: {}", err));
        process::exit(EXIT_BASE + 4);
    });

    let image = frame.decode_image::<RgbFormat>().unwrap_or_else(|err| {
        debug_print(format!("[!] failed to decode: {}", err));
        process::exit(EXIT_BASE + 5);
    });

    image.save(output_path).unwrap_or_else(|err| {
        debug_print(format!("[!] failed to save: {}", err));
        process::exit(EXIT_BASE + 6);
    });

    debug_print(format!("[*] saved to: {}", output_path));
    process::exit(0);
}

fn debug_print(log: impl AsRef<str>) {
    if !DEBUG {
        return;
    }

    println!("{}", log.as_ref());
}
