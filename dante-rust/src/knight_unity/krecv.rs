#![allow(dead_code)]

use crate::knight_unity::common::{Bool, Class, KStr, Nation, Vector3};

use binrw::{binrw, BinRead, BinWrite};
use derive_builder::Builder;
use log::info;
use std::{io::Cursor};
// use tokio::{
//     io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
//     net::{tcp::OwnedWriteHalf, TcpStream},
// };
#[binrw::binrw]
#[derive(Debug)]
#[br(assert(footer == 0xaa55u16 && header == 0x55AAu16))]
pub struct KRecv {
    header: u16,
    data_size: u16,
    #[br(count=data_size)]
    inner_data: Vec<u8>,
    footer: u16,
}

// Definition of all struct with magic values are included
#[binrw]
// #[derive(Debug)]
pub enum KRecvInner {
    #[br(magic(1u16))]
    Login(Login),

    #[br(magic(2u16))]
    ServerList(ServerList),

    #[br(magic(0x0au16))]
    NationSelectEvent(NationSelectEvent),

    #[br(magic(0x0bu16))]
    CharacterCreate(CharacterCreate),

    #[br(magic(0x0cu16))]
    CharacterSelect(CharacterSelect),

    #[br(magic(0x0du16))]
    CharacterSelectInfo(CharacterSelectInfos),

    #[br(magic(0x14u16))]
    GameStart(GameStart),

    #[br(magic(0x15u16))]
    MyInfo(MyInfo),

    #[br(magic(0x16u16))]
    OtherInfo(OtherInfo),

    #[br(magic(0x17u16))]
    UnitDestroy(UnitDestroy),

    #[br(magic(0x18u16))]
    UnitDead(UnitDead),

    #[br(magic(0x19u16))]
    UnitDestroyAll(UnitDestroyAll),

    #[br(magic(0x1eu16))]
    Move(Move),

    #[br(magic(0x1fu16))]
    BasicAttack(BasicAttack),

    #[br(magic(0x20u16))]
    UseMagicEvent(UseMagicEvent),

    #[br(magic(0x21u16))]
    Buff(Buff),

    #[br(magic(0x25u16))]
    TargetInfo(TargetInfo),

    #[br(magic(0x26u16))]
    OtherPlayerInfo(OtherPlayerInfo),

    #[br(magic(0x22u16))]
    BasicNpcAttack(BasicNpcAttack),

    #[br(magic(0x24u16))]
    SelectTarget(SelectTarget),

    #[br(magic(0x32u16))]
    InventorySlotUpdate(InventorySlotUpdate),

    #[br(magic(0x35u16))]
    Warehouse(Warehouse),

    #[br(magic(0x39u16))]
    EquipmentVisualUpdate(EquipmentVisualUpdate),

    #[br(magic(0x3au16))]
    CospreVisualToggle(CospreVisualToggle),

    #[br(magic(0x3cu16))]
    DropLoot(DropLoot),

    #[br(magic(0x3du16))]
    DropLootItemList(DropLootItemList),

    #[br(magic(0x46u16))]
    PartyPacket(PartyPacket),

    #[br(magic(0x47u16))]
    Friend(Friend),

    #[br(magic(0x48u16))]
    SeekingParty(SeekingParty),

    #[br(magic(0x50u16))]
    ChatNormal(ChatNormal),

    #[br(magic(0x51u16))]
    ChatInfo(ChatInfo),

    #[br(magic(0x52u16))]
    ChatNotice(ChatNotice),

    #[br(magic(0x53u16))]
    ChatWhisper(ChatWhisper),

    #[br(magic(0x64u16))]
    Warp(Warp),

    #[br(magic(0x65u16))]
    Teleport(Teleport),

    #[br(magic(0x6fu16))]
    Merchant(Merchant),

    #[br(magic(0x71u16))]
    Slave(Slave),

    #[br(magic(0x82u16))]
    SkillShortcuts(SkillShortcuts),

    #[br(magic(0x8cu16))]
    ItemUpgrade(ItemUpgrade),

    #[br(magic(0x8du16))]
    MagicAnvil(MagicAnvil),

    #[br(magic(0x96u16))]
    NpcInteraction(NpcInteraction),

    #[br(magic(0x97u16))]
    NpcTrading(NpcTrading),

    #[br(magic(0xa0u16))]
    Quest(Quest),

    #[br(magic(0xb4u16))]
    Damage(Damage),

    #[br(magic(0xbeu16))]
    Premium(Premium),

    #[br(magic(0xd2u16))]
    ZoneSettings(ZoneSettings),

    #[br(magic(0xdcu16))]
    ExchangeItem(ExchangeItem),

    #[br(magic(0x118u16))]
    NoahKnights(NoahKnights),

    #[br(magic(0x119u16))]
    ChickenSystem(ChickenSystem),

    // #[br(magic(0x1a4u16))]
    // Maintanence(Maintanence),
    #[br(magic(0x1c2u16))]
    MerchantItemSearch(MerchantItemSearch),

    #[br(magic(0x14au16))]
    LevelGift(LevelGift),

    #[br(magic(0x17cu16))]
    ItemBaseUpdate(ItemBaseUpdate),

    #[br(magic(0x186u16))]
    MilitaryCamp(MilitaryCamp),

    #[br(magic(0x19au16))]
    GeniePackets(GeniePackets),

    #[br(magic(0x1f4u16))]
    QueuePacket(QueuePacket),

    #[br(magic(0x1feu16))]
    PingPong(PingPong),

    #[br(magic(0x208u16))]
    Gamble(Gamble),

