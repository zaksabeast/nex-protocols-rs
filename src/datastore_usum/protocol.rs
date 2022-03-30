use nex_rs::client::ClientConnection;
use nex_rs::packet::PacketV1;
use num_enum::{IntoPrimitive, TryFromPrimitive};

pub const DATASTORE_PROTOCOL_ID: u8 = 0x73;

#[derive(Debug, Clone, Copy, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum DataStoreMethod {
    GetMetas = 0x9,
    RateObject = 0xF,
    PostMetaBinary = 0x15,
    ChangeMetas = 0x27,
    PrepareUploadPokemon = 0x2F,
    UploadPokemon = 0x30,
    PrepareTradePokemon = 0x32,
    TradePokemon = 0x33,
    DownloadOtherPokemon = 0x34,
    DownloadMyPokemon = 0x35,
    DeletePokemon = 0x36,
    SearchPokemonV2 = 0x37,
}

pub trait DataStoreProtocol {
    fn get_metas(&self, client: &mut ClientConnection) -> Result<(), &'static str>;
    fn rate_object(&self, client: &mut ClientConnection) -> Result<(), &'static str>;
    fn post_meta_binary(&self, client: &mut ClientConnection) -> Result<(), &'static str>;
    fn change_metas(&self, client: &mut ClientConnection) -> Result<(), &'static str>;
    fn prepare_upload_pokemon(&self, client: &mut ClientConnection) -> Result<(), &'static str>;
    fn upload_pokemon(&self, client: &mut ClientConnection) -> Result<(), &'static str>;
    fn prepare_trade_pokemon(&self, client: &mut ClientConnection) -> Result<(), &'static str>;
    fn trade_pokemon(&self, client: &mut ClientConnection) -> Result<(), &'static str>;
    fn download_other_pokemon(&self, client: &mut ClientConnection) -> Result<(), &'static str>;
    fn download_my_pokemon(&self, client: &mut ClientConnection) -> Result<(), &'static str>;
    fn delete_pokemon(&self, client: &mut ClientConnection) -> Result<(), &'static str>;
    fn search_pokemon_v2(&self, client: &mut ClientConnection) -> Result<(), &'static str>;

    fn handle_get_metas(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str>;
    fn handle_rate_object(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str>;
    fn handle_post_meta_binary(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str>;
    fn handle_change_metas(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str>;
    fn handle_prepare_upload_pokemon(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str>;
    fn handle_upload_pokemon(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str>;
    fn handle_prepare_trade_pokemon(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str>;
    fn handle_trade_pokemon(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str>;
    fn handle_download_other_pokemon(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str>;
    fn handle_download_my_pokemon(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str>;
    fn handle_delete_pokemon(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str>;
    fn handle_search_pokemon_v2(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str>;
}
