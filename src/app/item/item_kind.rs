/// A numeric identifier for an item.
#[repr(C)]
pub enum ItemKind {
    None = -1,

    /// Assist Trophy
    Assist,

    /// Barrel
    Barrel,

    /// Crate
    Box,

    /// Capsule
    Capsule,

    /// Rolling Crate
    Carrierbox,

    /// Party Ball
    Kusudama,

    /// Poké Ball
    Pokeball,

    /// Master Ball
    Masterball,

    /// Sandbag
    Sandbag,

    /// Grass
    Grass,

    /// Maxim Tomato
    Maximtomato,

    /// Heart Container
    Heart,

    /// Food
    Tabemono,

    /// Fairy Bottle
    Fairybottle,

    /// Peach (Peach Blossom)
    Finalpeach,

    /// Daisy (Daisy Blossom)
    Finaldaisy,

    /// Healing Sprout
    Healball,

    /// Healing Field
    Teamhealfield,

    /// Superspicy Curry
    Curry,

    /// Superspicy Curry (Projectile)
    Curryshot,

    /// Metal Box
    Metalblock,

    /// Poison Mushroom
    Mushd,

    /// Super Mushroom
    Mushroom,

    /// Timer
    Slow,

    /// Super Star
    Superstar,

    /// Lightning Bolt
    Thunder,

    /// Warp Star
    Warpstar,

    /// Fire Flower
    Fireflower,

    /// Fire Flower (Projectile)
    Fireflowershot,

    /// Ray Gun
    Raygun,

    /// Ray Gun (Projectile)
    Raygunshot,

    /// Super Scope
    Superscope,

    /// Super Scope (Projectile)
    Superscopeshot,

    /// Drill
    Drill,

    /// Drill (Projectile)
    Drillshot,

    /// Steel Diver
    Steeldiver,

    /// Steel Diver (Projectile)
    Steeldivershot,

    /// Gust Bellows
    Magicpot,
    Magicpotwind,

    /// Rage Blaster
    Revengeshooter,

    /// Rage Blaster (Projectile)
    Revengeshootershot,

    /// Banana Gun
    Bananagun,

    /// Banana Gun (Projectile)
    Bananagunshot,

    /// Staff
    Staff,

    /// Staff (Projectile)
    Staffshot,

    Staffroulette,
    Staffrouletteshot,

    /// Ramblin' Evil Mushroom
    Walkmush,
    Walkmushflower,
    Walkmushshot,

    /// Beam Sword
    Beamsword,

    /// Golden Hammer
    Goldenhammer,

    /// Hammer
    Hammer,

    /// Hammer (Head)
    Hammerhead,

    /// Home-Run Bat
    Homerunbat,

    /// Lip's Stick
    Ripstick,
    Ripstickflower,
    Ripstickshot,

    /// Star Rod
    Starrod,

    /// Star Rod (Projectile)
    Starrodshot,

    /// Ore Club
    Club,

    /// Ore Club (Projectile)
    Clubshot,

    /// Fire Bar
    Firebar,

    /// Killing Edge
    Killsword,

    /// Death's Scythe
    Deathscythe,

    /// Banana Peel
    Banana,

    /// Bob-omb
    Bombhei,

    /// Bumper
    Bumper,

    /// Gooey Bomb
    Chewing,

    /// Deku Nut
    Deku,

    /// Mr. Saturn
    Doseisan,

    /// Freezie
    Freezer,

    /// Green Shell
    Greenshell,

    /// Hothead
    Pasaran,

    /// Pitfall
    Pitfall,

    /// Motion-Sensor Bomb
    Sensorbomb,

    /// Smart Bomb
    Smartbomb,

    /// Smoke Ball
    Smokescreen,

    /// X Bomb
    Crossbomb,

    /// Hocotate Bomb
    Dolphinbomb,

    /// Boomerang
    Boomerang,

    /// Beehive
    Honeycomb,

    /// Beehive (Bee)
    Honeycombbee,

    /// Killer Eye
    Killereye,

    /// Killer Eye (Projectile)
    Killereyeshot,

    /// Bombchu
    Bombchu,

    /// Spiny Shell
    Togezoshell,

    /// Boss Galaga
    Bossgalaga,

    /// Beetle
    Beetle,

    /// POW Block
    Powblock,

    /// Super Launch Star
    Starring,

    /// Black Hole
    Blackball,

    /// Beastball
    Magicball,

    /// Soccer Ball
    Soccerball,

    /// Unira
    Unira,

    /// Franklin Badge
    Badge,

    /// Screw Attack
    Screw,

    /// Bunny Hood
    Usagihat,

    /// Super Leaf
    Superleaf,

    /// Super Leaf: Tail
    Superleaftail,

