# Parsing library for World Of Tanks replay files

The crate allows to load a metadata from `.wotreplay` files.

## Usage examples

```rust
use wot_replays;

fn main() {
    let replay = wot_replays::read_and_parse_from_file(
        "test_data/20210412_2144_ussr-R158_LT_432_01_karelia.wotreplay",
    ).unwrap();

    println!("Server: {}", &replay.battle_info.server_name);
    println!("Map: {}", &replay.battle_info.map_display_name);
    println!("Battle started at: {}", &replay.battle_info.date_time);
}
```

For more examples, see [examples](examples/) directory.