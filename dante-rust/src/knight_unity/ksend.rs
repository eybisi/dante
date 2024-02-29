#![allow(dead_code)]

use crate::knight_unity::common::{Bool, KStr, Nation, Vector3};

use binrw::{binrw, BinRead, BinWrite};
use crypto::buffer::{ReadBuffer, WriteBuffer};
use crypto::symmetriccipher;
use crypto::{aes, buffer};
use derive_builder::Builder;
use std::io::Cursor;

// use tokio::{
//     io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
//     net::{tcp::OwnedWriteHalf, TcpStream},
// };

#[binrw]
#[derive(Debug)]
#[br(assert(footer == 0xaa55u16 && header == 0x55AAu16))]
pub struct KSend {
    header: u16,
    data_size: u16,
    #[br(count=data_size)]
    inner_data: Vec<u8>,
    footer: u16,
}

#[binrw]
pub enum KSendInner {
    LoginMainServer(LoginMainServer),
    // LoginSubServer(LoginSubServer),
    NationSelect(NationSelect),
    StatUpgrade(StatUpgrade),
    SkillUpgrade(SkillUpgrade),
    DeviceInfo(DeviceInfo),
    CharacterCreate(CharacterCreate),
    CharacterSelect(CharacterSelect),
    Move(Move),
    GameStart(GameStart),
    Town(Town),
    GeniePacket(GeniePacket),
    PartyPacketResult(PartyPacketResult),
    MilitaryCampChange(MilitaryCampChange),
    PingPong(PingPong),
    PlayerUpdate(PlayerUpdate),
    TeleportEnd(TeleportEnd),
    FriendAddRequest(FriendAddRequest),
    DropReceiveItem(DropReceiveItem),
    DropItemList(DropItemList),
    Disconnect(Disconnect),
    TargetSelect(TargetSelect),
    UseMagic(UseMagic),
    ItemUpgrade(ItemUpgrade),
    NpcBuyItem(NpcBuyItem),
    NpcInteractionOpen(NpcInteractionOpen),
    WarehouseOpen(WarehouseOpen),
    WarehouseCoinSwap(WarehouseCoinSwap),
    PutItemToWarehouse(PutItemToWarehouse),
    SlaveMerchantGetMyItems(SlaveMerchantGetMyItems),
    SlaveMerchantOpen(SlaveMerchantOpen),
    SlaveMerchantItemAdd(SlaveMerchantItemAdd),
    SlaveMerchantListUpdate(SlaveMerchantListUpdate),
    SlaveMerchantInsert(SlaveMerchantInsert),
    SlaveMerchantRemove(SlaveMerchantRemove),
    SlaveMerchantClaimPrice(SlaveMerchantClaimPrice),
    GambleBuyTicketWithPrice(BuyTicketWithPrice),
    GambleBuyTicketWithItem(ButTicketWithItem),
    GambleBuyTicketWithCash(BuyTicketWithCash),
    MerchantItemSearchGetItems(MerchantItemSearchGetItems),
    SendOtp(SendOtp),
    Enc(Enc),
}

impl KSend {
    pub fn generate_from_inner(inner_data: Vec<u8>) -> Self {
        Self {
            header: 0x55AA,
            data_size: inner_data.len() as u16,
            inner_data,
            footer: 0xAA55,
        }
    }

    pub fn generate_encrypted_from_inner(plain_data: Vec<u8>) -> Self {
        let encrypted_data = encrypt_aes(&plain_data);
        let encrypted_data = encrypted_data.unwrap();
        let enc = Enc {
            key: encrypted_data.1,
            enc_size: encrypted_data.0.len() as u32,
            enc_data: encrypted_data.0,
        };

        let mut writer = Cursor::new(Vec::new());
        enc.write_le(&mut writer).unwrap();
        let buf = writer.into_inner();
        Self {
            header: 0x55AA,
            data_size: buf.len() as u16,
            inner_data: buf,
            footer: 0xAA55,
        }
    }

    pub fn parse_inner(&self) -> Result<KSendInner, binrw::Error> {
        let first_parse = KSendInner::read_le(&mut Cursor::new(&self.inner_data)).unwrap();
        if let KSendInner::Enc(enc) = first_parse {
            // dbg_info!("Request is encrypted !");
            let decrypted = enc.decrypt();
            // dbg_info!("decrypted packet : {}",decrypted.typ);
            match decrypted {
                Ok(d) => {
                    // dbg_info!("packet decrypted");
                    Ok(d)
                }
                Err(e) => {
                    // dbg_info!("Error while decrypting packet : {:?}", e);
                    return Err(e);
                }
            }
        } else {
            Ok(first_parse)
        }
    }
    pub fn to_buf(self) -> Vec<u8> {
        let mut writer = Cursor::new(Vec::new());
        self.write_le(&mut writer).unwrap();
        writer.into_inner()
    }
}

