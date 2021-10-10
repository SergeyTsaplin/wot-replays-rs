use serde_repr::*;

/// Contains possible arena finish reasons
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(i8)]
pub enum FinishReason {
    Unknow = 0,
    AllVehicleDestroyed = 1,
    BaseCaptured = 2,
    TimeOut = 3,
    ArenaFailure = 4,
    Technical = 5,
}

/// Contains possible vehicle death reasons
#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
#[repr(i8)]
pub enum DeathReason {
    Alive = -1,
    Shot = 0,
    Fire = 1,
    Rammin = 2,
    WorldCollision = 3,
    DeathZone = 4,
    Drowning = 5,
    GasAttack = 6,
    Overturn = 7,
    Manual = 8,
    ArtillerProtection = 9,
    ArtilerySector = 10,
    Bombers = 11,
    Recovery = 12,
    ArtilleryEq = 13,
    BomberEq = 14,
    None = 15,
}

/// Contains mastery level badges
pub enum MasterLevel {
    Third = 1,
    Second = 2,
    First = 3,
    Master = 4,
    None = 0,
}

/// Contains premium accounts values
pub enum PremType {
    Basic = 1,
    Plus = 2,
    Vip = 4,
    None = 0,
}

pub enum BattleHeroMedal {
    Invader = 35,
    Sniper = 36,
    Sniper2 = 227,
    MainGun = 228,
    Defender = 37,
    Steelwall = 38,
    Supporter = 39,
    Scout = 40,
    Evileye = 72,
}

pub enum EpicMedal {
    MedalRadleyWalters = 73,
    MedalLafayettePool = 74,
    HeroesOfRassenay = 110,
    MedalBillotte = 54,
    MedalBrunoPietro = 75,
    MedalTarczay = 76,
    MedalBurda = 53,
    MedalPascucci = 77,
    MedalDumitru = 78,
    MedalOskin = 51,
    MedalHalonen = 52,
    MedalKolobanov = 55,
    MedalFadin = 56,
    MedalDeLanglade = 145,
    MedalGore = 298,
    Huntsman = 148,
    MedalTamadaYoshio = 146,
    MedalStark = 300,
}

pub enum DossierAchivements {
    FragBeast = 14,
    FragsSinai = 108,
    FragsPatton = 153,
    Warrior = 34,
    Invader = 35,
    Sniper = 36,
    Sniper2 = 227,
    MainGun = 228,
    Defender = 37,
    Steelwall = 38,
    Supporter = 39,
    Scout = 40,
    Evileye = 72,
    BattleHeroes = 10,
    SniperSiries = 23,
    MaxSniperSiries = 24,
    InvincibleSeries = 25,
    MaxInvincibleSeries = 26,
    DiehardSeries = 27,
    MaxDiehardSeries = 28,
}
