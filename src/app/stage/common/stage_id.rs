use core::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use strum::{EnumCount, EnumString};

/// A numeric identifier for a stage.
#[allow(non_camel_case_types)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(
    Debug, Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd, EnumCount, EnumString,
)]
#[repr(C)]
pub enum StageID {
    /// Battlefield
    #[default]
    BattleField,

    /// Battlefield (Ω Form)
    End_BattleField,

    /// Big Battlefield
    BattleField_L,

    /// Final Destination
    End,

    /// Final Destination (Ω Form)
    Battle_End,

    /// Peach's Castle
    Mario_Castle64,

    /// Peach's Castle (Ω Form)
    End_Mario_Castle64,

    /// Peach's Castle (Battlefield Form)
    Battle_Mario_Castle64,

    /// Kongo Jungle
    DK_Jungle,

    /// Kongo Jungle (Ω Form)
    End_DK_Jungle,

    /// Kongo Jungle (Battlefield Form)
    Battle_DK_Jungle,

    /// Hyrule Castle
    Zelda_Hyrule,

    /// Hyrule Castle (Ω Form)
    End_Zelda_Hyrule,

    /// Hyrule Castle (Battlefield Form)
    Battle_Zelda_Hyrule,

    /// Super Happy Tree
    Yoshi_Story,

    /// Super Happy Tree (Ω Form)
    End_Yoshi_Story,

    /// Super Happy Tree (Battlefield Form)
    Battle_Yoshi_Story,

    /// Dream Land
    Kirby_Pupupu64,

    /// Dream Land (Ω Form)
    End_Kirby_Pupupu64,

    /// Dream Land (Battlefield Form)
    Battle_Kirby_Pupupu64,

    /// Saffron City
    Poke_Yamabuki,

    /// Saffron City (Ω Form)
    End_Poke_Yamabuki,

    /// Saffron City (Battlefield Form)
    Battle_Poke_Yamabuki,

    /// Mushroom Kingdom
    Mario_Past64,

    /// Mushroom Kingdom (Ω Form)
    End_Mario_Past64,

    /// Mushroom Kingdom (Battlefield Form)
    Battle_Mario_Past64,

    /// Princess Peach's Castle
    Mario_CastleDx,

    /// Princess Peach's Castle (Ω Form)
    End_Mario_CastleDx,

    /// Princess Peach's Castle (Battlefield Form)
    Battle_Mario_CastleDx,

    /// Rainbow Cruise
    Mario_Rainbow,

    /// Rainbow Cruise (Ω Form)
    End_Mario_Rainbow,

    /// Rainbow Cruise (Battlefield Form)
    Battle_Mario_Rainbow,

    /// Kongo Falls
    DK_WaterFall,

    /// Kongo Falls (Ω Form)
    End_DK_WaterFall,

    /// Kongo Falls (Battlefield Form)
    Battle_DK_WaterFall,

    /// Jungle Japes
    DK_Lodge,

    /// Jungle Japes (Ω Form)
    End_DK_Lodge,

    /// Jungle Japes (Battlefield Form)
    Battle_DK_Lodge,

    /// Great Bay
    Zelda_Greatbay,

    /// Great Bay (Ω Form)
    End_Zelda_Greatbay,

    /// Great Bay (Battlefield Form)
    Battle_Zelda_Greatbay,

    /// Temple
    Zelda_Temple,

    /// Temple (Ω Form)
    End_Zelda_Temple,

    /// Temple (Battlefield Form)
    Battle_Zelda_Temple,

    /// Yoshi's Story
    Yoshi_CartBoard,

    /// Yoshi's Story (Ω Form)
    End_Yoshi_CartBoard,

    /// Yoshi's Story (Battlefield Form)
    Battle_Yoshi_CartBoard,

    /// Yoshi's Island (Melee)
    Yoshi_Yoster,

    /// Yoshi's Island (Melee) (Ω Form)
    End_Yoshi_Yoster,

    /// Yoshi's Island (Melee) (Battlefield Form)
    Battle_Yoshi_Yoster,

    /// Fountain of Dreams
    Kirby_Fountain,

    /// Fountain of Dreams (Ω Form)
    End_Kirby_Fountain,

    /// Fountain of Dreams (Battlefield Form)
    Battle_Kirby_Fountain,

    /// Green Greens
    Kirby_Greens,

    /// Green Greens (Ω Form)
    End_Kirby_Greens,

    /// Green Greens (Battlefield Form)
    Battle_Kirby_Greens,

    /// Corneria
    Fox_Corneria,

    /// Corneria (Ω Form)
    End_Fox_Corneria,

    /// Corneria (Battlefield Form)
    Battle_Fox_Corneria,

    /// Venom
    Fox_Venom,

    /// Venom (Ω Form)
    End_Fox_Venom,

    /// Venom (Battlefield Form)
    Battle_Fox_Venom,

    /// Brinstar
    Metroid_ZebesDx,

    /// Brinstar (Ω Form)
    End_Metroid_ZebesDx,

    /// Brinstar (Battlefield Form)
    Battle_Metroid_ZebesDx,

    /// Onett
    Mother_Onett,

    /// Onett (Ω Form)
    End_Mother_Onett,

    /// Onett (Battlefield Form)
    Battle_Mother_Onett,

    /// Pokémon Stadium
    Poke_Stadium,

    /// Pokémon Stadium (Ω Form)
    End_Poke_Stadium,

    /// Pokémon Stadium (Battlefield Form)
    Battle_Poke_Stadium,

    /// Brinstar Depths
    Metroid_Kraid,

    /// Brinstar Depths (Ω Form)
    End_Metroid_Kraid,

    /// Brinstar Depths (Battlefield Form)
    Battle_Metroid_Kraid,

    /// Fourside
    Mother_Fourside,

    /// Fourside (Ω Form)
    End_Mother_Fourside,

    /// Fourside (Battlefield Form)
    Battle_Mother_Fourside,

    /// Big Blue
    Fzero_Bigblue,

    /// Big Blue (Ω Form)
    End_Fzero_Bigblue,

    /// Big Blue (Battlefield Form)
    Battle_Fzero_Bigblue,

    /// Mushroom Kingdom II
    Mario_PastUsa,

    /// Mushroom Kingdom II (Ω Form)
    End_Mario_PastUsa,

    /// Mushroom Kingdom II (Battlefield Form)
    Battle_Mario_PastUsa,

    /// Delfino Plaza
    Mario_Dolpic,

    /// Delfino Plaza (Ω Form)
    End_Mario_Dolpic,

    /// Delfino Plaza (Battlefield Form)
    Battle_Mario_Dolpic,

    /// Yoshi's Island
    Yoshi_Island,

    /// Yoshi's Island (Ω Form)
    End_Yoshi_Island,

    /// Yoshi's Island (Battlefield Form)
    Battle_Yoshi_Island,

    /// Lylat Cruise
    Fox_LylatCruise,

