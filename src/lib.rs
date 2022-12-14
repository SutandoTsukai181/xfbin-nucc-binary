mod nucc_binary_parsed;
mod utils;

use deku::ctx::Endian;
use regex::Regex;
use strum::{EnumMessage, IntoEnumIterator};
use strum_macros::{Display, EnumIter, EnumString};

pub use nucc_binary_parsed::*;

#[derive(Copy, Clone, EnumIter, Display, EnumString)]
pub enum NuccBinaryType {
    CharaCode(Endian),
    DDS,
    Ev(Endian),
    FCV,
    LUA,
    MessageInfo(Endian),
    PlayerColorParam(Endian),
    PNG,
    PrmLoad(Endian),
    SoundTestParam(Endian),
    StageInfo(Endian),
    XML,
}

impl NuccBinaryType {
    pub fn patterns(&self) -> Vec<(Regex, Endian)> {
        match self {
            NuccBinaryType::CharaCode(_) => {
                vec![(Regex::new(r"(characode\.bin)$").unwrap(), Endian::Little)]
            }
            NuccBinaryType::DDS => {
                vec![(Regex::new(r"(\.dds)$").unwrap(), Endian::Little)]
            }
            NuccBinaryType::Ev(_) => {
                vec![(Regex::new(r"(_ev\.bin)$").unwrap(), Endian::Little)]
            }
            NuccBinaryType::FCV => {
                vec![(Regex::new(r"(\.fcv)$").unwrap(), Endian::Little)]
            }
            NuccBinaryType::LUA => {
                vec![(Regex::new(r"(\.lua)$").unwrap(), Endian::Little)]
            }
            NuccBinaryType::MessageInfo(_) => {
                vec![
                    (
                        Regex::new(r"((WIN(32|64)|PS4).*?/message.*?\.bin)$").unwrap(),
                        Endian::Little,
                    ),
                    // (
                    //     Regex::new(r"(PS3.*?/message.*?\.bin)$").unwrap(),
                    //     Endian::Big,
                    // ),
                ]
            }
            NuccBinaryType::PlayerColorParam(_) => {
                vec![(
                    Regex::new(r"(PlayerColorParam\.bin)$").unwrap(),
                    Endian::Little,
                )]
            }
            NuccBinaryType::PNG => {
                vec![(Regex::new(r"(\.png)$").unwrap(), Endian::Little)]
            }
            NuccBinaryType::PrmLoad(_) => {
                vec![(Regex::new(r"(prm_load\.bin)$").unwrap(), Endian::Little)]
            }
            NuccBinaryType::SoundTestParam(_) => {
                vec![(
                    Regex::new(r"(SoundTestParam\.bin)$").unwrap(),
                    Endian::Little,
                )]
            }
            NuccBinaryType::StageInfo(_) => {
                vec![(Regex::new(r"(stageInfo\.bin)$").unwrap(), Endian::Little)]
            }
            NuccBinaryType::XML => {
                vec![(Regex::new(r"(\.xml)$").unwrap(), Endian::Little)]
            }
        }
    }

    pub fn examples(&self) -> Vec<String> {
        match self {
            NuccBinaryType::CharaCode(_) => {
                vec![String::from(
                    "D:/JARP/param/player/Converter/bin/characode.bin",
                )]
            }
            NuccBinaryType::DDS => {
                vec![String::from("Z:/STORM4_UI_DATA/charsel/charsel_I3.dds")]
            }
            NuccBinaryType::Ev(_) => {
                vec![String::from("player/1dio01_ev/1dio01_ev.bin")]
            }
            NuccBinaryType::FCV => {
                vec![String::from(
                    "J:/char/c/0bao01/anm/ghh1/fcv/0bao01ghh1_cut2_bright_rate.fcv",
                )]
            }
            NuccBinaryType::LUA => {
                vec![String::from("d01/d01_010.lua")]
            }
            NuccBinaryType::MessageInfo(_) => {
                vec![
                    String::from("WIN64/eng/message_DLC110.bin"),
                    // String::from("PS3//eng//messageInfo.bin"),
                ]
            }
            NuccBinaryType::PlayerColorParam(_) => {
                vec![String::from("PlayerColorParam.bin")]
            }
            NuccBinaryType::PNG => {
                vec![String::from("Z:/char/x/duel_item/tex/c_bat_067.png")]
            }
            NuccBinaryType::PrmLoad(_) => {
                vec![String::from(
                    "D:/JARP/trunk/param/player/Converter/bin/0bao01/0bao01prm_load.bin",
                )]
            }
            NuccBinaryType::SoundTestParam(_) => {
                vec![String::from("SoundTestParam.bin")]
            }
            NuccBinaryType::StageInfo(_) => {
                vec![String::from("stageInfo.bin")]
            }
            NuccBinaryType::XML => {
                vec![String::from("D:/JARP/trunk/param/spm/spm/0bao01_SPM.xml")]
            }
        }
    }

    pub fn version_options(&self) -> Vec<String> {
        match self {
            NuccBinaryType::Ev(_) => EvVersion::iter()
                .map(|version| {
                    format!(
                        "{} ({})",
                        version,
                        version.get_documentation().unwrap_or_default()
                    )
                })
                .collect(),
            _ => vec![],
        }
    }
}