    #[br(magic(0x212u16))]
    Robot(Robot),

    // #[br(magic(0x21cu16))]
    // DeleteCharacter(DeleteCharacter),
    #[br(magic(0x226u16))]
    CollectionRace(CollectionRace),

    #[br(magic(0x230u16))]
    ClanNts(ClanNts),

    #[br(magic(0x23au16))]
    Gender(Gender),

    #[br(magic(0x244u16))]
    SpecialGift(SpecialGift),

    #[br(magic(0x24eu16))]
    SafeDevice(SafeDevice),

    // #[br(magic(0x258u16))]
    // DeviceInfo(DeviceInfo),
    #[br(magic(0x262u16))]
    Map(Map),

    #[br(magic(0x26cu16))]
    CharacterSeal(CharacterSeal),

    #[br(magic(0x4e20u16))]
    FieldChanged(FieldChanged),

    #[br(magic(0xf3b4u16))]
    PingPongy(PingPongy),
}

#[binrw]
#[derive(Debug, Clone)]
pub struct ServerInfo {
    idx: u32,
    pub server_name: KStr,
    pub server_ip: KStr,
    pub port: u16,
    w: i32,
}

#[binrw]
#[derive(Debug)]
#[br(assert(footer == 0xaa55u16 && header == 0x55AAu16))]
pub struct KRecvInit {
    header: u16,
    data_size: u16,
    #[br(count=data_size)]
    inner_data: Vec<u8>,
    footer: u16,
}

#[binrw]
#[derive(Debug, Clone)]
pub enum KRecvInitInner {
    #[br(magic(1u16))]
    Login {
        err: u32,
        #[br(if(err != 2))]
        token: KStr,
    },
    #[br(magic(2u16))]
    ServerList {
        server_count: u32,
        #[br(count=server_count)]
        servers: Vec<ServerInfo>,
    },

    #[br(magic(5u16))]
    Otp {},

    #[br(magic(0x3a98u16))]
    MessageBox {
        title: KStr,
        msg: KStr,
        msg_box_type: u32,
    },
    #[br(magic(0xf3b4u16))]
    EndSocket {},
}

impl KRecvInit {
    pub fn generate_from_inner(inner_data: Vec<u8>) -> Self {
        Self {
            header: 0x55AA,
            data_size: inner_data.len() as u16,
            inner_data,
            footer: 0xAA55,
        }
    }
    pub fn parse_inner(&mut self) -> Result<KRecvInitInner, binrw::Error>{
        let res = KRecvInitInner::read_le(&mut Cursor::new(&mut self.inner_data));
        match res {
            Ok(res) => Ok(res),
            Err(e) => {
                info!("Error parsing KRecvInitInner: {:?}", e);
                Err(e)
            }
        }
    }
    pub fn to_buf(self) -> Vec<u8> {
        let mut writer = Cursor::new(Vec::new());
        self.write_le(&mut writer).unwrap();
        writer.into_inner()
    }
}

impl KRecv {
    pub fn generate_from_inner(inner_data: Vec<u8>) -> Self {
        Self {
            header: 0x55AA,
            data_size: inner_data.len() as u16,
            inner_data,
            footer: 0xAA55,
        }
    }
    pub fn parse_inner(&mut self) -> Result<KRecvInner, binrw::Error> {
        KRecvInner::read_le(&mut Cursor::new(&mut self.inner_data))
    }
    pub fn to_buf(self) -> Vec<u8> {
        let mut writer = Cursor::new(Vec::new());
        self.write_le(&mut writer).unwrap();
        writer.into_inner()
    }
}

#[binrw]
#[derive(Debug, Default, Builder)]
pub struct Login {
    isok: u32,
}
#[binrw]
// #[derive(Debug)]
pub struct ServerList {
    server_count: u32,
    #[br(count=server_count)]
    servers: Vec<Server>,
}

#[binrw]
#[derive(Debug)]
pub struct Otp;

#[binrw]
#[derive(Debug)]
pub struct Server {
    idx: u32,
    server_name: KStr,
    server_ip: KStr,
    w: u16,
}

#[binrw]
#[derive(Debug)]
pub struct NationSelectEvent {
    pub nation: Nation,
}

#[binrw]
#[derive(Debug)]
pub struct ClanData {
    clan_id: u32,
    clan_name: KStr,
    clan_grade: u32,
    cape_id: u32,
    cape_frame_id: u32,
    cape_color: u32,
    clan_symbol_id: u32,
    is_rank_5_clan: Bool,
    fame: u8,
}

#[binrw]
#[derive(Debug)]
pub struct EquippedItem {
    idx: u8,
    item_id: u32,
}

#[binrw]
#[derive(Debug)]
pub struct UserItemData {
    pub item_id: u32,
    pub count: u16,
    pub durability: u16,
    pub flag: u8,
    pub expire_time: u32,
    pub serial: u64,
}

#[binrw]
#[derive(Debug)]
pub struct PlayerBase {
    name: KStr,
    nation: u8,
    race: u8,
    pub class: Class,
    pub level: u8,
    s1: u32,
    pub pos: Vector3,
    target_position: Vector3,
    movespeed: f32,
    curr_health: u32,
    max_health: u32,
    curr_mana: u32,
    max_mana: u32,
    is_in_clan: Bool,
    #[br(if(is_in_clan.into()))]
    clan_data: Option<ClanData>,
    is_seeking_party: Bool,
    n_of_equipped_items: u8,
    #[br(count=n_of_equipped_items)]
    eitems: Vec<EquippedItem>,
    current_stealth_state: Bool,
    chicken_active: Bool,
    noah_knights_user: Bool,
    #[br(if(noah_knights_user.into()))]
    noah_knight_name: Option<KStr>,
    rank: u16,
    is_party_leader: Bool,
    title: KStr,
    title_color: u32,
    hide_cospre_armor: Bool,
    hide_cospre_helmet: Bool,
    genie_active: Bool,
    ne_bu: f32,
}