    /// Lylat Cruise (Ω Form)
    End_Fox_LylatCruise,

    /// Lylat Cruise (Battlefield Form)
    Battle_Fox_LylatCruise,

    /// Bridge of Eldin
    Zelda_Oldin,

    /// Bridge of Eldin (Ω Form)
    End_Zelda_Oldin,

    /// Bridge of Eldin (Battlefield Form)
    Battle_Zelda_Oldin,

    /// Smashville
    Animal_Village,

    /// Smashville (Ω Form)
    End_Animal_Village,

    /// Smashville (Battlefield Form)
    Battle_Animal_Village,

    /// Skyworld
    Icarus_SkyWorld,

    /// Skyworld (Ω Form)
    End_Icarus_SkyWorld,

    /// Skyworld (Battlefield Form)
    Battle_Icarus_SkyWorld,

    /// Castle Siege
    FE_Siege,

    /// Castle Siege (Ω Form)
    End_FE_Siege,

    /// Castle Siege (Battlefield Form)
    Battle_FE_Siege,

    /// WarioWare, Inc.
    Wario_Madein,

    /// WarioWare, Inc. (Ω Form)
    End_Wario_Madein,

    /// WarioWare, Inc. (Battlefield Form)
    Battle_Wario_Madein,

    /// Pokémon Stadium 2
    Poke_Stadium2,

    /// Pokémon Stadium 2 (Ω Form)
    End_Poke_Stadium2,

    /// Pokémon Stadium 2 (Battlefield Form)
    Battle_Poke_Stadium2,

    /// Halberd
    Kirby_Halberd,

    /// Halberd (Ω Form)
    End_Kirby_Halberd,

    /// Halberd (Battlefield Form)
    Battle_Kirby_Halberd,

    /// Shadow Moses Island
    MG_Shadowmoses,

    /// Shadow Moses Island (Ω Form)
    End_MG_Shadowmoses,

    /// Shadow Moses Island (Battlefield Form)
    Battle_MG_Shadowmoses,

    /// New Pork City
    Mother_Newpork,

    /// New Pork City (Ω Form)
    End_Mother_Newpork,

    /// New Pork City (Battlefield Form)
    Battle_Mother_Newpork,

    /// Summit
    Ice_Top,

    /// Summit (Ω Form)
    End_Ice_Top,

    /// Summit (Battlefield Form)
    Battle_Ice_Top,

    /// Norfair
    Metroid_Norfair,

    /// Norfair (Ω Form)
    End_Metroid_Norfair,

    /// Norfair (Battlefield Form)
    Battle_Metroid_Norfair,

    /// Figure-8 Circuit
    Kart_CircuitX,

    /// Figure-8 Circuit (Ω Form)
    End_Kart_CircuitX,

    /// Figure-8 Circuit (Battlefield Form)
    Battle_Kart_CircuitX,

    /// Frigate Orpheon
    Metroid_Orpheon,

    /// Frigate Orpheon (Ω Form)
    End_Metroid_Orpheon,

    /// Frigate Orpheon (Battlefield Form)
    Battle_Metroid_Orpheon,

    /// Distant Planet
    Pikmin_Planet,

    /// Distant Planet (Ω Form)
    End_Pikmin_Planet,

    /// Distant Planet (Battlefield Form)
    Battle_Pikmin_Planet,

    /// Mushroomy Kingdom
    Mario_PastX,

    /// Mushroomy Kingdom (Ω Form)
    End_Mario_PastX,

    /// Mushroomy Kingdom (Battlefield Form)
    Battle_Mario_PastX,

    /// Port Town Aero Dive
    Fzero_Porttown,

    /// Port Town Aero Dive (Ω Form)
    End_Fzero_Porttown,

    /// Port Town Aero Dive (Battlefield Form)
    Battle_Fzero_Porttown,

    /// Luigi's Mansion
    LuigiMansion,

    /// Luigi's Mansion (Ω Form)
    End_LuigiMansion,

    /// Luigi's Mansion (Battlefield Form)
    Battle_LuigiMansion,

    /// Pirate Ship
    Zelda_Pirates,

    /// Pirate Ship (Ω Form)
    End_Zelda_Pirates,

    /// Pirate Ship (Battlefield Form)
    Battle_Zelda_Pirates,

    /// Spear Pillar
    Poke_Tengam,

    /// Spear Pillar (Ω Form)
    End_Poke_Tengam,

    /// Spear Pillar (Battlefield Form)
    Battle_Poke_Tengam,

    /// 75 m
    _75m,

    /// 75 m (Ω Form)
    End_75m,

    /// 75 m (Battlefield Form)
    Battle_75m,

    /// Mario Bros.
    MarioBros,

    /// Mario Bros. (Ω Form)
    End_MarioBros,

    /// Mario Bros. (Battlefield Form)
    Battle_MarioBros,

    /// Hanenbow
    Plankton,

    /// Hanenbow (Ω Form)
    End_Plankton,

    /// Hanenbow (Battlefield Form)
    Battle_Plankton,

    /// Green Hill Zone
    Sonic_Greenhill,

    /// Green Hill Zone (Ω Form)
    End_Sonic_Greenhill,

    /// Green Hill Zone (Battlefield Form)
    Battle_Sonic_Greenhill,

    /// 3D Land
    Mario_3DLand,

    /// 3D Land (Ω Form)
    End_Mario_3DLand,

    /// 3D Land (Battlefield Form)
    Battle_Mario_3DLand,

    /// Golden Plains
    Mario_NewBros2,

    /// Golden Plains (Ω Form)
    End_Mario_NewBros2,

    /// Golden Plains (Battlefield Form)
    Battle_Mario_NewBros2,

    /// Paper Mario
    Mario_Paper,

    /// Paper Mario (Ω Form)
    End_Mario_Paper,

    /// Paper Mario (Battlefield Form)
    Battle_Mario_Paper,

    /// Gerudo Valley
    Zelda_Gerudo,

    /// Gerudo Valley (Ω Form)
    End_Zelda_Gerudo,

    /// Gerudo Valley (Battlefield Form)
    Battle_Zelda_Gerudo,

    /// Spirit Train
    Zelda_Train,

    /// Spirit Train (Ω Form)
    End_Zelda_Train,

    /// Spirit Train (Battlefield Form)
    Battle_Zelda_Train,

    /// Unova Pokémon League
    Poke_Unova,

    /// Unova Pokémon League (Ω Form)
    End_Poke_Unova,

    /// Unova Pokémon League (Battlefield Form)
    Battle_Poke_Unova,

    /// Prism Tower
    Poke_Tower,

    /// Prism Tower (Ω Form)
    End_Poke_Tower,

    /// Prism Tower (Battlefield Form)
    Battle_Poke_Tower,

    /// Arena Ferox
    FE_Arena,

    /// Arena Ferox (Ω Form)
    End_FE_Arena,

