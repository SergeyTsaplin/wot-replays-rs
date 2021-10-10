use std::error::Error;

enum Subcommands {
    Parse,
}

impl std::str::FromStr for Subcommands {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "parse" => Ok(Subcommands::Parse),
            _ => Err("Unknow subcommand"),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let version_description = format!(
        "{} (wot-replays-rs {})",
        clap::crate_version!(),
        wot_replays::get_version()
    );
    let run_app = clap::App::new("parse")
        .about("Parses wot replay file")
        .arg(clap::Arg::with_name("replay_path").index(1).required(true))
        .arg(
            clap::Arg::with_name("battle_info_only")
                .short("-i")
                .long("--battle-info-only")
                .conflicts_with("results_only"),
        )
        .arg(
            clap::Arg::with_name("results_only")
                .short("-r")
                .long("--results-only")
                .conflicts_with("battle_info_only"),
        );
    let mut clap_app = clap::App::new("wot-replay")
        .subcommand(run_app)
        .version(&version_description[..]);
    let clone = clap_app.clone();
    let matches = clone.get_matches();
    match matches.subcommand_name() {
        Some("parse") => handle_parse(matches.subcommand_matches("parse").unwrap()),
        _ => {
            clap_app.print_help()?;
            Ok(())
        }
    }
}

fn handle_parse(matches: &clap::ArgMatches) -> Result<(), Box<dyn Error>> {
    if let Some(replay_path) = matches.value_of("replay_path") {
        let replay = wot_replays::read_raw_from_file(&replay_path, true)?;
        print_raw_metadata(
            replay,
            matches.is_present("battle_info_only"),
            matches.is_present("results_only"),
        )
    } else {
        Ok(())
    }
}

fn print_raw_metadata(
    replay: wot_replays::models::RawReplay,
    battle_info_only: bool,
    results_only: bool,
) -> Result<(), Box<dyn Error>> {
    if !battle_info_only && !results_only {
        for i in 0..replay.data_chunks {
            println!("{}", std::str::from_utf8(&replay.data[i as usize].payload)?);
        }
        Ok(())
    } else if battle_info_only {
        println!("{}", std::str::from_utf8(&replay.data[0].payload)?);
        Ok(())
    } else if replay.data_chunks >= 2 {
        println!("{}", std::str::from_utf8(&replay.data[1].payload)?);
        Ok(())
    } else {
        Err("Replay doesn't conatain battle results".into())
    }
}