#[binrw]
#[derive(Debug)]
pub struct ZoneSettings {
    pub zone_name: KStr,
    pub can_town: Bool,
    pub can_talk_to_other_nation: Bool,
    pub can_attack_other_nation: Bool,
    pub can_attack_same_nation: Bool,
    pub can_attack_other_nation_npc: Bool,
    pub can_attack_same_nation_npc: Bool,
    pub can_merchant: Bool,
    pub can_trade_other_nation: Bool,
    pub can_trade_same_nation: Bool,
    pub rankable_zone: Bool,
    pub is_clan_war_area: Bool,
}

#[binrw]
#[derive(Debug)]
pub struct MyInfo {
    pub s0: u32,
    pub socket_id: u32,
    pub _char: PlayerBase,
    pub loyalty: u32,
    pub loyalty_monthly: u32,
    pub manner_point: u32,
    pub attack: u32,
    pub defense: u32,
    pub fire_damage: u32,
    pub ice_damage: u32,
    pub lightning_damage: u32,
    pub poison_damage: u32,
    pub fire_resist: u32,
    pub ice_resist: u32,
    pub lightning_resist: u32,
    pub poison_resist: u32,
    pub free_stat_points: u16,
    pub stats: [u8; 5],
    pub bonus_stats: [u16; 5],
    pub free_skills: u8,
    pub skill_points: [u8; 4],
    pub current_exp: u64,
    pub max_exp: u64,
    pub inventory_size: u32,
    #[br(count=inventory_size)]
    pub inventory: Vec<UserItemData>,
    pub magic_bag_expire_time: u32,
    pub vip_house_expire_time: u32,
    pub genie_time: u16,
}

#[binrw]
#[derive(Debug)]
pub struct GeniePackets {
    genie_op: GenieOp,
}

#[binrw]
#[derive(Debug)]
pub enum GenieOp {
    #[br(magic(0x00u32))]
    Start { player_id: u32 },
    #[br(magic(0x01u32))]
    Stop { player_id: u32 },
    #[br(magic(0x02u32))]
    TimeUpdate { genie_time: u16 },
    #[br(magic(0x03u32))]
    UseGenieItem { item_id: u32 },
    #[br(magic(0x04u32))]
    AutoUseGenieToggle {},
}

#[binrw]
#[derive(Debug)]
pub struct MerchantItemSearch {
    merch_search_op: KStr,
    merch_search_op2: KStr,
    merch_search_op3: KStr,
    merch_search_op4: KStr,
    merch_search_op5: KStr,
}

#[binrw]
#[derive(Debug)]
pub enum MerchantItemSearchOp {
    #[br(magic(0x00u32))]
    Test {},
}

#[binrw]
#[derive(Debug)]
pub struct CharacterCreate {
    err: ErrorCodes,
}

#[binrw]
#[derive(Debug)]
#[brw(repr=i32)]
pub enum ErrorCodes {
    Success = 1,
    InvalidRace = -1,
    InvalidClass = -2,
    InvalidStats = -3,
    InvalidStats2 = -4,
    InvalidCharacterName = -5,
}

#[binrw]
#[derive(Debug)]
pub struct CharacterSelect {
    res: u32,
    #[br(if(res >= 1))]
    zone_id: u32,
}

#[binrw]
#[derive(Debug)]
pub struct CharacterSelectInfos {
    #[br(try)]
    pub char_info1: Option<CharacterSelectInfo>,
    #[br(try)]
    pub char_info2: Option<CharacterSelectInfo>,
    #[br(try)]
    pub char_info3: Option<CharacterSelectInfo>,
    #[br(try)]
    pub char_info4: Option<CharacterSelectInfo>,
}

#[binrw]
#[derive(Debug)]
pub struct CharacterSelectInfo {
    pub char_id: u32,
    pub user_id: KStr,
    pub class: u8,
    pub race: u8,
    pub level: u8,
    pub item_count: u8,
    #[br(count=item_count)]
    pub inv: Vec<UserItemData>,
}

#[binrw]
#[derive(Debug)]
pub struct NPCInfo {
    pub name: KStr,
    pub nation: u32,
    s1: u32,
    pub pos: Vector3,
    target_pos: Vector3,
    xyzw: u32,
    k0: f32,
    k1: u8,
    curr_health: u32,
    max_health: u32,
    k4: Vector3,
    k5: u32,
    k6: Bool,
    k7: Bool,
    k8: u32,
}

#[binrw]
#[derive(Debug)]
pub struct OtherInfo {
    pub unit_type: u32,
    pub socket_id: u32,
    #[br(if(unit_type == 1))]
    pub pb: Option<PlayerBase>,
    #[br(if(unit_type == 0))]
    pub ni: Option<NPCInfo>,
}

#[binrw]
#[derive(Debug)]
pub struct OtherPlayerInfo {
    name: KStr,
    #[br(count = 49)]
    uid: Vec<UserItemData>,
}

#[binrw]
#[derive(Debug)]
pub struct BasicAttack {
    source_id: u32,
    target_id: u32,
}

#[binrw]
#[derive(Debug)]
pub struct BasicNpcAttack {
    source_id: u32,
    target_id: u32,
}