    /// Arena Ferox (Battlefield Form)
    Battle_FE_Arena,

    /// Reset Bomb Forest
    Icarus_Uprising,

    /// Reset Bomb Forest (Ω Form)
    End_Icarus_Uprising,

    /// Reset Bomb Forest (Battlefield Form)
    Battle_Icarus_Uprising,

    /// Tortimer Island
    Animal_Island,

    /// Tortimer Island (Ω Form)
    End_Animal_Island,

    /// Tortimer Island (Battlefield Form)
    Battle_Animal_Island,

    /// Boxing Ring
    PunchOutSB,

    /// Boxing Ring (Ω Form)
    End_PunchOutSB,

    /// Boxing Ring (Battlefield Form)
    Battle_PunchOutSB,

    /// The original version of the Boxing Ring stage.
    PunchOutW,

    /// The original version of the Boxing Ring stage (Ω Form).
    End_PunchOutW,

    /// The original version of the Boxing Ring stage (Battlefield Form).
    Battle_PunchOutW,

    /// Gaur Plain
    Xeno_Gaur,

    /// Gaur Plain (Ω Form)
    End_Xeno_Gaur,

    /// Gaur Plain (Battlefield Form)
    Battle_Xeno_Gaur,

    /// Living Room
    Nintendogs,

    /// Living Room (Ω Form)
    End_Nintendogs,

    /// Living Room (Battlefield Form)
    Battle_Nintendogs,

    /// Find Mii
    StreetPass,

    /// Find Mii (Ω Form)
    End_StreetPass,

    /// Find Mii (Battlefield Form)
    Battle_StreetPass,

    /// Tomodachi Life
    Tomodachi,

    /// Tomodachi Life (Ω Form)
    End_Tomodachi,

    /// Tomodachi Life (Battlefield Form)
    Battle_Tomodachi,

    /// PictoChat 2
    Pictochat2,

    /// PictoChat 2 (Ω Form)
    End_Pictochat2,

    /// PictoChat 2 (Battlefield Form)
    Battle_Pictochat2,

    /// Wily Castle
    Rock_Wily,

    /// Wily Castle (Ω Form)
    End_Rock_Wily,

    /// Wily Castle (Battlefield Form)
    Battle_Rock_Wily,

    /// Magicant
    Mother_Magicant,

    /// Magicant (Ω Form)
    End_Mother_Magicant,

    /// Magicant (Battlefield Form)
    Battle_Mother_Magicant,

    /// Dream Land GB
    Kirby_Gameboy,

    /// Dream Land GB (Ω Form)
    End_Kirby_Gameboy,

    /// Dream Land GB (Battlefield Form)
    Battle_Kirby_Gameboy,

    /// Balloon Fight
    BalloonFight,

    /// Balloon Fight (Ω Form)
    End_BalloonFight,

    /// Balloon Fight (Battlefield Form)
    Battle_BalloonFight,

    /// Mute City SNES
    Fzero_Mutecity3DS,

    /// Mute City SNES (Ω Form)
    End_Fzero_Mutecity3DS,

    /// Mute City SNES (Battlefield Form)
    Battle_Fzero_Mutecity3DS,

    /// Mushroom Kingdom U
    Mario_Uworld,

    /// Mushroom Kingdom U (Ω Form)
    End_Mario_Uworld,

    /// Mushroom Kingdom U (Battlefield Form)
    Battle_Mario_Uworld,

    /// Mario Galaxy
    Mario_Galaxy,

    /// Mario Galaxy (Ω Form)
    End_Mario_Galaxy,

    /// Mario Galaxy (Battlefield Form)
    Battle_Mario_Galaxy,

    /// Mario Circuit
    Kart_CircuitFor,

    /// Mario Circuit (Ω Form)
    End_Kart_CircuitFor,

    /// Mario Circuit (Battlefield Form)
    Battle_Kart_CircuitFor,

    /// Skyloft
    Zelda_Skyward,

    /// Skyloft (Ω Form)
    End_Zelda_Skyward,

    /// Skyloft (Battlefield Form)
    Battle_Zelda_Skyward,

    /// The Great Cave Offensive
    Kirby_Cave,

    /// The Great Cave Offensive (Ω Form)
    End_Kirby_Cave,

    /// The Great Cave Offensive (Battlefield Form)
    Battle_Kirby_Cave,

    /// Kalos Pokémon League
    Poke_Kalos,

    /// Kalos Pokémon League (Ω Form)
    End_Poke_Kalos,

    /// Kalos Pokémon League (Battlefield Form)
    Battle_Poke_Kalos,

    /// Coliseum
    FE_Colloseum,

    /// Coliseum (Ω Form)
    End_FE_Colloseum,

    /// Coliseum (Battlefield Form)
    Battle_FE_Colloseum,

    /// Palutena's Temple
    Icarus_Angeland,

    /// Palutena's Temple (Ω Form)
    End_Icarus_Angeland,

    /// Palutena's Temple (Battlefield Form)
    Battle_Icarus_Angeland,

    /// Gamer
    Wario_Gamer,

    /// Gamer (Ω Form)
    End_Wario_Gamer,

    /// Gamer (Battlefield Form)
    Battle_Wario_Gamer,

    /// Garden of Hope
    Pikmin_Garden,

    /// Garden of Hope (Ω Form)
    End_Pikmin_Garden,

    /// Garden of Hope (Battlefield Form)
    Battle_Pikmin_Garden,

    /// Town and City
    Animal_City,

    /// Town and City (Ω Form)
    End_Animal_City,

    /// Town and City (Battlefield Form)
    Battle_Animal_City,

    /// Wii Fit Studio
    WiiFit,

    /// Wii Fit Studio (Ω Form)
    End_WiiFit,

    /// Wii Fit Studio (Battlefield Form)
    Battle_WiiFit,

    /// Wrecking Crew
    WreckingCrew,

    /// Wrecking Crew (Ω Form)
    End_WreckingCrew,

    /// Wrecking Crew (Battlefield Form)
    Battle_WreckingCrew,

    /// Pilotwings
    Pilotwings,

    /// Pilotwings (Ω Form)
    End_Pilotwings,

    /// Pilotwings (Battlefield Form)
    Battle_Pilotwings,

    /// Wuhu Island
    WufuIsland,

    /// Wuhu Island (Ω Form)
    End_WufuIsland,

    /// Wuhu Island (Battlefield Form)
    Battle_WufuIsland,

    /// Windy Hill Zone
    Sonic_Windyhill,

    /// Windy Hill Zone (Ω Form)
    End_Sonic_Windyhill,

    /// Windy Hill Zone (Battlefield Form)
    Battle_Sonic_Windyhill,

    /// PAC-LAND
    Pac_Land,

    /// PAC-LAND (Ω Form)
    End_Pac_Land,

    /// PAC-LAND (Battlefield Form)
    Battle_Pac_Land,

    /// Flat Zone X
    FlatZoneX,

