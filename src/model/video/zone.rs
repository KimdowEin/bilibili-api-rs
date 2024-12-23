//! 视频分区


use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Zone {
    Douga(Douga),
    Anime(Anime),
    Guochuang(Guochuang),
    Music(Music),
    Dance(Dance),
    Game(Game),
    Knowledge(Knowledge),
    Tech(Tech),
    Sports(Sports),
    Car(Car),
    Life(Life),
    Food(Food),
    Ent(Ent),
    Kichiku(Kichiku),
    Fashion(Fashion),
    Information(Information),
    Animal(Animal),
    Cinephile(Cinephile),
    Documentary(Documentary),
    Movie(Movie),
    TV(TV),
    Unknown,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(i32)]
pub enum Douga {
    Douga = 1,
    Mad = 24,
    Mmd = 25,
    Handdraw = 47,
    Voice = 257,
    GarageKit = 210,
    Tokusatsu = 86,
    Acgntalks = 253,
    Other = 27,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(i32)]
pub enum Anime {
    Anime = 13,
    Information = 51,
    Offical = 152,
    Finish = 32,
    Serial = 33,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(i32)]
pub enum Guochuang {
    Guochuang = 167,
    Chinese = 153,
    Original = 168,
    Puppetry = 169,
    Information = 170,
    Motioncomic = 195,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(i32)]
pub enum Music {
    Music = 3,
    Original = 28,
    Cover = 31,
    Vocaloid = 30,
    Perform = 59,
    Mv = 193,
    Live = 29,
    Other = 130,
    Commentary = 243,
    Tutorial = 244,
    EleMusic = 194,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(i32)]
pub enum Dance {
    Dance = 129,
    Otaku = 20,
    ThreeD = 154,
    Demo = 156,
    Hiphop = 198,
    Star = 199,
    China = 200,
    Gestures = 255,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(i32)]
pub enum Game {
    Game = 4,
    StandAlone = 17,
    Esports = 171,
    Mobile = 172,
    Online = 65,
    Board = 173,
    Gmv = 121,
    Music = 136,
    Mugen = 19,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(i32)]
pub enum Knowledge {
    Knowledge = 36,
    Science = 201,
    SocialScience = 124,
    HumanityHistory = 228,
    Finance = 207,
    Campus = 208,
    Career = 209,
    Design = 229,
    Skill = 122,
    Speech = 39,
    War = 96,
    Machine = 98,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(i32)]
pub enum Tech {
    Tech = 188,
    Digital = 95,
    Application = 230,
    ComputerTech = 231,
    Industry = 232,
    Diy = 233,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(i32)]
pub enum Sports {
    Sports = 234,
    Basketball = 235,
    Football = 249,
    Aerobics = 164,
    Athletic = 236,
    Culture = 237,
    Comprehensive = 238,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(i32)]
pub enum Car {
    Car = 223,
    Knowledge = 258,
    Racing = 245,
    ModifiedVehicle = 246,
    NewEnergyVehicle = 247,
    TouringCar = 248,
    Motorcycle = 240,
    Strategy = 227,
    Life = 176,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(i32)]
pub enum Life {
    Life = 160,
    Funny = 138,
    Travel = 250,
    Rurallife = 251,
    Home = 239,
    Handmake = 161,
    Painting = 162,
    Daily = 21,
    Parenting = 254,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(i32)]
pub enum Food {
    Food = 211,
    Make = 76,
    Detective = 212,
    Measurement = 213,
    Rural = 214,
    Record = 215,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(i32)]
pub enum Animal {
    Animal = 217,
    Cat = 218,
    Dog = 219,
    SecondEdition = 220,
    WildAnimal = 221,
    Reptiles = 222,
    AnimalComposite = 75,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(i32)]
pub enum Kichiku {
    Kichiku = 119,
    Guide = 22,
    Mad = 26,
    ManualVocaloid = 126,
    Theatre = 216,
    Course = 127,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(i32)]
pub enum Fashion {
    Fashion = 155,
    Makeup = 157,
    Cos = 252,
    Clothing = 158,
    Catwalk = 159,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(i32)]
pub enum Information {
    Information = 202,
    Hotspot = 203,
    Global = 204,
    Social = 205,
    Multiple = 206,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(i32)]
pub enum Ent {
    Ent = 5,
    Variety = 71,
    Talker = 241,
    Fans = 242,
    Celebrity = 137,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(i32)]
pub enum Cinephile {
    Cinephile = 181,
    Cinecism = 182,
    Montage = 183,
    Shortfilm = 85,
    TrailerInfo = 184,
    Shortfilm2 = 256,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(i32)]
pub enum Documentary {
    Documentary = 177,
    History = 37,
    Science = 178,
    Military = 179,
    Travel = 180,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(i32)]
pub enum Movie {
    Movie = 23,
    Chinese = 147,
    West = 145,
    Japan = 146,
    Other = 83,
}

#[derive(Debug, Clone, Serialize_repr, Deserialize_repr, PartialEq)]
#[repr(i32)]
pub enum TV {
    TV = 11,
    Mainland = 185,
    Overseas = 187,
}