#[binrw]
#[derive(Debug)]
pub struct TargetInfo {
    source_unit_id: u32,
    target_unit_id: u32,
    w: u32,
}

#[binrw]
#[derive(Debug)]
pub struct ItemBaseUpdate {
    s0: u32,
    #[br(count=s0)]
    s1: Vec<u8>,
}

#[binrw]
#[derive(Debug)]
pub struct UnitDead {
    pub unit_id: u32,
}

#[binrw]
#[derive(Debug)]
pub struct UnitDestroy {
    pub unit_id: u32,
}
#[binrw]
#[derive(Debug)]
pub struct UnitDestroyAll {}

#[binrw]
#[derive(Debug)]
pub struct UseMagicEvent {
    magic_opcode: MagicOpcode,
    // #[br(if(magic_opcode == MagicOpcode::MagicCasting))]
    // magic_info: Option<MagicInfo>,
}

#[binrw]
#[derive(Debug, Copy, Clone)]
#[brw(repr(u32))]
pub enum MagicOpcode {
    MagicCasting = 0,
    MagicEffecting = 1,
    MagicFlying = 2,
    MagicCastingCancelled = 3,
    MagicCastingFailed = 4,
    MagicEffectFailed = 5,
    MagicFlyingFailed = 6,
    MagicSuccessEffecting = 7,
    MagicRemoveBuff = 8,
    MagicIDK = 32,
}

impl PartialEq for MagicOpcode {
    fn eq(&self, other: &Self) -> bool {
        *self as u32 == *other as u32
    }
}

#[binrw]
#[derive(Debug)]
pub struct MagicInfo {
    skill_id: u32,
    target_id: u32,
    idk: Vector3,
    str_key: KStr,
}

#[binrw]
#[derive(Debug)]
pub struct Teleport {
    _type: u32,
    #[br(if(_type == 2))]
    player_id: Option<u32>,
    #[br(if(_type == 2))]
    coord_x: Option<u32>,
    #[br(if(_type == 2))]
    coord_z: Option<u32>,
    #[br(if(_type == 2))]
    curr_health: Option<u32>,
    #[br(if(_type == 2))]
    max_health: Option<u32>,
}

#[binrw]
#[derive(Debug)]
pub struct Merchant {
    pub merchant_op: MerchantOp,
}

#[binrw]
#[derive(Debug)]
pub enum MerchantOp {
    #[br(magic(0x00u32))]
    MerchantOpen {},
    #[br(magic(0x01u32))]
    MerchantClose {},
    #[br(magic(0x03u32))]
    MerchantItemAddResult { is_success: Bool },
    #[br(magic(0x04u32))]
    MerchantItemListUpdate {
        #[br(count = 12)]
        merchant_items: Vec<TradeItem>,
    },
    #[br(magic(0x07u32))]
    MerchantItemList {
        slave_id: u32,
        #[br(count = 16)] // dude forgot it was 12
        merchant_items: Vec<MerchantItem>,
    },
    #[br(magic(0xbu32))]
    MerchantItemBuyResult { is_success: Bool },
    #[br(magic(0xcu32))]
    SlaveMerchantItemList {
        #[br(count = 12)]
        extended_merchant_items: Vec<ExtendedMerchantItem>,
    },
}

#[binrw]
#[derive(Debug)]
pub struct ExtendedMerchantItem {
    pub item_id: u32,
    pub item_count: u32,
    pub item_durability: u32,
    pub item_price: u32,
    pub sold_item_count: u32,
    pub claimable_price: u64,
    pub claimed_prices: u64,
    pub is_claimed: Bool,
    pub serial: u64,
}

#[binrw]
#[derive(Debug)]
pub struct TradeItem {
    pub item_id: u32,
    pub item_count: u32,
    pub item_price: u32,
}

#[binrw]
#[derive(Debug)]
pub struct Slave {
    slave_op: SlaveOp,
}

#[binrw]
#[derive(Debug)]
pub enum SlaveOp {
    #[br(magic(0x00u32))]
    SlaveInfo {
        slave_id: u32,
        slave_pos: Vector3,
        angle: f32,
        merchant_data: MerchantData,
    },
    #[br(magic(0x01u32))]
    SlaveRemove { slave_id: u32 },
    #[br(magic(0x02u32))]
    SlaveMerchantUpdate {
        slave_id: u32,
        merchant_data: MerchantData,
    },
}

#[binrw]
#[derive(Debug)]
pub struct MerchantData {
    #[br(count = 12)]
    pub merchant_items: Vec<MerchantItem>,
}

#[binrw]
#[derive(Debug)]
pub struct MerchantItem {
    pub item_id: u32,
    pub item_count: u32,
    pub item_duration: u32,
    pub item_price: u32,
    pub item_serial: u64,
}

// #[binrw]
// #[derive(Debug)]
// pub struct FieldChanged {
//     socket_id: u32,
//     field_type: u32,
//     #[br(if(field_type == 3))]
//     new_move_speed: Option<f32>,
//     #[br(if(field_type == 3))]
//     old_move_speed: Option<f32>,
// }
#[binrw]
#[derive(Debug)]
pub struct FieldChanged {
    pub socket_id: u32,
    pub field_changed_type: FieldChangedType,
}