impl From<KSendInner> for KSend {
    fn from(val: KSendInner) -> Self {
        let mut writer = Cursor::new(Vec::new());
        let _ = val.write_le(&mut writer);
        let buf = writer.into_inner();
        KSend::generate_from_inner(buf)
    }
}

#[binrw]
#[derive(Default, Builder)]
#[brw(little)]
#[brw(magic(4u16))]
pub struct Disconnect {
    disconnect: Bool,
}

#[binrw]
#[derive(Debug, Default, Builder)]
#[brw(little)]
#[brw(magic(1u16))]
pub struct LoginMainServer {
    pub username: KStr,
    pub password: KStr,
    pub dev_id: KStr,
    //LENOVO Lenovo P2a42,
    pub dev_model: KStr,
    //LENOVO P2,
    pub dev_name: KStr,
    //{GraphicsDeviceType}|_|- False -| {GraphicsDeviceType}
    pub dev_gpu: KStr,
    pub graphic_mem_size: u32,
    pub device_mem_size: u32,
    pub device_id: KStr,
}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x0au16))]
pub struct NationSelect {
    pub nation: Nation,
}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x0bu16))]
pub struct CharacterCreate {
    char_name: KStr,
    curr_idx: u32,
    selected_race: u32,
    selected_class: u32,
    current_stats: [u8; 5],
}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x0cu16))]
pub struct CharacterSelect {
    curr_idx: u32,
}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x14u16))]
pub struct GameStart {}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x1eu16))]
pub struct Move {
    is_moving: Bool,
    origin: Vector3,
    target: Vector3,
}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x28u16))]
pub struct StatUpgrade {
    stat_idx: u32,
}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x2au16))]
pub struct SkillUpgrade {
    skill_idx: u32,
}

#[binrw]
#[derive(Debug,Default, Builder)]
#[brw(magic(0x258u16))]
pub struct DeviceInfo {
    res_width: u32,
    res_height: u32,
    res_width_2: u32,
    res_height_2: u32,
    screen_width: u32,
    screen_height: u32,
    slider_value: f32,
    battery_status: u32,
    battery_level: f32,
    operation_system: KStr,
    processor_count: u32,
    processor_freq: f32,
    processor_type: KStr,
    supports_vibration: Bool,
    supports_multiview: Bool,
}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x24u16))]
pub struct SelectTarget {
    target_id: u32,
}

// 20003199 = inn hostess;

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x96u16))]
pub struct NpcInteractionOpen {
    npc_interaction_op: u32,
    npc_id: u32,
}

//[NPC_BUY_ITEM,20002796,18,281001001,1]
#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x97u16))]
pub struct NpcBuyItem {
    npc_trading_op: u32,
    npc_id: u32,
    item_slot: u8,
    item_id: u32,
    item_count: u32,
}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x3au16))]
pub struct CospreVisualToggle {
    _type: u8,
    is_on: Bool,
}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x0172u16))]
pub struct PlayerUpdate {}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x20u16))]
pub struct UseMagic {
    pub magic_opcode: u32,
    pub skill_id: u32,
    pub target_id: u32,
    pub idk: Vector3,
    pub str_key: KStr,
}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x8cu16))]
pub struct ItemUpgrade {
    pub upgrade_opcode: u32,
    pub item_slot: u8,
    pub item_id: u32,
    pub upgrade_sc_slot: u8,
    pub upgrade_sc_id: u32,
    pub extra_item_slot: u8,
    pub extra_item_id: u32,
    pub is_preview: Bool,
}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x1feu16))]
pub struct PingPong {
    pingpong_opcode: u64,
}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x1fu16))]
pub struct BasicAttack {
    target_id: u32,
}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x24u16))]
pub struct TargetSelect {
    pub target_id: u32,
}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x25u16))]
pub struct TargetInfo {
    target_id: u32,
}

#[binrw]
#[brw(magic(0x35u16))]
#[derive(Default, Builder)]
pub struct WarehouseOpen {
    interact_opcode: u32,
}

#[binrw]
#[brw(magic(0x35u16))]
#[derive(Default, Builder)]
pub struct WarehouseCoinSwap {
    #[brw(magic(0x04u32))]
    from: u8, // 0 for bank, 1 for inventory,
    coin: u32,
}