    /// Back Shield
    Backshield,

    /// Rocket Belt
    Rocketbelt,

    /// Instant Drop (Smash Run)
    Machstamp,

    /// Blast Box
    Powderbox,

    /// Smash Ball
    Smashball,

    /// Dragoon Parts
    Dragoon,

    /// Dragoon
    Dragoonset,

    /// Dragoon: Reticle
    Dragoonsight,

    /// Bullet Bill
    Killer,

    /// Bullet Bill (Transformation)
    Killertransform,

    /// Daybreak Parts
    Genesis,

    /// Daybreak
    Genesisset,

    /// Cucco
    Chicken,
    Chickenparty,

    /// Special Flag
    Specialflag,

    /// Fake Smash Ball
    Smashbomb,

    /// Bomber
    Bomber,

    Smashballheavy,
    Smashbombheavy,

    /// Peanut (Diddy Kong)
    Diddypeanuts,

    /// Remote Bomb (Link)
    Linkbomb,

    /// Vegetable (Peach)
    Peachdaikon,

    /// Gyro (R.O.B.)
    Robotgyro,

    /// Bomb (Toon Link)
    Toonlinkbomb,

    /// Wario Bike (Wario)
    Wariobike,

    /// Wario Bike (Wario): Engine
    Wariobikea,

    /// Wario Bike (Wario): Fork
    Wariobikeb,

    /// Wario Bike (Wario): Seat
    Wariobikec,

    /// Wario Bike (Wario): Front Wheel
    Wariobiked,

    /// Wario Bike (Wario): Back Wheel
    Wariobikee,

    /// Burst Grenade (Sheik)
    Explosionbomb,

    /// Tome (Robin)
    Book,

    /// Levin Sword (Robin)
    Thundersword,

    /// Cherry (PAC-MAN)
    Pacmancherry,

    /// Strawberry (PAC-MAN)
    Pacmanstrawberry,

    /// Orange (PAC-MAN)
    Pacmanorange,

    /// Apple (PAC-MAN)
    Pacmanapple,

    /// Melon (PAC-MAN)
    Pacmanmelon,

    /// Galaxian (PAC-MAN)
    Pacmanboss,

    /// Bell (PAC-MAN)
    Pacmanbell,

    /// Key (PAC-MAN)
    Pacmankey,

    /// Power Pellet (PAC-MAN)
    Poweresa,

    /// Metal Blade (Mega Man)
    Metalblade,

    /// Soccer Ball (Wii Fit Trainer)
    Wiifitball,

    /// Log (Greninja)
    Log,

    /// Doll (Greninja)
    Doll,

    /// Mechakoopa (Bowser Jr.)
    Mechakoopa,

    Wood,

    /// Fruit (Villager)
    Murabitofruit,

    /// Crown (King K. Rool)
    Kroolcrown,

    /// Hand Grenade (Snake)
    Snakegrenade,

    /// Cardboard Box (Snake)
    Snakecbox,

    /// Bomb (Young Link)
    Younglinkbomb,

    /// Vegetable (Daisy)
    Daisydaikon,

    /// Holy Water (Simon)
    Simonholywater,

    /// Holy Water (Richter)
    Richterholywater,

    /// Arrow (Link)
    Linkarrow,

    /// Fruit (Tortimer Island)
    StFruit,

    /// Attack Fruit (Tortimer Island)
    StFruitbomb,

    /// Pellet (Distant Planet)
    Pellet,

    /// Sidestepper (Mario Bros.)
    StCrab,

    /// Shellcreeper (Mario Bros.)
    StShell,

    /// Vegetable (Summit)
    StIceYasai,

    /// Apple (Green Greens)
    StApple,

    /// Birdo's Egg (Mushroom Kingdom II)
    StUsaEgg,

    /// Chansey's Egg (Saffron City)
    StYamabukiEgg,

    /// Andross
    Andross,

    /// Andross (Projectile)
    Androssshot,

    /// Samurai Goroh
    Goroh,

    /// Waluigi
    Waluigi,

    /// Starfy
    Stafy,

    /// Devil
    Devil,

    /// Metroid
    Metroid,

    /// Hammer Bro
    Hammerbros,

    /// Hammer Bro: Hammer
    Hammerbroshammer,

    /// Knuckle Joe
    Joe,
    Joeshot,

    /// Jeff
    Jeff,

    /// Jeff: Bottle Rocket (Small Missle)
    Jeffpencilbullet,

    /// Jeff: Bottle Rocket (Large Missle)
    Jeffpencilrocket,

    /// Shadow
    Shadow,

    /// Dr. Wright
    Wright,
    Wrightbuil,

    /// Lyn
    Lin,

    /// nintendog
    Nintendogs,

