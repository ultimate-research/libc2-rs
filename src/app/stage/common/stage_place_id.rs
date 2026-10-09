use core::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use strum::{EnumCount, EnumString};

/// A numeric umbrella identifier for a stage.
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(
    Debug, Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd, EnumCount, EnumString,
)]
#[repr(C)]
pub enum StagePlaceID {
    /// Battlefield
    #[default]
    BattleField,

    /// Big Battlefield
    BattleField_L,

    /// Final Destination
    End,

    /// Peach's Castle
    Mario_Castle64,

    /// Kongo Jungle
    DK_Jungle,

    /// Hyrule Castle
    Zelda_Hyrule,

    /// Super Happy Tree
    Yoshi_Story,

    /// Dream Land
    Kirby_Pupupu64,

    /// Saffron City
    Poke_Yamabuki,

    /// Mushroom Kingdom
    Mario_Past64,

    /// Princess Peach's Castle
    Mario_CastleDx,

    /// Rainbow Cruise
    Mario_Rainbow,

    /// Kongo Falls
    DK_WaterFall,

    /// Jungle Japes
    DK_Lodge,

    /// Great Bay
    Zelda_Greatbay,

    /// Temple
    Zelda_Temple,

    /// Yoshi's Story
    Yoshi_CartBoard,

    /// Yoshi's Island (Melee)
    Yoshi_Yoster,

    /// Fountain of Dreams
    Kirby_Fountain,

    /// Green Greens
    Kirby_Greens,

    /// Corneria
    Fox_Corneria,

    /// Venom
    Fox_Venom,

    /// Brinstar
    Metroid_ZebesDx,

    /// Onett
    Mother_Onett,

    /// Pokémon Stadium
    Poke_Stadium,

    /// Brinstar Depths
    Metroid_Kraid,

    /// Fourside
    Mother_Fourside,

    /// Big Blue
    Fzero_Bigblue,

    /// Mushroom Kingdom II
    Mario_PastUsa,

    /// Delfino Plaza
    Mario_Dolpic,

    /// Yoshi's Island
    Yoshi_Island,

    /// Lylat Cruise
    Fox_LylatCruise,

    /// Bridge of Eldin
    Zelda_Oldin,

    /// Smashville
    Animal_Village,

    /// Skyworld
    Icarus_SkyWorld,

    /// Castle Siege
    FE_Siege,

    /// WarioWare, Inc.
    Wario_Madein,

    /// Pokémon Stadium 2
    Poke_Stadium2,

    /// Halberd
    Kirby_Halberd,

    /// Shadow Moses Island
    MG_Shadowmoses,

    /// New Pork City
    Mother_Newpork,

    /// Summit
    Ice_Top,

    /// Norfair
    Metroid_Norfair,

    /// Figure-8 Circuit
    Kart_CircuitX,

    /// Frigate Orpheon
    Metroid_Orpheon,

    /// Distant Planet
    Pikmin_Planet,

    /// Mushroomy Kingdom
    Mario_PastX,

    /// Port Town Aero Dive
    Fzero_Porttown,

    /// Luigi's Mansion
    LuigiMansion,

    /// Pirate Ship
    Zelda_Pirates,

    /// Spear Pillar
    Poke_Tengam,

    /// 75 m
    _75m,

    /// Mario Bros.
    MarioBros,

    /// Hanenbow
    Plankton,

    /// Green Hill Zone
    Sonic_Greenhill,

    /// 3D Land
    Mario_3DLand,

    /// Golden Plains
    Mario_NewBros2,

    /// Paper Mario
    Mario_Paper,

    /// Gerudo Valley
    Zelda_Gerudo,

    /// Spirit Train
    Zelda_Train,

    /// Unova Pokémon League
    Poke_Unova,

    /// Prism Tower
    Poke_Tower,

    /// Arena Ferox
    FE_Arena,

    /// Reset Bomb Forest
    Icarus_Uprising,

    /// Tortimer Island
    Animal_Island,

    /// Boxing Ring
    PunchOutSB,

    /// The original version of the Boxing Ring stage.
    PunchOutW,