//53 and params [ITEM_SWAP,1,15,281005007,1,0,24]
#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x35u16))]
pub struct PutItemToWarehouse {
    interact_opcode: u32,
    from: u8, // 0 is from bank, 1 is from inventory
    from_inv_slot: u8,
    from_inv_item_id: u32,
    from_inv_item_count: u16,
    to: u8, // 0 is from bank, 1 is from inventory
    to_inv_slot: u8,
}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x3du16))]
pub struct DropItemList {
    pub drop_key: KStr,
    pub receive_type: Bool,
}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x3eu16))]
pub struct DropReceiveItem {
    pub drop_key: KStr,
    pub receive_type: u32, // 100 is collect all
}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x26u16))]
pub struct TargetPlayerInventory {
    target_id: u32,
}

#[binrw]
#[brw(magic(0x53u16))]
struct ChatWhisper {
    private_chat_opcodes: u32,
    data: ChatWhisperData,
}

#[binrw]
enum ChatWhisperData {
    #[br(magic(0u32))]
    ChatWhisper0 { socket_id: u32 },
    #[br(magic(1u32))]
    ChatWhisper1 { s0: KStr, s1: KStr, s2: u32, s3: u8 },
    #[br(magic(2u32))]
    ChatWhisper2 { name: KStr, s0: u32 },
}

#[binrw]
#[brw(magic(0x6fu16))]
#[derive(Default, Builder)]
pub struct SlaveMerchantGetMyItems {
    merchant_op: u32, // 0xc for my items
}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x6fu16))]
pub struct SlaveMerchantClaimPrice {
    #[brw(magic(0x0du32))]
    merchant_slot_id: u32,
}

// Merchant Open flow:
// [MERCHANT_OPEN]
// [MERCHANT_ITEM_ADD,15,281003007,1,4350000,0]
// [MERCHANT_INSERT,(317.02, 60.93, 431.71),161.9352]

#[binrw]
#[brw(magic(0x6fu16))]
#[derive(Default, Builder)]
pub struct SlaveMerchantOpen {
    merchant_op: u32, // 0x0 for open
}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x6fu16))]
pub struct SlaveMerchantItemAdd {
    #[brw(magic(0x02u32))]
    from_inv_slot: u32,
    from_inv_item_id: u32,
    from_inv_item_count: u32,
    price: u32,
    merchant_slot_id: u32,
}

#[binrw]
#[brw(magic(0x6f04u32))]
pub struct SlaveMerchantListUpdate;

#[binrw]
#[brw(magic(0x6fu16))]
#[derive(Default, Builder)]
pub struct SlaveMerchantInsert {
    #[brw(magic(0x05u32))]
    location: Vector3,
    angle: f32,
}

#[binrw]
#[brw(magic(0x6fu16))]
#[derive(Default, Builder)]
pub struct SlaveMerchantRemove {
    merchant_op: u32, // 0x6 for remove
}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x82u16))]
pub struct SkillShortcuts {
    #[br(count = 100)]
    skill_ids: Vec<u8>,
}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0xc8u16))]
pub struct Town {}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x46u16))]
pub struct PartyPacketResult {
    pub type_id: u32,
    pub result: Bool,
}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x186u16))]
pub struct MilitaryCampChange {
    pub type_id: u32, // 1 is change
    pub camp_id: u32,
}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x19au16))]
pub struct GeniePacket {
    pub type_id: u32, // 0 for start, 1 for stop
}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x1c2u16))]
pub struct MerchantItemSearchGetItems {
    pub type_id: u32, // 0 search,  stop
}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x208u16))]
pub struct BuyTicketWithPrice {
    #[brw(magic(0x06u32))]
    pub type_id: u32, // 0 for start, 1 for stop
}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x208u16))]
pub struct ButTicketWithItem {
    #[brw(magic(0x07u32))]
    pub type_id: u32, // 0 for start, 1 for stop
}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x208u16))]
pub struct BuyTicketWithCash {
    #[brw(magic(0x08u32))]
    pub type_id: u32, // 0 for start, 1 for stop
}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x65u16))]
pub struct TeleportEnd {
    type_id: u32,
}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x47u16))]
pub struct FriendAddRequest {
    pub type_id: u32, // 1 Is for add
    pub token: KStr,
    pub result: Bool,
}

#[binrw]
pub enum PartyOp {
    #[br(magic(0u32))]
    RequestUserAdd { user_name: KStr, timer: u32 },
    #[br(magic(1u32))]
    RequestResult { result: Bool },
    #[br(magic(2u32))]
    Leave {},
    #[br(magic(3u32))]
    KickUser { user_id: u32 },
    #[br(magic(4u32))]
    MakeLeader { user_id: u32 },
}

impl Default for PartyOp {
    fn default() -> Self {
        PartyOp::RequestUserAdd {
            user_name: KStr::default(),
            timer: 0,
        }
    }
}