    /// nintendog (High Resolution)
    Nintendogshi,

    /// Midna
    Midna,

    /// Mother Brain
    Motherbrain,

    /// Mother Brain: Rinka
    Motherbrainshot,

    /// Mother Brain: Laser
    Motherbrainshot2,

    /// Phosphora
    Eleka,
    Elekashot,
    Elekashot2,
    Elekashot3,

    /// Ghirahim
    Ghirahim,
    Ghirahimknife,
    Ghirahimknife2,

    /// Starman
    Starman,

    /// Starman: PK Beam Ω
    Starmanomegabeam,

    /// Starman: PK Beam γ
    Starmangammabeam,

    /// Sheriff
    Sheriff,

    /// Sheriff (Projectile)
    Sheriffshot,

    /// Skull Kid
    Skullkid,

    /// Ashley
    Ashley,

    /// Chain Chomp
    Wanwan,
    Wanwanpile,

    /// Ghosts
    Monsters,
    Monsterschild,

    /// Dr. Kawashima
    Kawashima,
    Kawashimashot,

    /// Riki
    Riki,

    /// Color TV-Game 15
    Tvgame,

    /// Color TV-Game 15: Ball
    Tvgameshot,

    /// Prince of Sablé
    Sable,

    /// Takamaru
    Takamaru,
    Takamarushot,

    /// Nightmare
    Nightmare,

    /// Dillon
    Jiro,

    /// Isaac
    Robin,
    Robinshot,
    Robinpress,
    Robinlift,

    /// Gray Fox
    Cyborg,

    /// Guile
    Guile,
    Guileshot,

    /// Squid Sisters
    Shiokarazu,

    /// Krystal
    Krystal,
    Krystalshot,

    /// Moon
    Moon,

    /// Zero (Mega Man)
    Zero,
    Zeroshot,

    /// Alucard
    Alucard,

    /// Akira
    Akira,

    /// Bomberman
    Bomberman,
    Bombermanbombmanager,
    Bombermanbomb,
    Bombermanbomb2,

    /// Black Knight
    Darkknight,

    /// Burrowing Snagret
    Hebigarasu,

    /// Chef Kawasaki
    Kawasaki,
    Kawasakipot,
    Kawasakishot,

    /// Rathalos
    Lioleus,
    Lioleusshot,

    /// Shovel Knight
    Shovelknight,
    Shovelknightshot,
    Shovelknightpitfall,

    /// Rodin
    Rodin,
    Rodinarmr,
    Rodinarml,
    Rodinarmstraight,
    Rodinarmupper,

    /// Knuckles
    Knuckles,

    /// Yuri Kozukata
    Kozukata,
    Kozukatasight,
    Kozukataghost,

    /// Sukapon
    Sukapon,
    Sukaponshot,

    /// Arcade Bunny
    Baito,

    /// Arcade Bunny: Crane
    Baitocrane,

    /// Klaptrap
    Claptrap,

    /// Kapp'n
    Driver,

    /// Thwomp
    Dossun,

    /// Spring Man
    Springman,
    Springmanshot,

    /// Wily Capsule
    Wilycapsule,
    Wilycapsuleshot,
    Wilycapsuleshot2,

    /// Wily Capsule: Dr. Wily
    Wilycapsulewily,

    /// Flies & Hand
    Flyandhand,

    /// Flies & Hand (Fly)
    Flyandhandfly,

    /// Vince
    Vince,

    /// Tiki
    Tiki,
    Tikishot,

    /// Nikki
    Nikki,

    /// Nikki: Drawing (Monster)
    Nikkiillust1,

    /// Nikki: Drawing (Pinwheel)
    Nikkiillust2,

    /// Nikki: Drawing (Smash Ball)
    Nikkiillust3,

    /// Nikki: Drawing (Ghost)
    Nikkiillust4,

    /// Nikki: Drawing (Bullet Bill)
    Nikkiillust5,

    /// Nikki: Drawing (Bird)
    Nikkiillust6,

    /// Meowth
    Nyarth,
    Nyarthshot,

    /// Electrode
    Marumine,

    /// Goldeen
    Tosakinto,

    /// Staryu
    Hitodeman,
    Hitodemanshot,

    /// Snorlax
    Kabigon,

    /// Togepi
    Togepy,

    /// Bellossom
    Kireihana,

    /// Gardevoir
    Sirnight,

    /// Metagross
    Metagross,

    /// Moltres
    Fire,

    /// Mew
    Mew,

    /// Entei
    Entei,

    /// Suicune
    Suicune,

    /// Lugia
    Lugia,
    Lugiashot,

    /// Latias & Latios
    Latiaslatios,

    /// Kyogre
    Kyogre,
    Kyogreshot,