#[binrw]
#[derive(Debug)]
pub enum FieldChangedType {
    #[br(magic(1u32))]
    Loyalty(i32),
    #[br(magic(2u32))]
    LoyaltyMonthly(i32),
    #[br(magic(3u32))]
    Movespeed(f32),
    #[br(magic(4u32))]
    CurrentExp(u64),
    #[br(magic(5u32))]
    MaxExp(u64),
    #[br(magic(6u32))]
    CurrentHealth(u32),
    #[br(magic(7u32))]
    MaxHealth(u32),
    #[br(magic(8u32))]
    CurrentMana(u32),
    #[br(magic(9u32))]
    MaxMana(u32),
    #[br(magic(10u32))]
    Attack(u32),
    #[br(magic(11u32))]
    Defense(u32),
    #[br(magic(12u32))]
    DamageStats {
        fire_damage: u32,
        ice_damage: u32,
        lightning_damage: u32,
        poison_damage: u32,
    },
    #[br(magic(13u32))]
    ResistStats {
        fire_resist: u32,
        ice_resist: u32,
        lightning_resist: u32,
        poison_resist: u32,
    },
    #[br(magic(14u32))]
    StatPoints {
        free_stat_points: u16,
        stats: [u8; 5],
        bonus_stats: [u16; 5],
    },
    #[br(magic(15u32))]
    SkillPoints {
        free_skills: u8,
        skill_points: [u8; 4],
    },
    #[br(magic(16u32))]
    MannerPoint(u32),
    #[br(magic(17u32))]
    Level(u8),
    #[br(magic(18u32))]
    Coins(u32),
    #[br(magic(19u32))]
    Cash(u32),
}

#[binrw]
#[derive(Debug)]
pub struct PingPong {
    ms_timer: u32,
}

#[binrw]
#[derive(Debug)]
pub struct Gamble {
    gamble_op: GambleOp,
}

#[binrw]
#[derive(Debug)]
pub enum GambleOp {
    #[br(magic(0x00u32))]
    GambleStart {},
    #[br(magic(0x01u32))]
    GambleEnd {},
    #[br(magic(0x02u32))]
    GambleData { is_success: Bool },
    #[br(magic(0x03u32))]
    GambleUpdate { my_tickets_per_user: u32, s0: f32 },
    #[br(magic(0x04u32))]
    TicketsCountUpdate {},
    #[br(magic(0x05u32))]
    GambleLastWinners {},
}

#[binrw]
#[derive(Debug)]
pub struct CollectionRace {
    collection_race_op: CollectionRaceOp,
}

#[binrw]
#[derive(Debug)]
pub enum CollectionRaceOp {
    #[br(magic(0x00u32))]
    StateChanged {},
    #[br(magic(0x01u32))]
    Time {},
    #[br(magic(0x02u32))]
    KillsUpdate {},
    #[br(magic(0x03u32))]
    KillIncrease {},
    #[br(magic(0x04u32))]
    Completed {},
    #[br(magic(0x05u32))]
    Notify {},
}

#[binrw]
#[derive(Debug)]
pub struct ClanNts {
    clan_nts_op: ClanNtsOp,
}

#[binrw]
#[derive(Debug)]
pub enum ClanNtsOp {
    #[br(magic(0x00u32))]
    ReqList {},
    #[br(magic(0x01u32))]
    NtsProcess {},
}

#[binrw]
#[derive(Debug)]
pub struct Robot {}

#[binrw]
#[derive(Debug)]
pub struct SpecialGift {
    special_gift_op: SpecialGiftOp,
}

#[binrw]
#[derive(Debug)]
pub enum SpecialGiftOp {
    #[br(magic(0x00u32))]
    ReqList {},
    #[br(magic(0x01u32))]
    ClearList {},
    #[br(magic(0x02u32))]
    AddList {},
    #[br(magic(0x03u32))]
    Claim {},
}

#[binrw]
#[derive(Debug)]
pub struct SafeDevice {
    safe_device_op: SafeDeviceOp,
}

#[binrw]
#[derive(Debug)]
pub enum SafeDeviceOp {
    #[br(magic(0x00u32))]
    Disabled {},
    #[br(magic(0x01u32))]
    Enabled {},
    #[br(magic(0x02u32))]
    Limited {},
    #[br(magic(0x03u32))]
    SetCurrentDevice {},
    #[br(magic(0x04u32))]
    RemoveCurrentDevice {},
}

#[binrw]
#[derive(Debug)]
pub struct Gender {
    gender_op: GenderOp,
}

#[binrw]
#[derive(Debug)]
pub enum GenderOp {
    #[br(magic(0x00u32))]
    Change {},
}

#[binrw]
#[derive(Debug)]
pub struct Map {
    map_op: MapOp,
}

#[binrw]
#[derive(Debug)]
pub enum MapOp {
    #[br(magic(0x00u32))]
    NpcInfo {},
    #[br(magic(0x01u32))]
    PartyPlayersInfo {},
    #[br(magic(0x02u32))]
    Teleport {}, // Teleport locations
}

#[binrw]
#[derive(Debug)]
pub struct CharacterSeal {
    character_seal_op: CharacterSealOp,
}

#[binrw]
#[derive(Debug)]
pub enum CharacterSealOp {
    #[br(magic(0x00u32))]
    ReqOpen {},
    #[br(magic(0x01u32))]
    Seal {},
    #[br(magic(0x02u32))]
    Unseal {},
    #[br(magic(0x03u32))]
    CharacterLists {},
    #[br(magic(0x04u32))]
    CharacterInfo {},
    #[br(magic(0x05u32))]
    CodeActivation {},
    #[br(magic(0x06u32))]
    ReqCodeActivation {},
    #[br(magic(0x07u32))]
    CharacterListForUnseal {},
}

#[binrw]
#[derive(Debug)]
pub struct PingPongy {}

