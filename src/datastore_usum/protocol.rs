use crate::datastore_usum::{
    DataStoreChangeMetaParam, DataStoreGetMetaParam, DataStorePreparePostParam,
    DataStoreRateObjectParam, DataStoreRatingTarget, GlobalTradeStationDeletePokemonParam,
    GlobalTradeStationDownloadMyPokemonParam, GlobalTradeStationDownloadOtherPokemonParam,
    GlobalTradeStationPrepareTradePokemonParam, GlobalTradeStationSearchPokemonParam,
    GlobalTradeStationTradePokemonParam, GlobalTradeStationUploadPokemonParam,
};
use async_trait::async_trait;
use nex_rs::client::ClientConnection;
use nex_rs::nex_types::NexList;
use nex_rs::packet::{Packet, PacketV1};
use no_std_io::{StreamContainer, StreamReader};
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

#[async_trait(?Send)]
pub trait DataStoreProtocol {
    fn get_metas(
        &self,
        client: &mut ClientConnection,
        call_id: u32,
        data_ids: NexList<u64>,
        param: DataStoreGetMetaParam,
    ) -> Result<(), &'static str>;
    fn rate_object(
        &self,
        client: &mut ClientConnection,
        call_id: u32,
        target: DataStoreRatingTarget,
        param: DataStoreRateObjectParam,
        fetch_ratings: bool,
    ) -> Result<(), &'static str>;
    fn post_meta_binary(
        &self,
        client: &mut ClientConnection,
        call_id: u32,
        param: DataStorePreparePostParam,
    ) -> Result<(), &'static str>;
    fn change_metas(
        &self,
        client: &mut ClientConnection,
        call_id: u32,
        data_ids: NexList<u64>,
        params: NexList<DataStoreChangeMetaParam>,
        transactional: bool,
    ) -> Result<(), &'static str>;
    fn prepare_upload_pokemon(
        &self,
        client: &mut ClientConnection,
        call_id: u32,
    ) -> Result<(), &'static str>;
    fn upload_pokemon(
        &self,
        client: &mut ClientConnection,
        param: GlobalTradeStationUploadPokemonParam,
    ) -> Result<(), &'static str>;
    fn prepare_trade_pokemon(
        &self,
        client: &mut ClientConnection,
        call_id: u32,
        param: GlobalTradeStationPrepareTradePokemonParam,
    ) -> Result<(), &'static str>;
    fn trade_pokemon(
        &self,
        client: &mut ClientConnection,
        call_id: u32,
        param: GlobalTradeStationTradePokemonParam,
    ) -> Result<(), &'static str>;
    fn download_other_pokemon(
        &self,
        client: &mut ClientConnection,
        call_id: u32,
        param: GlobalTradeStationDownloadOtherPokemonParam,
    ) -> Result<(), &'static str>;
    fn download_my_pokemon(
        &self,
        client: &mut ClientConnection,
        call_id: u32,
        param: GlobalTradeStationDownloadMyPokemonParam,
    ) -> Result<(), &'static str>;
    fn delete_pokemon(
        &self,
        client: &mut ClientConnection,
        param: GlobalTradeStationDeletePokemonParam,
    ) -> Result<(), &'static str>;
    fn search_pokemon_v2(
        &self,
        client: &mut ClientConnection,
        call_id: u32,
        param: GlobalTradeStationSearchPokemonParam,
    ) -> Result<(), &'static str>;

