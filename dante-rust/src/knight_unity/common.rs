#![allow(dead_code)]

use binrw::binrw;
use derive_builder::Builder;
use std::fmt::{self, Debug, Display, Formatter};

pub const PACKET_HEADER: [u8; 2] = [0x55, 0xaa];
pub const PACKET_FOOTER: [u8; 2] = [0xaa, 0x55];

#[binrw]
#[brw(repr=u32)]
#[derive(Debug, Clone, Copy, Default)]
pub enum Nation {
    #[default]
    Unselected = 0,
    Karus = 1,
    Elmorad = 2,
}

#[binrw]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Default)]
#[brw(repr(u8))]
pub enum Class {
    Warrior = 1,
    Rogue = 2,
    #[default]
    Mage = 3,
    Priest = 4,
}

pub enum KarusRace {
    ArchTuarek = 1,
    Tuarek = 2,
    WrinkleTuarek = 3,
    PuryTuarek = 4,
}

// Implement from for Class
impl From<u8> for Class {
    fn from(val: u8) -> Self {
        match val {
            1 => Self::Warrior,
            2 => Self::Rogue,
            3 => Self::Mage,
            4 => Self::Priest,
            _ => panic!("Invalid class value: {}", val),
        }
    }
}

// Implement into for Class

#[binrw]
#[derive(Default, Clone, Builder, Debug)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[binrw]
#[derive(Default, Clone)]
pub struct KStr {
    len: u16,
    #[br(count=len)]
    str: Vec<u16>,
}

impl Display for KStr {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let f2 = String::from_utf16(self.str.as_slice());
        if f2.is_err() {
            return write!(f, "Invalid UTF-16");
        }
        write!(f, "{}", f2.unwrap())
    }
}

// Implement Debug display
impl std::fmt::Debug for KStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let f2 = String::from_utf16(self.str.as_slice());
        if f2.is_err() {
            return write!(f, "Invalid UTF-16");
        }
        write!(f, "{}", String::from_utf16(self.str.as_slice()).unwrap())
    }
}

impl From<KStr> for String {
    fn from(kstr: KStr) -> Self {
        let f2 = String::from_utf16(kstr.str.as_slice());
        if f2.is_err() {
            return String::from("Invalid K");
        }
        String::from_utf16(kstr.str.as_slice()).unwrap()
    }
}

impl From<String> for KStr {
    fn from(s: String) -> Self {
        let str = s.encode_utf16().collect::<Vec<u16>>();
        let len = str.len() as u16;
        Self { len, str }
    }
}

impl From<&str> for KStr {
    fn from(s: &str) -> Self {
        let str = s.encode_utf16().collect::<Vec<u16>>();
        let len = str.len() as u16;
        Self { len, str }
    }
}

#[binrw]
#[derive(Default)]
pub struct Bool {
    #[br(map = |x: u8| x > 0)]
    #[bw(map = |x: &bool| *x as u8)]
    val: bool,
}
// Impl into bool for Mybool
impl From<Bool> for bool {
    fn from(mybool: Bool) -> Self {
        mybool.val
    }
}

// Impl Clone for Bool
impl Clone for Bool {
    fn clone(&self) -> Self {
        Self { val: self.val }
    }
}

// Impl copy for Bool
impl Copy for Bool {}

// Impl Default for Boo

// Impl PartialEq for Bool
impl PartialEq for Bool {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}
// Impl Eq for Bool
impl Eq for Bool {}

// Impl Display for Bool
impl Display for Bool {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}

// Impl From<bool> for Bool
impl From<bool> for Bool {
    fn from(val: bool) -> Self {
        Self { val }
    }
}

impl Debug for Bool {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)
    }
}