    /// Gaur Plain
    Xeno_Gaur,

    /// Living Room
    Nintendogs,

    /// Find Mii
    StreetPass,

    /// Tomodachi Life
    Tomodachi,

    /// PictoChat 2
    Pictochat2,

    /// Wily Castle
    Rock_Wily,

    /// Magicant
    Mother_Magicant,

    /// Dream Land GB
    Kirby_Gameboy,

    /// Balloon Fight
    BalloonFight,

    /// Mute City SNES
    Fzero_Mutecity3DS,

    /// Mushroom Kingdom U
    Mario_Uworld,

    /// Mario Galaxy
    Mario_Galaxy,

    /// Mario Circuit
    Kart_CircuitFor,

    /// Skyloft
    Zelda_Skyward,

    /// The Great Cave Offensive
    Kirby_Cave,

    /// Kalos Pokémon League
    Poke_Kalos,

    /// Coliseum
    FE_Colloseum,

    /// Palutena's Temple
    Icarus_Angeland,

    /// Gamer
    Wario_Gamer,

    /// Garden of Hope
    Pikmin_Garden,

    /// Town and City
    Animal_City,

    /// Wii Fit Studio
    WiiFit,

    /// Wrecking Crew
    WreckingCrew,

    /// Pilotwings
    Pilotwings,

    /// Wuhu Island
    WufuIsland,

    /// Windy Hill Zone
    Sonic_Windyhill,

    /// PAC-LAND
    Pac_Land,

    /// Flat Zone X
    FlatZoneX,

    /// Duck Hunt
    DuckHunt,

    /// Suzaku Castle
    SF_Suzaku,

    /// Super Mario Maker
    Mario_Maker,

    /// Midgar
    FF_Midgar,

    /// Umbra Clock Tower
    Bayo_Clock,

    /// Moray Towers
    Spla_Parking,

    /// Dracula's Castle
    Dracula_Castle,

    /// Great Plateau Tower
    Zelda_Tower,

    /// New Donk City Hall
    Mario_Odyssey,

    /// Training
    Training,

    /// The stage where players can preview and test various settings.
    SettingStage,

    /// The stage where the results take place.
    ResultStage,

    /// The stage where players can practice before an online battle.
    ShamFight,

    /// The stage where defeated spirits from the Spirit Board are shot.
    SpiritsRoulette,

    /// The ad hoc Adventure map stage.
    CampaignMap,

    /// The stage where Bonus Game takes place.
    BonusGame,

    /// The stage where Home-Run Contest takes place.
    HomerunContest,

    /// The stage where the credits take place.
    Staffroll,

    /// The stage where Ganon, The Demon King is fought.
    BossStage_GanonBoss,

    /// The stage where Rathalos is fought.
    BossStage_Rathalos,

    /// The stage where Marx is fought.
    BossStage_Marx,

    /// The stage where Dracula is fought.
    BossStage_Dracula,

    /// The stage where Galleom is fought.
    BossStage_Galleom,

    /// The first stage of the final battle in Adventure.
    BossStage_Final1,

    /// The second stage of the final battle in Adventure.
    BossStage_Final2,

    /// The third stage of the final battle in Adventure.
    BossStage_Final3,

    /// The ad hoc Stage Builder stage.
    SP_Edit,

    /// The stage where Mii Fighter previews are rendered.
    PhotoStage,

    /// A stripped development stage.
    TestStage,

    /// Mementos
    Jack_Mementoes,

    /// The stage where the results take place when Joker is the winner.
    ResultStage_Jack,

    /// Yggdrasil's Altar
    Brave_Altar,

    /// Spiral Mountain
    Buddy_Spiral,

    /// King of Fighters Stadium
    Dolly_Stadium,

    /// Garreg Mach Monastary
    FE_Shrine,

    /// Spring Stadium
    Tantan_Spring,

    /// Small Battlefield
    BattleField_S,

    /// Minecraft World
    Pickel_World,

    /// Northern Cave
    FF_Cave,

    /// The stage where the results take place when Sephiroth is the winner.
    ResultStage_Edge,