    /// Flat Zone X (Ω Form)
    End_FlatZoneX,

    /// Flat Zone X (Battlefield Form)
    Battle_FlatZoneX,

    /// Duck Hunt
    DuckHunt,

    /// Duck Hunt (Ω Form)
    End_DuckHunt,

    /// Duck Hunt (Battlefield Form)
    Battle_DuckHunt,

    /// Suzaku Castle
    SF_Suzaku,

    /// Suzaku Castle (Ω Form)
    End_SF_Suzaku,

    /// Suzaku Castle (Battlefield Form)
    Battle_SF_Suzaku,

    /// Super Mario Maker
    Mario_Maker,

    /// Super Mario Maker (Ω Form)
    End_Mario_Maker,

    /// Super Mario Maker (Battlefield Form)
    Battle_Mario_Maker,

    /// Midgar
    FF_Midgar,

    /// Midgar (Ω Form)
    End_FF_Midgar,

    /// Midgar (Battlefield Form)
    Battle_FF_Midgar,

    /// Umbra Clock Tower
    Bayo_Clock,

    /// Umbra Clock Tower (Ω Form)
    End_Bayo_Clock,

    /// Umbra Clock Tower (Battlefield Form)
    Battle_Bayo_Clock,

    /// Moray Towers
    Spla_Parking,

    /// Moray Towers (Ω Form)
    End_Spla_Parking,

    /// Moray Towers (Battlefield Form)
    Battle_Spla_Parking,

    /// Dracula's Castle
    Dracula_Castle,

    /// Dracula's Castle (Ω Form)
    End_Dracula_Castle,

    /// Dracula's Castle (Battlefield Form)
    Battle_Dracula_Castle,

    /// Great Plateau Tower
    Zelda_Tower,

    /// Great Plateau Tower (Ω Form)
    End_Zelda_Tower,

    /// Great Plateau Tower (Battlefield Form)
    Battle_Zelda_Tower,

    /// New Donk City Hall
    Mario_Odyssey,

    /// New Donk City Hall (Ω Form)
    End_Mario_Odyssey,

    /// New Donk City Hall (Battlefield Form)
    Battle_Mario_Odyssey,

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

    /// Mementos (Ω Form)
    End_Jack_Mementoes,

    /// Mementos (Battlefield Form)
    Battle_Jack_Mementoes,

    /// The stage where the results take place when Joker is the winner.
    ResultStage_Jack,

    /// Yggdrasil's Altar
    Brave_Altar,

    /// Yggdrasil's Altar (Ω Form)
    End_Brave_Altar,

    /// Yggdrasil's Altar (Battlefield Form)
    Battle_Brave_Altar,

    /// Spiral Mountain
    Buddy_Spiral,

    /// Spiral Mountain (Ω Form)
    End_Buddy_Spiral,

    /// Spiral Mountain (Battlefield Form)
    Battle_Buddy_Spiral,

    /// King of Fighters Stadium
    Dolly_Stadium,

    /// King of Fighters Stadium (Ω Form)
    End_Dolly_Stadium,

    /// King of Fighters Stadium (Battlefield Form)
    Battle_Dolly_Stadium,

    /// Garreg Mach Monastary
    FE_Shrine,

    /// Garreg Mach Monastary (Ω Form)
    End_FE_Shrine,

    /// Garreg Mach Monastary (Battlefield Form)
    Battle_FE_Shrine,

    /// Spring Stadium
    Tantan_Spring,

    /// Spring Stadium (Ω Form)
    End_Tantan_Spring,

    /// Spring Stadium (Battlefield Form)
    Battle_Tantan_Spring,

    /// Small Battlefield
    BattleField_S,

    /// Minecraft World
    Pickel_World,

    /// Minecraft World (Ω Form)
    End_Pickel_World,

    /// Minecraft World (Battlefield Form)
    Battle_Pickel_World,

    /// Northern Cave
    FF_Cave,

    /// Northern Cave (Ω Form)
    End_FF_Cave,

    /// Northern Cave (Battlefield Form)
    Battle_FF_Cave,

    /// The stage where the results take place when Sephiroth is the winner.
    ResultStage_Edge,

    /// Cloud Sea of Alrest
    Xeno_Alst,

    /// Cloud Sea of Alrest (Ω Form)
    End_Xeno_Alst,

    /// Cloud Sea of Alrest (Battlefield Form)
    Battle_Xeno_Alst,

    /// Mishima Dojo
    Demon_Dojo,

    /// Mishima Dojo (Ω Form)
    End_Demon_Dojo,

    /// Mishima Dojo (Battlefield Form)
    Battle_Demon_Dojo,

    /// Hollow Bastion
    Trail_Castle,

    /// Hollow Bastion (Ω Form)
    End_Trail_Castle,

    /// Hollow Bastion (Battlefield Form)
    Battle_Trail_Castle,
}

impl StageID {
    /// The total number of identifiers.
    pub const COUNT: usize = <Self as EnumCount>::COUNT;
}

impl fmt::Display for StageID {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn assert_mem() {
        use super::*;