#[binrw]
#[derive(Debug)]
pub struct ChatNotice {
    should_show: Bool,
    msg: KStr,
}

#[binrw]
#[derive(Debug)]
pub struct ChatNormal {
    pub chat_type: ChatNormalType,
    pub msg: KStr,
    pub from: KStr,
    pub socket_id: u32,
    pub from_nation: u8,
}

#[binrw]
#[brw(repr(u32))]
#[derive(Debug, PartialEq, Eq)]
pub enum ChatNormalType {
    AllChat = 0,
    Shout = 1,
    Party = 2,
    Clan = 3,
    NoahKnights = 4,
    Notice = 20,
    NoticeAll = 21,
    Private = 30,
    Obtained = 31,
}

#[binrw]
#[derive(Default, Builder, Debug)]
pub struct Move {
    pub user_id: u32,
    pub is_moving: Bool,
    pub origin: Vector3,
    pub target: Vector3,
}

#[binrw]
#[derive(Debug)]
pub struct NoahKnights {
    s0: u32,
    player_id: u32,
    noah_knights_active: Bool,
    #[br(if(noah_knights_active.into()))]
    txt_clan: Option<KStr>,
}

#[binrw]
#[derive(Debug)]
pub struct SeekingPartyElement {
    socket_id: u32,
    user_id: KStr,
    nation: u8,
    class: u8,
    level: u8,
}

#[binrw]
#[derive(Debug)]
pub struct SeekingParty {
    seek_type: u32,
    #[br(if(seek_type == 1))]
    s0: u16,
    #[br(if(seek_type == 1))]
    #[br(count=s0)]
    spe: Vec<SeekingPartyElement>,
    #[br(if(seek_type == 0))]
    player_id: Option<u32>,
    #[br(if(seek_type == 0))]
    seeking_state: Option<Bool>,
}

#[binrw]
#[derive(Debug)]
pub struct MilitaryCampElement {
    pub camp_id: u32,
    pub camp_name: KStr,
    pub zone_id: u32,
}

#[binrw]
#[derive(Debug)]
pub struct WarpListData {
    id: u32,
    zone_name: KStr,
    zone_description: KStr,
    req_coint: u16,
    min_level: u8,
    max_level: u8,
    nation: u8,
    can_teleport: Bool,
}

#[binrw]
#[derive(Debug)]
pub struct WarpObject {
    s0: u32,
    s1: u32,
    s2: u32,
    s3: u32,
    s4: u8,
}

#[binrw]
#[derive(Debug)]
pub struct Warp {
    w0: u32,
    #[br(if(w0 == 2))]
    object_count: u32,
    #[br(if(w0 == 2))]
    #[br(count=object_count)]
    wo: Vec<WarpObject>,
    #[br(if(w0 == 0))]
    data_size: u32,
    #[br(if(w0 == 0))]
    #[br(count=data_size)]
    wld: Vec<WarpListData>,
}

#[binrw]
#[derive(Debug)]
pub struct MilitaryCamp {
    pub s0: u32,
    #[br(if(s0 == 0))]
    pub current_zone_id: u32,
    #[br(if(s0 == 0))]
    pub zone_size: u8,
    #[br(if(s0 == 0))]
    #[br(count=zone_size)]
    pub mce: Vec<MilitaryCampElement>,
}

#[binrw]
#[derive(Debug)]
pub struct QueuePacket {
    pub queue_op: QueueOp,
}

#[binrw]
#[derive(Debug)]
pub enum QueueOp {
    #[br(magic(0u32))]
    Queue { queue_position: u32 },
    #[br(magic(1u32))]
    Loginned {},
}

#[binrw]
#[derive(Debug)]
pub struct ChickenSystem {
    s0: u32,
    #[br(if(s0 <= 1))]
    player_id: u32,
    #[br(if(s0 <= 1))]
    toggle_chicken_symbol: Bool,
}

#[binrw]
#[derive(Debug)]
pub struct SelectTarget {
    player_id: u32,
    selected_id: u32,
}

#[binrw]
#[derive(Debug)]
pub struct EquipmentVisualUpdate {
    player_id: u32,
    behavoir_type: u8,
    item_id: u32,
    s0: u16,
}

#[binrw]
#[derive(Debug)]
pub struct CospreVisualToggle {
    player_id: u32,
    cospre_type: u8,
    toggle: Bool,
}

#[binrw]
#[derive(Debug)]
pub struct PartyPacket {
    pub po: PartyOp,
}

#[binrw]
#[derive(Debug)]
pub enum PartyOp {
    #[br(magic(0u32))]
    RequestUserAdd { user_name: KStr, timer: u32 },
    #[br(magic(1u32))]
    RequestResult { user_name: KStr, result: u32 },
    #[br(magic(2u32))]
    Leave { user_name: KStr, timer: u32 },
    #[br(magic(3u32))]
    KickUser { user_name: KStr, timer: u32 },
    #[br(magic(4u32))]
    MakeLeader { user_name: KStr, timer: u32 },
    #[br(magic(5u32))]
    PartyInfo {
        party_user_count: u32,
        #[br(count=party_user_count)]
        party_users: Vec<PartyUser>,
    },
    #[br(magic(6u32))]
    PartyUserHpChanged {
        party_idx: u32,
        current_health: u32,
        max_health: u32,
    },
    #[br(magic(7u32))]
    PartyUserManaChanged {
        party_idx: u32,
        current_mana: u32,
        max_mana: u32,
    },
    #[br(magic(8u32))]
    PartyBuffDebuffAddRemove {
        party_idx: u32,
        magic_id: u32,
        is_add: Bool,
        is_debuff: Bool,
    },
    #[br(magic(9u32))]
    PartyLeaderState {
        user_id: u32,
        toggle_party_leader_symbol: Bool,
    },
}

