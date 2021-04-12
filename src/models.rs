use chrono::{DateTime, Utc};

use byteorder::{ByteOrder, LittleEndian};
use log::trace;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::TryFrom;
use std::str;

#[derive(Serialize, Deserialize, Debug)]
pub struct DataChunk {
    pub length: u32,
    pub payload: Vec<u8>,
}

impl DataChunk {
    pub fn read<T: std::io::Read>(stream: &mut T) -> Result<DataChunk, Box<dyn std::error::Error>> {
        let mut length_buf = [0; 4];
        stream.read_exact(&mut length_buf)?;
        let payload_length = LittleEndian::read_u32(&length_buf);
        let mut payload_buf = vec![0u8; payload_length as usize];
        stream.read_exact(&mut payload_buf)?;
        Ok(DataChunk {
            length: payload_length,
            payload: payload_buf,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RawReplay {
    /// First 4 bytes of the replay file. Should be always equeals to 288633362
    pub magic: u32,
    /// Second 4 bytes (5 - 8) of the file
    pub data_chunks: u32,
    /// Data chunks. The length of the vector should be equals to the `data_chunks` value
    pub data: Vec<DataChunk>,
    /// The reamin part of the replay (the in-battle binary data)
    pub replay: Vec<u8>,
}

impl RawReplay {
    /// Fully reads the replay stream including the binary replay data.
    pub fn read<T: std::io::Read>(
        mut stream: &mut T,
    ) -> Result<RawReplay, Box<dyn std::error::Error>> {
        let mut raw_replay = Self::read_data_only(&mut stream)?;
        let mut replay: Vec<u8> = Vec::new();
        stream.read_to_end(&mut replay)?;
        raw_replay.replay = replay;
        Ok(raw_replay)
    }

    /// Reads only the informational part of the replay. The reamin part will be ignored.
    /// It's usefull for less memory footprint.
    pub fn read_data_only<T: std::io::Read>(
        mut stream: &mut T,
    ) -> Result<RawReplay, Box<dyn std::error::Error>> {
        let mut magic_buf = [0; 4];
        trace!("reading magic");
        stream.read_exact(&mut magic_buf)?;
        if magic_buf != [0x12, 0x32, 0x34, 0x11] {
            return Err("Unknown file format".into());
        }
        let magic = LittleEndian::read_u32(&magic_buf);

        trace!("reading data_chunks");
        let mut data_chunks_buf = [0; 4];
        stream.read_exact(&mut data_chunks_buf)?;
        let data_chunks = LittleEndian::read_u32(&data_chunks_buf);

        trace!("Found {} data chunks", &data_chunks);
        let mut data = vec![];
        for i in 0..data_chunks {
            trace!("reading data chunk number {}", i);
            data.push(DataChunk::read(&mut stream)?);
        }
        Ok(RawReplay {
            magic,
            data_chunks,
            data,
            replay: vec![],
        })
    }
}

pub struct Replay {
    pub battle_info: BattleInfo,
    pub results: Option<BattleResults>,
}

impl TryFrom<&RawReplay> for Replay {
    type Error = &'static str;

    fn try_from(raw_replay: &RawReplay) -> Result<Self, Self::Error> {
        let raw_battle_info = if let Some(v) = &raw_replay.data.get(0) {
            &v.payload
        } else {
            trace!("No battle info data in the replay");
            return Err("No battle info data in the replay");
        };
        let battle_info: BattleInfo = match serde_json::from_slice(&raw_battle_info[..]) {
            Ok(v) => v,
            Err(e) => {
                trace!("Invalid json: {}", e);
                return Err("Invalid json");
            }
        };
        let results = if raw_replay.data_chunks > 1 {
            let raw_results = if let Some(v) = &raw_replay.data.get(1) {
                &v.payload
            } else {
                return Err("Invalid replay format: no battle results while there are more than 1 data chunks declared");
            };
            let results: BattleResults = match serde_json::from_slice(&raw_results[..]) {
                Ok(v) => v,
                Err(e) => {
                    println!("Invalid json: {}", e);
                    return Err("Invalid json");
                }
            };
            Some(results)
        } else {
            None
        };
        Ok(Self {
            battle_info,
            results,
        })
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VehicleInfo {
    pub wtr: u32,
    pub vehicle_type: String,
    pub is_alive: u8,

    #[serde(rename = "personalMissionIDs")]
    pub personal_missions_ids: Vec<u32>,
    pub forbid_in_battle_invitations: bool,
    pub fake_name: String,
    pub max_health: u32,
    pub igr_type: u32,
    pub clan_abbrev: String,
    pub ranked: Vec<u32>,
    pub is_team_killer: u8,
    pub team: u8,
    pub overridden_badge: u32,
    #[serde(rename = "avatarSessionID")]
    pub avatar_session_id: String,
    pub badges: Vec<Vec<u32>>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BattleInfo {
    pub player_vehicle: String,

    pub client_version_from_xml: String,
    pub client_version_from_exe: String,

    pub region_code: String,
    pub server_name: String,
    pub map_name: String,
    pub map_display_name: String,
    pub server_settings: serde_json::Value,
    #[serde(rename = "gameplayID")]
    pub gameplay_id: String,
    pub battle_type: u16,
    pub has_mods: bool,
    #[serde(with = "wot_date_format")]
    pub date_time: DateTime<Utc>,
    #[serde(rename = "playerID")]
    pub player_id: u64,
    pub player_name: String,

    pub vehicles: HashMap<String, VehicleInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AvatarResults {
    pub avatar_kills: u16,
    pub player_rank: u16,
    pub base_points_diff: u32,
    pub has_battle_pass: bool,
    pub avatar_damaged: u16,
    pub total_damaged: u16,
    pub avatar_damage_dealt: u32,
    pub sum_points: u32,
    pub fairplay_violations: Vec<i32>,
    pub badges: Vec<Vec<u16>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PersonalAvatarResults {
    pub base_points_diff: u32,
    pub avatar_damage_dealt: u32,
    pub bpcoin_replay: Option<u32>,
    pub credits_replay: Option<u32>,
    #[serde(rename = "freeXPReplay")]
    pub free_xp_replay: Option<u32>,
    pub sum_points: u32,
    pub fairplay_violations: Vec<i32>,
    pub event_bpcoin: u32,
    pub badges: Vec<Vec<u16>>,
    pub active_rents: HashMap<String, u32>,
    #[serde(rename = "eventFreeXP")]
    pub event_free_xp: u32,
    pub event_credits: u32,
    pub xp_replay: Option<u32>,
    pub crystal: u32,
    pub damage_event_list: Option<serde_json::Value>,
    pub eligible_for_crystal_rewards: bool,
    pub dog_tags: serde_json::Value,
    pub is_premature_leave: bool,
    pub squad_bonus_info: Option<serde_json::Value>,
    pub winner_if_draw: u8,
    #[serde(rename = "freeXP")]
    pub free_xp: u32,
    pub avatar_kills: u16,
    #[serde(rename = "eventTMenXP")]
    pub event_t_men_xp: u16,
    #[serde(rename = "recruitsIDs")]
    pub recruits_ids: Vec<serde_json::Value>,
    pub avatar_damage_event_list: Option<serde_json::Value>,
    #[serde(rename = "PM2Progress")]
    pub pm2_progress: serde_json::Value,
    pub has_battle_pass: bool,
    pub total_damaged: u16,
    pub gold_replay: Option<serde_json::Value>,
    pub event_crystal: u16,
    pub event_gold: u32,
    #[serde(rename = "tmenXPReplay")]
    pub tmen_xp_replay: Option<serde_json::Value>,
    pub event_coin_replai: Option<serde_json::Value>,
    pub quests_progress: serde_json::Value,
    #[serde(rename = "accountDBID")]
    pub account_db_id: u64,
    pub avatar_ammo: Vec<serde_json::Value>,
    #[serde(rename = "fareTeamXPPosition")]
    pub fare_team_xp_position: u16,
    #[serde(rename = "eventXP")]
    pub event_xp: u16,
    #[serde(rename = "fortClanDBIDs")]
    pub fort_clan_db_ids: Vec<serde_json::Value>,
    pub xp: u16,
    pub player_rank: u16,
    pub avatar_damaged: u16,
    #[serde(rename = "recruiterID")]
    pub recruiter_id: u64,
    pub progressive_reward: Option<serde_json::Value>,
    pub crystal_replay: Option<serde_json::Value>,
    pub rank_change: u32,
    pub team: u8,
    #[serde(rename = "clanDBID")]
    pub clan_db_id: Option<u64>,
    pub credits: i64,
    pub event_event_coin: u64,
    pub watched_battle_to_the_end: bool,
    #[serde(rename = "flXPReplay")]
    pub fl_xp_replay: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VehicleResult {
    pub spotted: u8,
    pub vehicle_num_captured: u16,
    pub damage_assisted_track: u32,
    pub xp_penalty: i32,
    pub direct_team_hits: u32,
    pub damage_received: u32,
    pub sniper_damage_dealt: u32,
    pub piercing_enemy_hits: u16,
    pub damage_assisted_radio: u32,
    pub mileage: u32,
    pub stun_duration: f32,
    pub piercings: u16,
    pub damage_blocked_by_armor: u32,
    pub xp: u32,
    pub dropped_capture_points: u16,
    #[serde(rename = "killerID")]
    pub killer_id: u64,
    #[serde(rename = "xp/other")]
    pub xp_other: u32,
    pub index: u32,
    pub direct_hits_received: u32,
    pub damage_received_from_invisibles: u32,
    pub explosion_hits_received: u32,
    #[serde(rename = "achievementXP")]
    pub achievement_xp: u32,
    pub death_reason: crate::dictionaries::DeathReason,
    pub capture_points: u32,
    pub num_recovered: u16,
    pub direct_enemy_hits: u32,
    pub max_health: u32,
    pub damage_event_list: Option<serde_json::Value>,
    pub health: i32,
    pub stop_respawn: bool,
    pub achievement_credits: u32,
    pub achievements: Vec<u16>,
    #[serde(rename = "xp/assist")]
    pub xp_assist: u32,
    pub shots: u32,
    pub kills: u16,
    pub death_count: u16,
    pub flag_capture: u32,
    pub damaged: u16,
    pub tdamage_dealt: u32,
    pub resource_absorbed: u32,
    pub credits: u32,
    #[serde(rename = "accountDBID")]
    pub account_db_id: u64,
    pub life_time: u64,
    pub no_damage_direct_hits_received: u16,
    pub num_defended: u32,
    pub stunned: u16,
    pub equipment_damage_dealt: u32,
    pub is_team_killer: bool,
    pub type_comp_descr: u32,
    pub solo_flag_capture: u32,
    pub destructibles_hits: u32,
    pub capturing_base: Option<serde_json::Value>,
    pub damage_assisted_stun: u32,
    pub rollouts_count: u32,
    pub tkills: u16,
    pub potential_damage_received: u32,
    pub damage_dealt: u32,
    pub destructibles_num_destroyed: u32,
    pub damage_assisted_smoke: u32,
    pub destructibles_damage_dealt: u32,
    pub flag_actions: [u32; 4],
    pub win_points: u32,
    pub explosion_hits: u32,
    pub team: u8,
    #[serde(rename = "xp/attack")]
    pub xp_attack: u32,
    pub tdestroyed_modules: u32,
    pub stun_num: u32,
    pub damage_assisted_inspire: u32,
    #[serde(rename = "achievementFreeXP")]
    pub achievement_free_xp: u32,
    pub direct_hits: u32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PersonalBattleResults {
    pub avatar: PersonalAvatarResults,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInfo {
    pub name: String,
    #[serde(rename = "prebattleID")]
    pub prebattle_id: u64,
    pub igr_type: u32,
    pub clan_abbrev: String,
    pub team: u8,
    #[serde(rename = "clanDBID")]
    pub clan_dbid: u64,
    pub real_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommonBattleInfo {
    pub division: Option<u32>,
    pub finish_reason: crate::dictionaries::FinishReason,
    pub gui_type: u32,
    pub common_num_defended: u32,
    pub common_num_captured: u32,
    pub common_num_started: u32,
    pub arena_create_time: u128,
    pub common_num_destroyed: u32,
    pub duration: u32,
    pub team_health: HashMap<String, u32>,
    #[serde(rename = "arenaTypeID")]
    pub arena_type_id: u32,
    pub gas_attack_winner_team: i32,
    pub winner_team: u16,
    pub veh_lock_mode: u16,
    pub bonus_type: u16,
    pub bots: serde_json::Value,
    pub account_comp_descr: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GeneralBattleResults {
    #[serde(rename = "arenaUniqueID")]
    pub arena_unique_id: u64,
    pub personal: PersonalBattleResults,
    pub vehicles: HashMap<String, Vec<VehicleResult>>,
    pub avatars: HashMap<String, AvatarResults>,
    pub players: HashMap<String, PlayerInfo>,
    pub common: CommonBattleInfo,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerResults {
    pub wtr: u32,
    pub vehicle_type: String,
    #[serde(deserialize_with = "serde_aux::deserialize_bool_from_anything")]
    pub is_alive: bool,
    #[serde(rename = "personalMissionIDs")]
    pub personal_mission_ids: Vec<u32>,
    pub personal_mission_info: HashMap<String, Vec<u32>>,
    pub forbid_in_battle_invitations: bool,
    pub fake_name: String,
    pub max_health: u32,
    pub igr_type: u32,
    pub clan_abbrev: String,
    pub ranked: Vec<u32>,
    pub is_team_killer: u8,
    pub team: u8,
    pub events: HashMap<String, serde_json::Value>,
    pub overridden_badge: u32,
    #[serde(rename = "avatarSessionID")]
    pub avatar_session_id: String,
    pub badges: Vec<Vec<u32>>,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerFrags {
    pub frags: u8,
}

pub type PlayersResults = HashMap<String, PlayerResults>;

pub type Frags = HashMap<String, PlayerFrags>;

pub type BattleResults = (GeneralBattleResults, PlayersResults, Frags);

mod wot_date_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%d.%m.%Y %H:%M:%S";

    // The signature of a serialize_with function must follow the pattern:
    //
    //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
    //    where
    //        S: Serializer
    //
    // although it may also be generic over the input types T.
    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    // The signature of a deserialize_with function must follow the pattern:
    //
    //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
    //    where
    //        D: Deserializer<'de>
    //
    // although it may also be generic over the output types T.
    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_battle_results_deserializing() {
        let json = fs::read_to_string("test_data/battle_results.json").unwrap();
        let _result: BattleResults = match serde_json::from_str(&json) {
            Ok(v) => v,
            Err(e) => {
                panic!("Invalid json: {}", e);
            }
        };
        assert!(true);
    }
}