    fn handle_get_metas(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str> {
        let request = packet.get_rmc_request();
        let parameters = request.parameters.as_slice();

        let mut parameters_stream = StreamContainer::new(parameters);

        let data_ids = parameters_stream
            .read_stream_le::<NexList<u64>>()
            .map_err(|_| "Can not read data ids list")?;
        let param = parameters_stream
            .read_stream_le::<DataStoreGetMetaParam>()
            .map_err(|_| "Can not read DataStoreGetMetaParam")?;

        self.get_metas(client, request.call_id, data_ids, param)
    }

    fn handle_rate_object(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str> {
        let request = packet.get_rmc_request();
        let parameters = request.parameters.as_slice();

        let mut parameters_stream = StreamContainer::new(parameters);

        let target = parameters_stream
            .read_stream_le::<DataStoreRatingTarget>()
            .map_err(|_| "Can not read DataStoreRatingTarget")?;
        let param = parameters_stream
            .read_stream_le::<DataStoreRateObjectParam>()
            .map_err(|_| "Can not read DataStoreRateObjectParam")?;
        let fetch_ratings = parameters_stream
            .read_stream_le::<bool>()
            .map_err(|_| "Can not read fetch ratings bool")?;

        self.rate_object(client, request.call_id, target, param, fetch_ratings)
    }

    fn handle_post_meta_binary(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str> {
        let request = packet.get_rmc_request();
        let parameters = request.parameters.as_slice();

        let mut parameters_stream = StreamContainer::new(parameters);

        let param = parameters_stream
            .read_stream_le::<DataStorePreparePostParam>()
            .map_err(|_| "Can not read DataStorePreparePostParam")?;

        self.post_meta_binary(client, request.call_id, param)
    }

    fn handle_change_metas(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str> {
        let request = packet.get_rmc_request();
        let parameters = request.parameters.as_slice();

        let mut parameters_stream = StreamContainer::new(parameters);

        let data_ids = parameters_stream
            .read_stream_le::<NexList<u64>>()
            .map_err(|_| "Can not read data ids list")?;
        let params = parameters_stream
            .read_stream_le::<NexList<DataStoreChangeMetaParam>>()
            .map_err(|_| "Can not read DataStoreChangeMetaParam list")?;
        let transactional = parameters_stream
            .read_stream_le::<bool>()
            .map_err(|_| "Can not read transactional bool")?;

        self.change_metas(client, request.call_id, data_ids, params, transactional)
    }

    fn handle_prepare_upload_pokemon(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str> {
        self.prepare_upload_pokemon(client, packet.get_rmc_request().call_id)
    }

    fn handle_upload_pokemon(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str> {
        let request = packet.get_rmc_request();
        let parameters = request.parameters.as_slice();

        let mut parameters_stream = StreamContainer::new(parameters);

        let param = parameters_stream
            .read_stream_le::<GlobalTradeStationUploadPokemonParam>()
            .map_err(|_| "Can not read GlobalTradeStationUploadPokemonParam")?;

        self.upload_pokemon(client, param)
    }

    fn handle_prepare_trade_pokemon(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str> {
        let request = packet.get_rmc_request();
        let parameters = request.parameters.as_slice();

        let mut parameters_stream = StreamContainer::new(parameters);

        let param = parameters_stream
            .read_stream_le::<GlobalTradeStationPrepareTradePokemonParam>()
            .map_err(|_| "Can not read GlobalTradeStationPrepareTradePokemonParam")?;

        self.prepare_trade_pokemon(client, request.call_id, param)
    }

    fn handle_trade_pokemon(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str> {
        let request = packet.get_rmc_request();
        let parameters = request.parameters.as_slice();

        let mut parameters_stream = StreamContainer::new(parameters);

        let param = parameters_stream
            .read_stream_le::<GlobalTradeStationTradePokemonParam>()
            .map_err(|_| "Can not read GlobalTradeStationTradePokemonParam")?;

        self.trade_pokemon(client, request.call_id, param)
    }

    fn handle_download_other_pokemon(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str> {
        let request = packet.get_rmc_request();
        let parameters = request.parameters.as_slice();

        let mut parameters_stream = StreamContainer::new(parameters);

        let param = parameters_stream
            .read_stream_le::<GlobalTradeStationDownloadOtherPokemonParam>()
            .map_err(|_| "Can not read GlobalTradeStationDownloadOtherPokemonParam")?;

        self.download_other_pokemon(client, request.call_id, param)
    }

    fn handle_download_my_pokemon(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str> {
        let request = packet.get_rmc_request();
        let parameters = request.parameters.as_slice();

        let mut parameters_stream = StreamContainer::new(parameters);

        let param = parameters_stream
            .read_stream_le::<GlobalTradeStationDownloadMyPokemonParam>()
            .map_err(|_| "Can not read GlobalTradeStationDownloadMyPokemonParam")?;

        self.download_my_pokemon(client, request.call_id, param)
    }

    fn handle_delete_pokemon(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str> {
        let request = packet.get_rmc_request();
        let parameters = request.parameters.as_slice();

        let mut parameters_stream = StreamContainer::new(parameters);

        let param = parameters_stream
            .read_stream_le::<GlobalTradeStationDeletePokemonParam>()
            .map_err(|_| "Can not read GlobalTradeStationDeletePokemonParam")?;

        self.delete_pokemon(client, param)
    }

    fn handle_search_pokemon_v2(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str> {
        let request = packet.get_rmc_request();
        let parameters = request.parameters.as_slice();

        let mut parameters_stream = StreamContainer::new(parameters);

        let param = parameters_stream
            .read_stream_le::<GlobalTradeStationSearchPokemonParam>()
            .map_err(|_| "Can not read GlobalTradeStationSearchPokemonParam")?;

        self.search_pokemon_v2(client, request.call_id, param)
    }
}