#[binrw]
#[derive(Debug)]
pub struct PartyUser {
    is_party_leader: Bool,
    socket_id: u32,
    username: KStr,
    nation: u8,
    class: u8,
    health: u32,
    max_health: u32,
    mana: u32,
    max_mana: u32,
    s0: u8,
    buff_count: u8,
    #[br(count=buff_count)]
    buff_list: Vec<u32>,
    debuff_count: u8,
    #[br(count=debuff_count)]
    debuff_list: Vec<u32>,
}

#[binrw]
#[derive(Debug)]
pub struct Friend {
    pub f: FriendReceiveOp,
}

#[binrw]
#[derive(Debug)]
pub enum FriendReceiveOp {
    #[br(magic(0u32))]
    FriendListResponse {
        list_size: u8,
        #[br(count = list_size)]
        friends: Vec<FriendData>,
    },
    #[br(magic(1u32))]
    FriendAddRequest { s0: KStr, user_name: KStr },
}

#[binrw]
#[derive(Debug)]
pub struct FriendData {
    user_id: u32,
    socket_id: u32,
    user_name: KStr,
    friend_type: u8,
}

#[binrw]
#[derive(Debug)]
pub struct LevelGiftElemInner {
    item_id: u32,
    item_count: u32,
    duration: u16,
    hour: u16,
}

#[binrw]
#[derive(Debug)]
pub struct LevelGiftElement {
    gift_id: u32,
    gift_level: u8,
    claimed: Bool,
    size: u8,
    #[br(count=size)]
    lge: Vec<LevelGiftElemInner>,
}

#[binrw]
#[derive(Debug)]
pub struct LevelGift {
    gifts: LevelGiftTypes,
}

#[binrw]
#[derive(Debug)]
pub enum LevelGiftTypes {
    #[br(magic(0u32))]
    LevelGift0 { s0: u8 },
    #[br(magic(2u32))]
    LevelGift2 {
        s0: u8,
        #[br(count=s0)]
        lge: Vec<LevelGiftElement>,
    },
}

#[binrw]
#[derive(Debug)]
pub struct SkillShortcuts {
    #[br(count = 100)]
    skill_ids: Vec<u32>,
}

#[binrw]
#[derive(Debug)]
pub struct ChatWhisper {
    pub chat_whisper_enum: ChatWhisperEnum,
}

#[binrw]
#[derive(Debug)]
pub enum ChatWhisperEnum {
    #[br(magic = 0u32)]
    RequestPM { name: KStr, socket_id: u32 },
    #[br(magic = 1u32)]
    Message {
        from: KStr,
        message: KStr,
        s2: u32,
        s3: u8,
    },
    #[br(magic = 2u32)]
    Seen { name: KStr, s0: u32 },
}

#[binrw]
#[derive(Debug)]
pub struct QuestType2 {
    req_item_id: u32,
    req_item_count: u16,
    req_rel: u16,
}

#[binrw]
#[derive(Debug)]
pub struct QuestRewardItem {
    item_id: u32,
    item_count: u32,
    item_dat: u32,
}

#[binrw]
#[derive(Debug)]
pub enum Quest {
    #[br(magic = 0u32)]
    UpdateCurrentQuestList {
        update_current_quest_type: UpdateCurrentQuestListType,
    },
    #[br(magic = 1u32)]
    UpdateQuestKillAmount {
        quest_id: u32,
        curr_idx: u8,
        new_curr_count: u16,
    },
    #[br(magic = 2u32)]
    QuestCompleted { quest_id: u32, s0: Bool },
}

#[binrw]
#[derive(Debug)]
pub enum UpdateCurrentQuestListType {
    #[br(magic(1u8))]
    Add {
        quest_id: u32,
        is_completed: Bool,
        need_level: u8,
        quest_name: KStr,
        quest_description: KStr,
        quest: QuestType,
        #[br(count = 4)]
        qri: Vec<QuestRewardItem>,
    },
    #[br(magic(0u8))]
    Remove {},
}

#[binrw]
#[derive(Debug)]
pub enum QuestType {
    #[br(magic = 1u8)]
    KillMonster {
        monster_name: KStr,
        target_count: u16,
        curr_count: u16,
    },
    #[br(magic = 2u8)]
    CollectItem {
        #[br(count = 4)]
        qt2: Vec<QuestType2>,
    },
}

// #[binrw]
// #[derive(Debug)]
// enum QuestE {

// }

#[binrw]
#[derive(Debug)]
pub struct GameStart {
    s0: u32,
}

#[binrw]
#[derive(Debug)]
pub struct ExchangeItemDataF {
    w0: u32,
    w1: u32,
    w2: u32,
    w3: u32,
}

#[binrw]
#[derive(Debug)]
pub struct ExchangeItemData {
    s0: u8,
    s1: u8,
    size: u8,
    #[br(count=size)]
    eidf: Vec<ExchangeItemDataF>,
}

#[binrw]
#[derive(Debug)]
pub struct ExchangeItem {
    _type: u32,
    #[br(if(_type == 2))]
    eid: Option<ExchangeItemData>,
    #[br(if(_type == 0))]
    item_list_size: u32,
    #[br(if(_type == 0))]
    #[br(count=item_list_size)]
    item_id_list: Option<Vec<u32>>,
}