    /// Cloud Sea of Alrest
    Xeno_Alst,

    /// Mishima Dojo
    Demon_Dojo,

    /// Hollow Bastion
    Trail_Castle,
}

impl StagePlaceID {
    /// The total number of identifiers.
    pub const COUNT: usize = <Self as EnumCount>::COUNT;
}

impl fmt::Display for StagePlaceID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;

        assert_eq!(size_of::<StagePlaceID>(), 0x4);
        assert_eq!(StagePlaceID::BattleField as i32, 0x0);
        assert_eq!(StagePlaceID::BattleField_L as i32, 0x1);
        assert_eq!(StagePlaceID::End as i32, 0x2);
        assert_eq!(StagePlaceID::Mario_Castle64 as i32, 0x3);
        assert_eq!(StagePlaceID::DK_Jungle as i32, 0x4);
        assert_eq!(StagePlaceID::Zelda_Hyrule as i32, 0x5);
        assert_eq!(StagePlaceID::Yoshi_Story as i32, 0x6);
        assert_eq!(StagePlaceID::Kirby_Pupupu64 as i32, 0x7);
        assert_eq!(StagePlaceID::Poke_Yamabuki as i32, 0x8);
        assert_eq!(StagePlaceID::Mario_Past64 as i32, 0x9);
        assert_eq!(StagePlaceID::Mario_CastleDx as i32, 0xA);
        assert_eq!(StagePlaceID::Mario_Rainbow as i32, 0xB);
        assert_eq!(StagePlaceID::DK_WaterFall as i32, 0xC);
        assert_eq!(StagePlaceID::DK_Lodge as i32, 0xD);
        assert_eq!(StagePlaceID::Zelda_Greatbay as i32, 0xE);
        assert_eq!(StagePlaceID::Zelda_Temple as i32, 0xF);
        assert_eq!(StagePlaceID::Yoshi_CartBoard as i32, 0x10);
        assert_eq!(StagePlaceID::Yoshi_Yoster as i32, 0x11);
        assert_eq!(StagePlaceID::Kirby_Fountain as i32, 0x12);
        assert_eq!(StagePlaceID::Kirby_Greens as i32, 0x13);
        assert_eq!(StagePlaceID::Fox_Corneria as i32, 0x14);
        assert_eq!(StagePlaceID::Fox_Venom as i32, 0x15);
        assert_eq!(StagePlaceID::Metroid_ZebesDx as i32, 0x16);
        assert_eq!(StagePlaceID::Mother_Onett as i32, 0x17);
        assert_eq!(StagePlaceID::Poke_Stadium as i32, 0x18);
        assert_eq!(StagePlaceID::Metroid_Kraid as i32, 0x19);
        assert_eq!(StagePlaceID::Mother_Fourside as i32, 0x1A);
        assert_eq!(StagePlaceID::Fzero_Bigblue as i32, 0x1B);
        assert_eq!(StagePlaceID::Mario_PastUsa as i32, 0x1C);
        assert_eq!(StagePlaceID::Mario_Dolpic as i32, 0x1D);
        assert_eq!(StagePlaceID::Yoshi_Island as i32, 0x1E);
        assert_eq!(StagePlaceID::Fox_LylatCruise as i32, 0x1F);
        assert_eq!(StagePlaceID::Zelda_Oldin as i32, 0x20);
        assert_eq!(StagePlaceID::Animal_Village as i32, 0x21);
        assert_eq!(StagePlaceID::Icarus_SkyWorld as i32, 0x22);
        assert_eq!(StagePlaceID::FE_Siege as i32, 0x23);
        assert_eq!(StagePlaceID::Wario_Madein as i32, 0x24);
        assert_eq!(StagePlaceID::Poke_Stadium2 as i32, 0x25);
        assert_eq!(StagePlaceID::Kirby_Halberd as i32, 0x26);
        assert_eq!(StagePlaceID::MG_Shadowmoses as i32, 0x27);
        assert_eq!(StagePlaceID::Mother_Newpork as i32, 0x28);
        assert_eq!(StagePlaceID::Ice_Top as i32, 0x29);
        assert_eq!(StagePlaceID::Metroid_Norfair as i32, 0x2A);
        assert_eq!(StagePlaceID::Kart_CircuitX as i32, 0x2B);
        assert_eq!(StagePlaceID::Metroid_Orpheon as i32, 0x2C);
        assert_eq!(StagePlaceID::Pikmin_Planet as i32, 0x2D);
        assert_eq!(StagePlaceID::Mario_PastX as i32, 0x2E);
        assert_eq!(StagePlaceID::Fzero_Porttown as i32, 0x2F);
        assert_eq!(StagePlaceID::LuigiMansion as i32, 0x30);
        assert_eq!(StagePlaceID::Zelda_Pirates as i32, 0x31);
        assert_eq!(StagePlaceID::Poke_Tengam as i32, 0x32);
        assert_eq!(StagePlaceID::_75m as i32, 0x33);
        assert_eq!(StagePlaceID::MarioBros as i32, 0x34);
        assert_eq!(StagePlaceID::Plankton as i32, 0x35);
        assert_eq!(StagePlaceID::Sonic_Greenhill as i32, 0x36);
        assert_eq!(StagePlaceID::Mario_3DLand as i32, 0x37);
        assert_eq!(StagePlaceID::Mario_NewBros2 as i32, 0x38);
        assert_eq!(StagePlaceID::Mario_Paper as i32, 0x39);
        assert_eq!(StagePlaceID::Zelda_Gerudo as i32, 0x3A);
        assert_eq!(StagePlaceID::Zelda_Train as i32, 0x3B);
        assert_eq!(StagePlaceID::Poke_Unova as i32, 0x3C);
        assert_eq!(StagePlaceID::Poke_Tower as i32, 0x3D);
        assert_eq!(StagePlaceID::FE_Arena as i32, 0x3E);
        assert_eq!(StagePlaceID::Icarus_Uprising as i32, 0x3F);
        assert_eq!(StagePlaceID::Animal_Island as i32, 0x40);
        assert_eq!(StagePlaceID::PunchOutSB as i32, 0x41);
        assert_eq!(StagePlaceID::PunchOutW as i32, 0x42);
        assert_eq!(StagePlaceID::Xeno_Gaur as i32, 0x43);
        assert_eq!(StagePlaceID::Nintendogs as i32, 0x44);
        assert_eq!(StagePlaceID::StreetPass as i32, 0x45);
        assert_eq!(StagePlaceID::Tomodachi as i32, 0x46);
        assert_eq!(StagePlaceID::Pictochat2 as i32, 0x47);
        assert_eq!(StagePlaceID::Rock_Wily as i32, 0x48);
        assert_eq!(StagePlaceID::Mother_Magicant as i32, 0x49);
        assert_eq!(StagePlaceID::Kirby_Gameboy as i32, 0x4A);
        assert_eq!(StagePlaceID::BalloonFight as i32, 0x4B);
        assert_eq!(StagePlaceID::Fzero_Mutecity3DS as i32, 0x4C);
        assert_eq!(StagePlaceID::Mario_Uworld as i32, 0x4D);
        assert_eq!(StagePlaceID::Mario_Galaxy as i32, 0x4E);
        assert_eq!(StagePlaceID::Kart_CircuitFor as i32, 0x4F);
        assert_eq!(StagePlaceID::Zelda_Skyward as i32, 0x50);
        assert_eq!(StagePlaceID::Kirby_Cave as i32, 0x51);
        assert_eq!(StagePlaceID::Poke_Kalos as i32, 0x52);
        assert_eq!(StagePlaceID::FE_Colloseum as i32, 0x53);
        assert_eq!(StagePlaceID::Icarus_Angeland as i32, 0x54);
        assert_eq!(StagePlaceID::Wario_Gamer as i32, 0x55);
        assert_eq!(StagePlaceID::Pikmin_Garden as i32, 0x56);
        assert_eq!(StagePlaceID::Animal_City as i32, 0x57);
        assert_eq!(StagePlaceID::WiiFit as i32, 0x58);
        assert_eq!(StagePlaceID::WreckingCrew as i32, 0x59);
        assert_eq!(StagePlaceID::Pilotwings as i32, 0x5A);
        assert_eq!(StagePlaceID::WufuIsland as i32, 0x5B);
        assert_eq!(StagePlaceID::Sonic_Windyhill as i32, 0x5C);
        assert_eq!(StagePlaceID::Pac_Land as i32, 0x5D);
        assert_eq!(StagePlaceID::FlatZoneX as i32, 0x5E);
        assert_eq!(StagePlaceID::DuckHunt as i32, 0x5F);
        assert_eq!(StagePlaceID::SF_Suzaku as i32, 0x60);
        assert_eq!(StagePlaceID::Mario_Maker as i32, 0x61);
        assert_eq!(StagePlaceID::FF_Midgar as i32, 0x62);
        assert_eq!(StagePlaceID::Bayo_Clock as i32, 0x63);
        assert_eq!(StagePlaceID::Spla_Parking as i32, 0x64);
        assert_eq!(StagePlaceID::Dracula_Castle as i32, 0x65);
        assert_eq!(StagePlaceID::Zelda_Tower as i32, 0x66);
        assert_eq!(StagePlaceID::Mario_Odyssey as i32, 0x67);
        assert_eq!(StagePlaceID::Training as i32, 0x68);
        assert_eq!(StagePlaceID::SettingStage as i32, 0x69);
        assert_eq!(StagePlaceID::ResultStage as i32, 0x6A);
        assert_eq!(StagePlaceID::ShamFight as i32, 0x6B);
        assert_eq!(StagePlaceID::SpiritsRoulette as i32, 0x6C);
        assert_eq!(StagePlaceID::CampaignMap as i32, 0x6D);
        assert_eq!(StagePlaceID::BonusGame as i32, 0x6E);
        assert_eq!(StagePlaceID::HomerunContest as i32, 0x6F);
        assert_eq!(StagePlaceID::Staffroll as i32, 0x70);
        assert_eq!(StagePlaceID::BossStage_GanonBoss as i32, 0x71);
        assert_eq!(StagePlaceID::BossStage_Rathalos as i32, 0x72);
        assert_eq!(StagePlaceID::BossStage_Marx as i32, 0x73);
        assert_eq!(StagePlaceID::BossStage_Dracula as i32, 0x74);
        assert_eq!(StagePlaceID::BossStage_Galleom as i32, 0x75);
        assert_eq!(StagePlaceID::BossStage_Final1 as i32, 0x76);
        assert_eq!(StagePlaceID::BossStage_Final2 as i32, 0x77);
        assert_eq!(StagePlaceID::BossStage_Final3 as i32, 0x78);
        assert_eq!(StagePlaceID::SP_Edit as i32, 0x79);
        assert_eq!(StagePlaceID::PhotoStage as i32, 0x7A);
        assert_eq!(StagePlaceID::TestStage as i32, 0x7B);
        assert_eq!(StagePlaceID::Jack_Mementoes as i32, 0x7C);
        assert_eq!(StagePlaceID::ResultStage_Jack as i32, 0x7D);
        assert_eq!(StagePlaceID::Brave_Altar as i32, 0x7E);
        assert_eq!(StagePlaceID::Buddy_Spiral as i32, 0x7F);
        assert_eq!(StagePlaceID::Dolly_Stadium as i32, 0x80);
        assert_eq!(StagePlaceID::FE_Shrine as i32, 0x81);
        assert_eq!(StagePlaceID::Tantan_Spring as i32, 0x82);
        assert_eq!(StagePlaceID::BattleField_S as i32, 0x83);
        assert_eq!(StagePlaceID::Pickel_World as i32, 0x84);
        assert_eq!(StagePlaceID::FF_Cave as i32, 0x85);
        assert_eq!(StagePlaceID::ResultStage_Edge as i32, 0x86);
        assert_eq!(StagePlaceID::Xeno_Alst as i32, 0x87);
        assert_eq!(StagePlaceID::Demon_Dojo as i32, 0x88);
        assert_eq!(StagePlaceID::Trail_Castle as i32, 0x89);
    }
}