    /// Deoxys
    Deoxys,

    /// Eevee
    Eievui,

    /// Abomasnow
    Yukinooh,

    /// Fletchling
    Yayakoma,

    /// Spewpa
    Kofuurai,

    /// Inkay
    Maaiika,

    /// Giratina
    Giratina,
    Giratinashot,

    /// Darkrai
    Darkrai,
    Darkraishot,

    /// Zoroark
    Zoroark,

    /// Meloetta
    Meloetta,
    Meloettashot,

    /// Genesect
    Genesect,
    Genesectrapidshot,

    /// Snivy
    Tsutarja,
    Tsutarjashot,

    /// Oshawott
    Mijumaru,

    /// Chespin
    Harimaron,

    /// Fennekin
    Fokko,
    Fokkoshot,

    /// Gogoat
    Gogoat,

    /// Swirlix
    Peroppafu,

    /// Dedenne
    Dedenne,
    Dedenneshot,

    /// Palkia
    Palkia,

    /// Arceus
    Arceus,

    /// Victini
    Victini,

    /// Kyurem
    Kyurem,

    /// Keldeo
    Keldeo,

    /// Xerneas
    Xerneas,

    /// Ditto
    Metamon,

    /// Alolan Vulpix
    Rokona,
    Rokonashot,

    /// Alolan Exeggutor
    Nassya,

    /// Bewear
    Kiteruguma,

    /// Mimikyu
    Mimikkyu,

    /// Solgaleo
    Solgaleo,

    /// Lunala
    Lunala,
    Lunalashot,

    /// Vulpix
    Rokon,
    Rokonshot,

    /// Alolan Raichu
    Raichua,

    /// Pyukumuku
    Namakobushi,

    /// Togedemaru
    Togedemaru,
    Togedemarushot,

    /// Abra
    Kasey,

    /// Tapu Koko
    Kapukokeko,
    Kapukokekoshot,

    /// Marshadow
    Marshadow,

    /// Scizor
    Hassam,

    /// Master Hand
    Masterhand,
    Masterhandbullet,
    Masterhandironball,
    Masterhandpaintball,
    Masterhandchakram,
    Masterhandkenzan,
    Masterhandenergyshot,
    Masterhandsatellite,
    Masterhandsatelliteshot,

    /// Crazy Hand
    Crazyhand,
    Crazyhandbomb,
    Crazyhandfire,
    Crazyhandsearchlight,
    Crazyhandgravityball,
    Crazyhandshockwave,

    /// Galleom
    Galleom,
    Galleommissile,
    Galleomshockwave,

    /// Ganon, The Demon King
    Ganonboss,
    Ganonbossshot,
    Ganonbossshot2,

    /// Dracula
    Dracula,
    Dracula3way,
    Draculastraight,
    Draculapillar,
    Draculafill,
    Draculabat,

    /// Dracula (2nd Form)
    Dracula2,
    Dracula2fireshot,
    Dracula2homingshot,
    Dracula2shockwave,
    Dracula2squashshockwave,

    /// Marx
    Marx,
    Marxcutter,
    Marxplantseed,
    Marxplantbody,
    Marxicebomb1,
    Marxicebomb2,
    Marxchaseeye,
    Marxeyelaser,

    /// Rathalos
    Lioleusboss,
    Lioleusbossshot,
    Lioleusbossshot2,
    Lioleusbossbarrel,
    Lioleusbossbombhei,
    Lioleusbosssmartbomb,

    /// Giga Bowser
    Koopag,

    /// Galeem
    Kiila,
    Kiilacrossbomb,
    Kiilacrushwing,
    Kiilalaser,
    Kiilasmartbomb,
    Kiilaexplodeshot1,
    Kiilaexplodeshot2,
    Kiilashockwave,
    Kiilastraightshot,
    Kiilacore,
    Kiilatorrent,

    /// Dharkon
    Darz,
    Darzcrossbomb,
    Darzgatlingshot,
    Darztentacleshot,
    Darzspacerush,
    Darzspacerushstencil,
    Darztearup,
    Darzcentipede,
    Darzpillar,
    Darzpierce,
    Darztorrent,

    Kiiladarzmanager,

    PlayableMasterhand,
    PlayableMasterhandbullet,
    PlayableMasterhandironball,
    PlayableMasterhandchakram,

    /// Sandbag (Home-Run Contest)
    Sandbaghr,

    /// Home-Run Bat (Home-Run Contest)
    Homerunbathr,

    /// Reward (Home-Run Contest)
    Prizehr,

    /// Grenade Egg (Banjo & Kazooie)
    Buddybomb,

    /// Block (Steve)
    Pickelobject,

    /// TNT Block (Steve)
    Pickelbomb,
}