#[binrw]
#[derive(Debug)]
pub struct DropLoot {
    pub unit_id: u32,
    pub unit_name: KStr,
}

#[binrw]
#[derive(Debug)]
pub struct DropLootItem {
    s0: u32,
    s1: u32,
    s2: u32,
    s3: u32,
    s4: u32,
}

#[binrw]
#[derive(Debug)]
pub struct DropLootItemList {
    drop_id: KStr,
    is_looted: u32,
    item_count: u32,
    #[br(count=item_count)]
    dli: Vec<DropLootItem>,
}

#[binrw]
#[derive(Debug)]
pub struct ChatInfo {
    pub msg: KStr,
}

#[binrw]
#[derive(Debug)]
pub struct Damage {
    socket_id: u32,
    damage: i32,
}

#[binrw]
#[derive(Debug)]
pub struct InventorySlotUpdate {
    pub item_inventory_id: u32,
    pub item_id: u32,
    pub count: u16,
    pub durability: u16,
    pub flag: u8,
    pub expire_time: u32,
    pub serial: u64,
}

#[binrw]
#[derive(Debug)]
pub struct Warehouse {
    pub warehouse_op: WarehouseOp,
}

#[binrw]
#[derive(Debug)]
pub enum WarehouseOp {
    #[br(magic = 0u32)]
    WarehouseOpen { open_ui_window: Bool },
    #[br(magic = 1u32)]
    WarehouseClose {},
    #[br(magic = 2u32)]
    WarehouseItemList {
        warehouse_coins: u32,
        item_count: u8,
        #[br(count=item_count)]
        items: Vec<UserItemData>,
    },

    #[br(magic = 3u32)]
    WarehouseItemSwap {},

    #[br(magic = 4u32)]
    WarehouseCoinSwap { warehouse_coins: u32 },
    #[br(magic = 5u32)]
    WarehouseItemSlotUpdate {
        item_slot: u8,
        new_item: UserItemData,
    },
}

#[binrw]
#[derive(Debug)]
pub struct ItemUpgrade {
    pub item_op: ItemUpgradeOp,
}

#[binrw]
#[derive(Debug)]
pub enum ItemUpgradeOp {
    #[br(magic = 0u32)]
    ReqItemUpgradeData {
        can_be_upgraded: Bool,
        #[br(if(can_be_upgraded.into()))]
        update_rate: u32,
        #[br(if(can_be_upgraded.into()))]
        needed_coins: u32,
    },
    #[br(magic = 1u32)]
    ReqItemAccessoryUpgradeData {
        can_be_upgraded: Bool,
        #[br(if(can_be_upgraded.into()))]
        update_rate: u32,
        #[br(if(can_be_upgraded.into()))]
        needed_coins: u32,
    },
    #[br(magic = 2u32)]
    ItemUpgrade {
        is_success: Bool,
        #[br(if(is_success.into()))]
        result_item_base: u32,
    },
    #[br(magic = 3u32)]
    ItemAccessoryUpgrade {},
    #[br(magic = 4u32)]
    ItemPreview { result_item_base: u32 },
    #[br(magic = 5u32)]
    ItemAccessoryPreview { result_item_base: u32 },
    #[br(magic = 6u32)]
    ItemUpgradeResult {},
    #[br(magic = 7u32)]
    ItemAccessoryUpgradeResult {},
}

#[binrw]
#[derive(Debug)]
pub struct MagicAnvil {
    pub is_success: u32,
}

#[binrw]
#[derive(Debug)]
pub struct NpcInteraction {
    is_ok: u32,
    #[br(if(is_ok == 1))]
    npc_id: Option<u32>,
}

#[binrw]
#[derive(Debug)]
pub struct NpcTrading {
    trading_op: NpcTradingOp,
}

#[binrw]
#[derive(Debug)]
pub enum NpcTradingOp {
    #[br(magic = 0u32)]
    NpcItemList {
        npc_id: u32,
        npc_name: KStr,
        have_items: Bool,
        // #[br(if(have_items.into()))]
    },
    #[br(magic = 1u32)]
    NpcBuyItem { is_success: Bool },
    #[br(magic = 2u32)]
    NpcSellItem { is_success: Bool },
}

#[binrw]
#[derive(Debug)]
pub struct Buff {
    _type: u32,
    #[br(if(_type == 2))]
    s0: Option<u32>,
    #[br(if(_type == 2))]
    s1: Option<u32>,
    #[br(if(_type == 2))]
    s2: Option<f32>,
    #[br(if(_type == 2))]
    b0: Option<Bool>,
    #[br(if(_type == 2))]
    b1: Option<Bool>,
    #[br(if(_type == 0))]
    skill_id: Option<u32>,
    #[br(if(_type == 0))]
    buff_type: Option<u32>,
    #[br(if(_type == 0))]
    remaining_time: Option<u32>,
    #[br(if(_type == 0))]
    is_buff: Option<Bool>,
    #[br(if(_type == 0))]
    is_active: Option<Bool>,
    #[br(if(_type == 1))]
    buffelem: Option<u32>,
    #[br(if(_type == 1))]
    _k0: Option<u32>,
}

#[binrw]
#[derive(Debug)]
pub struct Premium {
    _type: u32,
    #[br(if(_type == 0))]
    is_active: Bool,
    #[br(if(_type == 0 && is_active.into()))]
    pd: Option<PremiumData>,
}

#[binrw]
#[derive(Debug)]
pub struct PremiumData {
    remaining_seconds: u32,
    premium_name: KStr,
    premium_description: KStr,
}