#[binrw]
#[derive(Default, Builder)]
#[brw(magic(0x5u16))]
pub struct SendOtp {
    pub code: KStr,
}

#[binrw]
#[derive(Debug, Default, Builder)]
#[brw(magic(0x79e0u16))]
pub struct Enc {
    pub key: u32,
    pub enc_size: u32,
    #[br(count=enc_size)]
    pub enc_data: Vec<u8>,
}

pub trait KEncryption {
    fn decrypt(&self) -> Result<KSendInner, binrw::Error>;
    fn encrypt(&self) -> Result<KSendInner, binrw::Error>;
}

impl KEncryption for Enc {
    fn decrypt(&self) -> Result<KSendInner, binrw::Error> {
        let result = decrypt_aes(self.enc_data.as_slice(), self.key)
            .ok()
            .unwrap();
        // dbg_info!("decrypted packet : {:?}",result);
        let mut c = Cursor::new(result);
        KSendInner::read_le(&mut c)
    }

    fn encrypt(&self) -> Result<KSendInner, binrw::Error> {
        todo!()
    }
}

fn pad_pkcs7(input: &mut Vec<u8>, block_size: usize) {
    let padding = block_size - (input.len() % block_size);
    for _ in 0..padding {
        input.push(0 as u8);
    }
}

fn encrypt_aes(input: &[u8]) -> Result<(Vec<u8>, u32), symmetriccipher::SymmetricCipherError> {
    // Generate a random u32 key
    let key = 0x073412u32;
    // Rjust the key to 16 bytes
    let key_v: [u8; 4] = key.to_le_bytes();
    let mut key_up = vec![0u8; 16];
    for _ in 0..key_v.len() {
        key_up.pop();
    }
    for k in 0..key_v.len() {
        key_up.push(key_v[k]);
    }
    let iv = [0u8; 16];
    let mut res = aes::cbc_encryptor(
        aes::KeySize::KeySize128,
        &key_up,
        &iv,
        crypto::blockmodes::NoPadding,
    );
    let mut output = [0u8; 4096];
    let mut final_res = Vec::<u8>::new();
    let mut out_buf = buffer::RefWriteBuffer::new(&mut output);
    // pad_pkcs7(&mut input,16);
    let mut padded_input = input.to_vec();
    pad_pkcs7(&mut padded_input, 16);
    let mut enc_buf = buffer::RefReadBuffer::new(&padded_input.as_slice());

    let enc_res = res.encrypt(&mut enc_buf, &mut out_buf, true)?;
    match enc_res {
        buffer::BufferResult::BufferUnderflow => {
            // dbg_info!("buffer underflow");
        }
        buffer::BufferResult::BufferOverflow => {
            // dbg_info!("buffer overflow");
        }
    }
    final_res.extend(
        out_buf
            .take_read_buffer()
            .take_remaining()
            .iter()
            .map(|&i| i),
    );
    Ok((final_res, key))
}
fn decrypt_aes(input: &[u8], key: u32) -> Result<Vec<u8>, symmetriccipher::SymmetricCipherError> {
    // info!("incoming input : {:?}",input);
    // info!("incoming input size : {}",input.len());
    // info!("incoming key : {}",key);
    let key_v: [u8; 4] = key.to_le_bytes();
    let mut key_up = vec![0u8; 16];
    for _ in 0..key_v.len() {
        key_up.pop();
    }
    for k in 0..key_v.len() {
        key_up.push(key_v[k]);
    }
    let iv = [0u8; 16];
    let mut res = aes::cbc_decryptor(
        aes::KeySize::KeySize128,
        &key_up,
        &iv,
        crypto::blockmodes::NoPadding,
    );

    let mut output = [0u8; 4096];
    let mut final_res = Vec::<u8>::new();
    let mut out_buf = buffer::RefWriteBuffer::new(&mut output);
    let mut enc_buf = buffer::RefReadBuffer::new(input);
    // info!("before enc");
    let dec_res = res.decrypt(&mut enc_buf, &mut out_buf, true);
    match dec_res {
        Ok(_) => {}
        Err(_e) => {
            // info!("Symmetriccipher error : {:?}",e);
        }
    }

    let dec_res = dec_res.unwrap();

    match dec_res {
        buffer::BufferResult::BufferUnderflow => {
            // dbg_info!("buffer underflow");
        }
        buffer::BufferResult::BufferOverflow => {
            // dbg_info!("buffer overflow");
        }
    }
    // info!("after enc");
    // println!("{:?}",out_buf.take_read_buffer().take_remaining());
    final_res.extend(
        out_buf
            .take_read_buffer()
            .take_remaining()
            .iter()
            .map(|&i| i),
    );
    // info!("decrypted packet : {:?}",final_res);
    Ok(final_res)
}
