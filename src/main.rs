use clap::{Arg, Command};
use emulator::Emulator;

mod chip8;
mod constants;
mod emulator;
mod keys;
mod media;

fn main() {
    let scale_param_help = format!(
        "Positive integer for screen scale, default {} for resolution {} x {}",
        constants::DEF_SCALE,
        constants::SCREEN_WIDTH * constants::DEF_SCALE as usize,
        constants::SCREEN_HEIGHT * constants::DEF_SCALE as usize
    );
    let ips_param_help = format!(
        "Number of instructions per second for emulation, default, {}",
        constants::EMULATION_IPS
    );
    let matches = Command::new("CHIP-8")
        .version("0.1")
        .author("Marin-Georign Badita")
        .about("Simple Chip-8 emulator")
        .arg(
            Arg::new("rom-path")
                .required(true)
                .index(1)
                .help("Path to ROM file"),
        )
        .arg(
            Arg::new("scale")
                .required(false)
                .short('s')
                .takes_value(true)
                .help(scale_param_help.as_str()),
        )
        .arg(
            Arg::new("emulation-ips")
                .required(false)
                .short('i')
                .takes_value(true)
                .help(ips_param_help.as_str()),
        )
        .arg(
            Arg::new("debug")
                .required(false)
                .short('d')
                .takes_value(false)
                .help("Whether the emulator is run in debug mode"),
        )
        .get_matches();

    let rom_path = matches.value_of("rom-path").unwrap();
    let scale = if matches.is_present("scale") {
        matches
            .value_of("scale")
            .unwrap_or_default()
            .parse::<u32>()
            .unwrap_or(constants::DEF_SCALE)
    } else {
        constants::DEF_SCALE
    };
    let emulation_ips = if matches.is_present("emulation-ips") {
        matches
            .value_of("emulation-ips")
            .unwrap_or_default()
            .parse::<u128>()
            .unwrap_or(constants::EMULATION_IPS)
    } else {
        constants::EMULATION_IPS
    };
    let debug = matches.occurrences_of("debug") > 0;

    let mut emulator = Emulator::new("CHIP-8 Emulation", rom_path, scale, emulation_ips, debug);
    emulator.emulate();
}
