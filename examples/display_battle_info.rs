use wot_replays;

fn display_battle_info(battle_info: &wot_replays::models::BattleInfo) {
    println!("Battle info:");
    println!("Server: {}", battle_info.server_name);
    println!(
        "Map: {} ({})",
        battle_info.map_display_name, battle_info.map_name
    );
    println!("Batle started: {}", battle_info.date_time);
}

fn display_teams(battle_info: &wot_replays::models::BattleInfo) {
    println!("First team:");
    let current_player = &battle_info.player_name;
    for (_, vehicle) in &battle_info.vehicles {
        if vehicle.team == 1 {
            let sign = if current_player == &vehicle.name {
                "*"
            } else {
                " "
            };
            println!("{} {}\t{}", sign, vehicle.fake_name, vehicle.vehicle_type);
        }
    }

    println!("Second team:");
    for (_, vehicle) in &battle_info.vehicles {
        if vehicle.team == 2 {
            let sign = if current_player == &vehicle.name {
                "*"
            } else {
                " "
            };
            println!("{} {}\t{}", sign, vehicle.fake_name, vehicle.vehicle_type);
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let replay = wot_replays::read_and_parse_from_file(
        "test_data/20210412_2144_ussr-R158_LT_432_01_karelia.wotreplay",
    )?;

    display_battle_info(&replay.battle_info);
    display_teams(&replay.battle_info);
    Ok(())
}