        assert_eq!(size_of::<StageID>(), 0x4);
        assert_eq!(StageID::BattleField as i32, 0x0);
        assert_eq!(StageID::End_BattleField as i32, 0x1);
        assert_eq!(StageID::BattleField_L as i32, 0x2);
        assert_eq!(StageID::End as i32, 0x3);
        assert_eq!(StageID::Battle_End as i32, 0x4);
        assert_eq!(StageID::Mario_Castle64 as i32, 0x5);
        assert_eq!(StageID::End_Mario_Castle64 as i32, 0x6);
        assert_eq!(StageID::Battle_Mario_Castle64 as i32, 0x7);
        assert_eq!(StageID::DK_Jungle as i32, 0x8);
        assert_eq!(StageID::End_DK_Jungle as i32, 0x9);
        assert_eq!(StageID::Battle_DK_Jungle as i32, 0xA);
        assert_eq!(StageID::Zelda_Hyrule as i32, 0xB);
        assert_eq!(StageID::End_Zelda_Hyrule as i32, 0xC);
        assert_eq!(StageID::Battle_Zelda_Hyrule as i32, 0xD);
        assert_eq!(StageID::Yoshi_Story as i32, 0xE);
        assert_eq!(StageID::End_Yoshi_Story as i32, 0xF);
        assert_eq!(StageID::Battle_Yoshi_Story as i32, 0x10);
        assert_eq!(StageID::Kirby_Pupupu64 as i32, 0x11);
        assert_eq!(StageID::End_Kirby_Pupupu64 as i32, 0x12);
        assert_eq!(StageID::Battle_Kirby_Pupupu64 as i32, 0x13);
        assert_eq!(StageID::Poke_Yamabuki as i32, 0x14);
        assert_eq!(StageID::End_Poke_Yamabuki as i32, 0x15);
        assert_eq!(StageID::Battle_Poke_Yamabuki as i32, 0x16);
        assert_eq!(StageID::Mario_Past64 as i32, 0x17);
        assert_eq!(StageID::End_Mario_Past64 as i32, 0x18);
        assert_eq!(StageID::Battle_Mario_Past64 as i32, 0x19);
        assert_eq!(StageID::Mario_CastleDx as i32, 0x1A);
        assert_eq!(StageID::End_Mario_CastleDx as i32, 0x1B);
        assert_eq!(StageID::Battle_Mario_CastleDx as i32, 0x1C);
        assert_eq!(StageID::Mario_Rainbow as i32, 0x1D);
        assert_eq!(StageID::End_Mario_Rainbow as i32, 0x1E);
        assert_eq!(StageID::Battle_Mario_Rainbow as i32, 0x1F);
        assert_eq!(StageID::DK_WaterFall as i32, 0x20);
        assert_eq!(StageID::End_DK_WaterFall as i32, 0x21);
        assert_eq!(StageID::Battle_DK_WaterFall as i32, 0x22);
        assert_eq!(StageID::DK_Lodge as i32, 0x23);
        assert_eq!(StageID::End_DK_Lodge as i32, 0x24);
        assert_eq!(StageID::Battle_DK_Lodge as i32, 0x25);
        assert_eq!(StageID::Zelda_Greatbay as i32, 0x26);
        assert_eq!(StageID::End_Zelda_Greatbay as i32, 0x27);
        assert_eq!(StageID::Battle_Zelda_Greatbay as i32, 0x28);
        assert_eq!(StageID::Zelda_Temple as i32, 0x29);
        assert_eq!(StageID::End_Zelda_Temple as i32, 0x2A);
        assert_eq!(StageID::Battle_Zelda_Temple as i32, 0x2B);
        assert_eq!(StageID::Yoshi_CartBoard as i32, 0x2C);
        assert_eq!(StageID::End_Yoshi_CartBoard as i32, 0x2D);
        assert_eq!(StageID::Battle_Yoshi_CartBoard as i32, 0x2E);
        assert_eq!(StageID::Yoshi_Yoster as i32, 0x2F);
        assert_eq!(StageID::End_Yoshi_Yoster as i32, 0x30);
        assert_eq!(StageID::Battle_Yoshi_Yoster as i32, 0x31);
        assert_eq!(StageID::Kirby_Fountain as i32, 0x32);
        assert_eq!(StageID::End_Kirby_Fountain as i32, 0x33);
        assert_eq!(StageID::Battle_Kirby_Fountain as i32, 0x34);
        assert_eq!(StageID::Kirby_Greens as i32, 0x35);
        assert_eq!(StageID::End_Kirby_Greens as i32, 0x36);
        assert_eq!(StageID::Battle_Kirby_Greens as i32, 0x37);
        assert_eq!(StageID::Fox_Corneria as i32, 0x38);
        assert_eq!(StageID::End_Fox_Corneria as i32, 0x39);
        assert_eq!(StageID::Battle_Fox_Corneria as i32, 0x3A);
        assert_eq!(StageID::Fox_Venom as i32, 0x3B);
        assert_eq!(StageID::End_Fox_Venom as i32, 0x3C);
        assert_eq!(StageID::Battle_Fox_Venom as i32, 0x3D);
        assert_eq!(StageID::Metroid_ZebesDx as i32, 0x3E);
        assert_eq!(StageID::End_Metroid_ZebesDx as i32, 0x3F);
        assert_eq!(StageID::Battle_Metroid_ZebesDx as i32, 0x40);
        assert_eq!(StageID::Mother_Onett as i32, 0x41);
        assert_eq!(StageID::End_Mother_Onett as i32, 0x42);
        assert_eq!(StageID::Battle_Mother_Onett as i32, 0x43);
        assert_eq!(StageID::Poke_Stadium as i32, 0x44);
        assert_eq!(StageID::End_Poke_Stadium as i32, 0x45);
        assert_eq!(StageID::Battle_Poke_Stadium as i32, 0x46);
        assert_eq!(StageID::Metroid_Kraid as i32, 0x47);
        assert_eq!(StageID::End_Metroid_Kraid as i32, 0x48);
        assert_eq!(StageID::Battle_Metroid_Kraid as i32, 0x49);
        assert_eq!(StageID::Mother_Fourside as i32, 0x4A);
        assert_eq!(StageID::End_Mother_Fourside as i32, 0x4B);
        assert_eq!(StageID::Battle_Mother_Fourside as i32, 0x4C);
        assert_eq!(StageID::Fzero_Bigblue as i32, 0x4D);
        assert_eq!(StageID::End_Fzero_Bigblue as i32, 0x4E);
        assert_eq!(StageID::Battle_Fzero_Bigblue as i32, 0x4F);
        assert_eq!(StageID::Mario_PastUsa as i32, 0x50);
        assert_eq!(StageID::End_Mario_PastUsa as i32, 0x51);
        assert_eq!(StageID::Battle_Mario_PastUsa as i32, 0x52);
        assert_eq!(StageID::Mario_Dolpic as i32, 0x53);
        assert_eq!(StageID::End_Mario_Dolpic as i32, 0x54);
        assert_eq!(StageID::Battle_Mario_Dolpic as i32, 0x55);
        assert_eq!(StageID::Yoshi_Island as i32, 0x56);
        assert_eq!(StageID::End_Yoshi_Island as i32, 0x57);
        assert_eq!(StageID::Battle_Yoshi_Island as i32, 0x58);
        assert_eq!(StageID::Fox_LylatCruise as i32, 0x59);
        assert_eq!(StageID::End_Fox_LylatCruise as i32, 0x5A);
        assert_eq!(StageID::Battle_Fox_LylatCruise as i32, 0x5B);
        assert_eq!(StageID::Zelda_Oldin as i32, 0x5C);
        assert_eq!(StageID::End_Zelda_Oldin as i32, 0x5D);
        assert_eq!(StageID::Battle_Zelda_Oldin as i32, 0x5E);
        assert_eq!(StageID::Animal_Village as i32, 0x5F);
        assert_eq!(StageID::End_Animal_Village as i32, 0x60);
        assert_eq!(StageID::Battle_Animal_Village as i32, 0x61);
        assert_eq!(StageID::Icarus_SkyWorld as i32, 0x62);
        assert_eq!(StageID::End_Icarus_SkyWorld as i32, 0x63);
        assert_eq!(StageID::Battle_Icarus_SkyWorld as i32, 0x64);
        assert_eq!(StageID::FE_Siege as i32, 0x65);
        assert_eq!(StageID::End_FE_Siege as i32, 0x66);
        assert_eq!(StageID::Battle_FE_Siege as i32, 0x67);
        assert_eq!(StageID::Wario_Madein as i32, 0x68);
        assert_eq!(StageID::End_Wario_Madein as i32, 0x69);
        assert_eq!(StageID::Battle_Wario_Madein as i32, 0x6A);
        assert_eq!(StageID::Poke_Stadium2 as i32, 0x6B);
        assert_eq!(StageID::End_Poke_Stadium2 as i32, 0x6C);
        assert_eq!(StageID::Battle_Poke_Stadium2 as i32, 0x6D);
        assert_eq!(StageID::Kirby_Halberd as i32, 0x6E);
        assert_eq!(StageID::End_Kirby_Halberd as i32, 0x6F);
        assert_eq!(StageID::Battle_Kirby_Halberd as i32, 0x70);
        assert_eq!(StageID::MG_Shadowmoses as i32, 0x71);
        assert_eq!(StageID::End_MG_Shadowmoses as i32, 0x72);
        assert_eq!(StageID::Battle_MG_Shadowmoses as i32, 0x73);
        assert_eq!(StageID::Mother_Newpork as i32, 0x74);
        assert_eq!(StageID::End_Mother_Newpork as i32, 0x75);
        assert_eq!(StageID::Battle_Mother_Newpork as i32, 0x76);
        assert_eq!(StageID::Ice_Top as i32, 0x77);
        assert_eq!(StageID::End_Ice_Top as i32, 0x78);
        assert_eq!(StageID::Battle_Ice_Top as i32, 0x79);
        assert_eq!(StageID::Metroid_Norfair as i32, 0x7A);
        assert_eq!(StageID::End_Metroid_Norfair as i32, 0x7B);
        assert_eq!(StageID::Battle_Metroid_Norfair as i32, 0x7C);
        assert_eq!(StageID::Kart_CircuitX as i32, 0x7D);
        assert_eq!(StageID::End_Kart_CircuitX as i32, 0x7E);
        assert_eq!(StageID::Battle_Kart_CircuitX as i32, 0x7F);
        assert_eq!(StageID::Metroid_Orpheon as i32, 0x80);
        assert_eq!(StageID::End_Metroid_Orpheon as i32, 0x81);
        assert_eq!(StageID::Battle_Metroid_Orpheon as i32, 0x82);
        assert_eq!(StageID::Pikmin_Planet as i32, 0x83);
        assert_eq!(StageID::End_Pikmin_Planet as i32, 0x84);
        assert_eq!(StageID::Battle_Pikmin_Planet as i32, 0x85);
        assert_eq!(StageID::Mario_PastX as i32, 0x86);
        assert_eq!(StageID::End_Mario_PastX as i32, 0x87);
        assert_eq!(StageID::Battle_Mario_PastX as i32, 0x88);
        assert_eq!(StageID::Fzero_Porttown as i32, 0x89);
        assert_eq!(StageID::End_Fzero_Porttown as i32, 0x8A);
        assert_eq!(StageID::Battle_Fzero_Porttown as i32, 0x8B);
        assert_eq!(StageID::LuigiMansion as i32, 0x8C);
        assert_eq!(StageID::End_LuigiMansion as i32, 0x8D);
        assert_eq!(StageID::Battle_LuigiMansion as i32, 0x8E);
        assert_eq!(StageID::Zelda_Pirates as i32, 0x8F);
        assert_eq!(StageID::End_Zelda_Pirates as i32, 0x90);
        assert_eq!(StageID::Battle_Zelda_Pirates as i32, 0x91);
        assert_eq!(StageID::Poke_Tengam as i32, 0x92);
        assert_eq!(StageID::End_Poke_Tengam as i32, 0x93);
        assert_eq!(StageID::Battle_Poke_Tengam as i32, 0x94);
        assert_eq!(StageID::_75m as i32, 0x95);
        assert_eq!(StageID::End_75m as i32, 0x96);
        assert_eq!(StageID::Battle_75m as i32, 0x97);
        assert_eq!(StageID::MarioBros as i32, 0x98);
        assert_eq!(StageID::End_MarioBros as i32, 0x99);
        assert_eq!(StageID::Battle_MarioBros as i32, 0x9A);
        assert_eq!(StageID::Plankton as i32, 0x9B);
        assert_eq!(StageID::End_Plankton as i32, 0x9C);
        assert_eq!(StageID::Battle_Plankton as i32, 0x9D);
        assert_eq!(StageID::Sonic_Greenhill as i32, 0x9E);
        assert_eq!(StageID::End_Sonic_Greenhill as i32, 0x9F);
        assert_eq!(StageID::Battle_Sonic_Greenhill as i32, 0xA0);
        assert_eq!(StageID::Mario_3DLand as i32, 0xA1);
        assert_eq!(StageID::End_Mario_3DLand as i32, 0xA2);
        assert_eq!(StageID::Battle_Mario_3DLand as i32, 0xA3);
        assert_eq!(StageID::Mario_NewBros2 as i32, 0xA4);
        assert_eq!(StageID::End_Mario_NewBros2 as i32, 0xA5);
        assert_eq!(StageID::Battle_Mario_NewBros2 as i32, 0xA6);
        assert_eq!(StageID::Mario_Paper as i32, 0xA7);
        assert_eq!(StageID::End_Mario_Paper as i32, 0xA8);
        assert_eq!(StageID::Battle_Mario_Paper as i32, 0xA9);
        assert_eq!(StageID::Zelda_Gerudo as i32, 0xAA);
        assert_eq!(StageID::End_Zelda_Gerudo as i32, 0xAB);
        assert_eq!(StageID::Battle_Zelda_Gerudo as i32, 0xAC);
        assert_eq!(StageID::Zelda_Train as i32, 0xAD);
        assert_eq!(StageID::End_Zelda_Train as i32, 0xAE);
        assert_eq!(StageID::Battle_Zelda_Train as i32, 0xAF);
        assert_eq!(StageID::Poke_Unova as i32, 0xB0);
        assert_eq!(StageID::End_Poke_Unova as i32, 0xB1);
        assert_eq!(StageID::Battle_Poke_Unova as i32, 0xB2);
        assert_eq!(StageID::Poke_Tower as i32, 0xB3);
        assert_eq!(StageID::End_Poke_Tower as i32, 0xB4);
        assert_eq!(StageID::Battle_Poke_Tower as i32, 0xB5);
        assert_eq!(StageID::FE_Arena as i32, 0xB6);
        assert_eq!(StageID::End_FE_Arena as i32, 0xB7);
        assert_eq!(StageID::Battle_FE_Arena as i32, 0xB8);
        assert_eq!(StageID::Icarus_Uprising as i32, 0xB9);
        assert_eq!(StageID::End_Icarus_Uprising as i32, 0xBA);
        assert_eq!(StageID::Battle_Icarus_Uprising as i32, 0xBB);
        assert_eq!(StageID::Animal_Island as i32, 0xBC);
        assert_eq!(StageID::End_Animal_Island as i32, 0xBD);
        assert_eq!(StageID::Battle_Animal_Island as i32, 0xBE);
        assert_eq!(StageID::PunchOutSB as i32, 0xBF);
        assert_eq!(StageID::End_PunchOutSB as i32, 0xC0);
        assert_eq!(StageID::Battle_PunchOutSB as i32, 0xC1);
        assert_eq!(StageID::PunchOutW as i32, 0xC2);
        assert_eq!(StageID::End_PunchOutW as i32, 0xC3);
        assert_eq!(StageID::Battle_PunchOutW as i32, 0xC4);
        assert_eq!(StageID::Xeno_Gaur as i32, 0xC5);
        assert_eq!(StageID::End_Xeno_Gaur as i32, 0xC6);
        assert_eq!(StageID::Battle_Xeno_Gaur as i32, 0xC7);
        assert_eq!(StageID::Nintendogs as i32, 0xC8);
        assert_eq!(StageID::End_Nintendogs as i32, 0xC9);
        assert_eq!(StageID::Battle_Nintendogs as i32, 0xCA);
        assert_eq!(StageID::StreetPass as i32, 0xCB);
        assert_eq!(StageID::End_StreetPass as i32, 0xCC);
        assert_eq!(StageID::Battle_StreetPass as i32, 0xCD);
        assert_eq!(StageID::Tomodachi as i32, 0xCE);
        assert_eq!(StageID::End_Tomodachi as i32, 0xCF);
        assert_eq!(StageID::Battle_Tomodachi as i32, 0xD0);
        assert_eq!(StageID::Pictochat2 as i32, 0xD1);
        assert_eq!(StageID::End_Pictochat2 as i32, 0xD2);
        assert_eq!(StageID::Battle_Pictochat2 as i32, 0xD3);
        assert_eq!(StageID::Rock_Wily as i32, 0xD4);
        assert_eq!(StageID::End_Rock_Wily as i32, 0xD5);
        assert_eq!(StageID::Battle_Rock_Wily as i32, 0xD6);
        assert_eq!(StageID::Mother_Magicant as i32, 0xD7);
        assert_eq!(StageID::End_Mother_Magicant as i32, 0xD8);
        assert_eq!(StageID::Battle_Mother_Magicant as i32, 0xD9);
        assert_eq!(StageID::Kirby_Gameboy as i32, 0xDA);
        assert_eq!(StageID::End_Kirby_Gameboy as i32, 0xDB);
        assert_eq!(StageID::Battle_Kirby_Gameboy as i32, 0xDC);
        assert_eq!(StageID::BalloonFight as i32, 0xDD);
        assert_eq!(StageID::End_BalloonFight as i32, 0xDE);
        assert_eq!(StageID::Battle_BalloonFight as i32, 0xDF);
        assert_eq!(StageID::Fzero_Mutecity3DS as i32, 0xE0);
        assert_eq!(StageID::End_Fzero_Mutecity3DS as i32, 0xE1);
        assert_eq!(StageID::Battle_Fzero_Mutecity3DS as i32, 0xE2);
        assert_eq!(StageID::Mario_Uworld as i32, 0xE3);
        assert_eq!(StageID::End_Mario_Uworld as i32, 0xE4);
        assert_eq!(StageID::Battle_Mario_Uworld as i32, 0xE5);
        assert_eq!(StageID::Mario_Galaxy as i32, 0xE6);
        assert_eq!(StageID::End_Mario_Galaxy as i32, 0xE7);
        assert_eq!(StageID::Battle_Mario_Galaxy as i32, 0xE8);
        assert_eq!(StageID::Kart_CircuitFor as i32, 0xE9);
        assert_eq!(StageID::End_Kart_CircuitFor as i32, 0xEA);
        assert_eq!(StageID::Battle_Kart_CircuitFor as i32, 0xEB);
        assert_eq!(StageID::Zelda_Skyward as i32, 0xEC);
        assert_eq!(StageID::End_Zelda_Skyward as i32, 0xED);
        assert_eq!(StageID::Battle_Zelda_Skyward as i32, 0xEE);
        assert_eq!(StageID::Kirby_Cave as i32, 0xEF);
        assert_eq!(StageID::End_Kirby_Cave as i32, 0xF0);
        assert_eq!(StageID::Battle_Kirby_Cave as i32, 0xF1);
        assert_eq!(StageID::Poke_Kalos as i32, 0xF2);
        assert_eq!(StageID::End_Poke_Kalos as i32, 0xF3);
        assert_eq!(StageID::Battle_Poke_Kalos as i32, 0xF4);
        assert_eq!(StageID::FE_Colloseum as i32, 0xF5);
        assert_eq!(StageID::End_FE_Colloseum as i32, 0xF6);
        assert_eq!(StageID::Battle_FE_Colloseum as i32, 0xF7);
        assert_eq!(StageID::Icarus_Angeland as i32, 0xF8);
        assert_eq!(StageID::End_Icarus_Angeland as i32, 0xF9);
        assert_eq!(StageID::Battle_Icarus_Angeland as i32, 0xFA);
        assert_eq!(StageID::Wario_Gamer as i32, 0xFB);
        assert_eq!(StageID::End_Wario_Gamer as i32, 0xFC);
        assert_eq!(StageID::Battle_Wario_Gamer as i32, 0xFD);
        assert_eq!(StageID::Pikmin_Garden as i32, 0xFE);
        assert_eq!(StageID::End_Pikmin_Garden as i32, 0xFF);
        assert_eq!(StageID::Battle_Pikmin_Garden as i32, 0x100);
        assert_eq!(StageID::Animal_City as i32, 0x101);
        assert_eq!(StageID::End_Animal_City as i32, 0x102);
        assert_eq!(StageID::Battle_Animal_City as i32, 0x103);
        assert_eq!(StageID::WiiFit as i32, 0x104);
        assert_eq!(StageID::End_WiiFit as i32, 0x105);
        assert_eq!(StageID::Battle_WiiFit as i32, 0x106);
        assert_eq!(StageID::WreckingCrew as i32, 0x107);
        assert_eq!(StageID::End_WreckingCrew as i32, 0x108);
        assert_eq!(StageID::Battle_WreckingCrew as i32, 0x109);
        assert_eq!(StageID::Pilotwings as i32, 0x10A);
        assert_eq!(StageID::End_Pilotwings as i32, 0x10B);
        assert_eq!(StageID::Battle_Pilotwings as i32, 0x10C);
        assert_eq!(StageID::WufuIsland as i32, 0x10D);
        assert_eq!(StageID::End_WufuIsland as i32, 0x10E);
        assert_eq!(StageID::Battle_WufuIsland as i32, 0x10F);
        assert_eq!(StageID::Sonic_Windyhill as i32, 0x110);
        assert_eq!(StageID::End_Sonic_Windyhill as i32, 0x111);
        assert_eq!(StageID::Battle_Sonic_Windyhill as i32, 0x112);
        assert_eq!(StageID::Pac_Land as i32, 0x113);
        assert_eq!(StageID::End_Pac_Land as i32, 0x114);
        assert_eq!(StageID::Battle_Pac_Land as i32, 0x115);
        assert_eq!(StageID::FlatZoneX as i32, 0x116);
        assert_eq!(StageID::End_FlatZoneX as i32, 0x117);
        assert_eq!(StageID::Battle_FlatZoneX as i32, 0x118);
        assert_eq!(StageID::DuckHunt as i32, 0x119);
        assert_eq!(StageID::End_DuckHunt as i32, 0x11A);
        assert_eq!(StageID::Battle_DuckHunt as i32, 0x11B);
        assert_eq!(StageID::SF_Suzaku as i32, 0x11C);
        assert_eq!(StageID::End_SF_Suzaku as i32, 0x11D);
        assert_eq!(StageID::Battle_SF_Suzaku as i32, 0x11E);
        assert_eq!(StageID::Mario_Maker as i32, 0x11F);
        assert_eq!(StageID::End_Mario_Maker as i32, 0x120);
        assert_eq!(StageID::Battle_Mario_Maker as i32, 0x121);
        assert_eq!(StageID::FF_Midgar as i32, 0x122);
        assert_eq!(StageID::End_FF_Midgar as i32, 0x123);
        assert_eq!(StageID::Battle_FF_Midgar as i32, 0x124);
        assert_eq!(StageID::Bayo_Clock as i32, 0x125);
        assert_eq!(StageID::End_Bayo_Clock as i32, 0x126);
        assert_eq!(StageID::Battle_Bayo_Clock as i32, 0x127);
        assert_eq!(StageID::Spla_Parking as i32, 0x128);
        assert_eq!(StageID::End_Spla_Parking as i32, 0x129);
        assert_eq!(StageID::Battle_Spla_Parking as i32, 0x12A);
        assert_eq!(StageID::Dracula_Castle as i32, 0x12B);
        assert_eq!(StageID::End_Dracula_Castle as i32, 0x12C);
        assert_eq!(StageID::Battle_Dracula_Castle as i32, 0x12D);
        assert_eq!(StageID::Zelda_Tower as i32, 0x12E);
        assert_eq!(StageID::End_Zelda_Tower as i32, 0x12F);
        assert_eq!(StageID::Battle_Zelda_Tower as i32, 0x130);
        assert_eq!(StageID::Mario_Odyssey as i32, 0x131);
        assert_eq!(StageID::End_Mario_Odyssey as i32, 0x132);
        assert_eq!(StageID::Battle_Mario_Odyssey as i32, 0x133);
        assert_eq!(StageID::Training as i32, 0x134);
        assert_eq!(StageID::SettingStage as i32, 0x135);
        assert_eq!(StageID::ResultStage as i32, 0x136);
        assert_eq!(StageID::ShamFight as i32, 0x137);
        assert_eq!(StageID::SpiritsRoulette as i32, 0x138);
        assert_eq!(StageID::CampaignMap as i32, 0x139);
        assert_eq!(StageID::BonusGame as i32, 0x13A);
        assert_eq!(StageID::HomerunContest as i32, 0x13B);
        assert_eq!(StageID::Staffroll as i32, 0x13C);
        assert_eq!(StageID::BossStage_GanonBoss as i32, 0x13D);
        assert_eq!(StageID::BossStage_Rathalos as i32, 0x13E);
        assert_eq!(StageID::BossStage_Marx as i32, 0x13F);
        assert_eq!(StageID::BossStage_Dracula as i32, 0x140);
        assert_eq!(StageID::BossStage_Galleom as i32, 0x141);
        assert_eq!(StageID::BossStage_Final1 as i32, 0x142);
        assert_eq!(StageID::BossStage_Final2 as i32, 0x143);
        assert_eq!(StageID::BossStage_Final3 as i32, 0x144);
        assert_eq!(StageID::SP_Edit as i32, 0x145);
        assert_eq!(StageID::PhotoStage as i32, 0x146);
        assert_eq!(StageID::TestStage as i32, 0x147);
        assert_eq!(StageID::Jack_Mementoes as i32, 0x148);
        assert_eq!(StageID::End_Jack_Mementoes as i32, 0x149);
        assert_eq!(StageID::Battle_Jack_Mementoes as i32, 0x14A);
        assert_eq!(StageID::ResultStage_Jack as i32, 0x14B);
        assert_eq!(StageID::Brave_Altar as i32, 0x14C);
        assert_eq!(StageID::End_Brave_Altar as i32, 0x14D);
        assert_eq!(StageID::Battle_Brave_Altar as i32, 0x14E);
        assert_eq!(StageID::Buddy_Spiral as i32, 0x14F);
        assert_eq!(StageID::End_Buddy_Spiral as i32, 0x150);
        assert_eq!(StageID::Battle_Buddy_Spiral as i32, 0x151);
        assert_eq!(StageID::Dolly_Stadium as i32, 0x152);
        assert_eq!(StageID::End_Dolly_Stadium as i32, 0x153);
        assert_eq!(StageID::Battle_Dolly_Stadium as i32, 0x154);
        assert_eq!(StageID::FE_Shrine as i32, 0x155);
        assert_eq!(StageID::End_FE_Shrine as i32, 0x156);
        assert_eq!(StageID::Battle_FE_Shrine as i32, 0x157);
        assert_eq!(StageID::Tantan_Spring as i32, 0x158);
        assert_eq!(StageID::End_Tantan_Spring as i32, 0x159);
        assert_eq!(StageID::Battle_Tantan_Spring as i32, 0x15A);
        assert_eq!(StageID::BattleField_S as i32, 0x15B);
        assert_eq!(StageID::Pickel_World as i32, 0x15C);
        assert_eq!(StageID::End_Pickel_World as i32, 0x15D);
        assert_eq!(StageID::Battle_Pickel_World as i32, 0x15E);
        assert_eq!(StageID::FF_Cave as i32, 0x15F);
        assert_eq!(StageID::End_FF_Cave as i32, 0x160);
        assert_eq!(StageID::Battle_FF_Cave as i32, 0x161);
        assert_eq!(StageID::ResultStage_Edge as i32, 0x162);
        assert_eq!(StageID::Xeno_Alst as i32, 0x163);
        assert_eq!(StageID::End_Xeno_Alst as i32, 0x164);
        assert_eq!(StageID::Battle_Xeno_Alst as i32, 0x165);
        assert_eq!(StageID::Demon_Dojo as i32, 0x166);
        assert_eq!(StageID::End_Demon_Dojo as i32, 0x167);
        assert_eq!(StageID::Battle_Demon_Dojo as i32, 0x168);
        assert_eq!(StageID::Trail_Castle as i32, 0x169);
        assert_eq!(StageID::End_Trail_Castle as i32, 0x16A);
        assert_eq!(StageID::Battle_Trail_Castle as i32, 0x16B);
    }
}
