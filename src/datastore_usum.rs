use nex_rs::client::ClientConnection;
use nex_rs::nex_types::{DateTime, NexList, NexQBuffer, NexString, ResultCode, ResultRange};
use nex_rs::packet::PacketV1;
use no_std_io::{EndianRead, EndianWrite};

#[derive(Debug, EndianRead, EndianWrite)]
struct GetMetasRequest {
    data_ids: NexList<u64>,
    param: DataStoreGetMetaParam,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct GetMetasResponse {
    p_meta_info: NexList<DataStoreMetaInfo>,
    p_results: NexList<ResultCode>,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct RateObjectRequest {
    target: DataStoreRatingTarget,
    param: DataStoreRateObjectParam,
    fetch_ratings: bool,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct RateObjectResponse {
    p_rating: DataStoreRatingInfo,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct DataStoreRatingInfoWithSlot {
    slot: i8,
    rating: DataStoreRatingInfo,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct DataStoreMetaInfo {
    data_id: u64,
    owner_id: u32,
    size: u32,
    name: NexString,
    data_type: u16,
    meta_binary: NexQBuffer,
    permission: DataStorePermission,
    del_permission: DataStorePermission,
    created_time: DateTime,
    updated_time: DateTime,
    period: u16,
    status: u8,
    referred_cnt: u32,
    refer_data_id: u32,
    flag: u32,
    referred_time: DateTime,
    expire_time: DateTime,
    tags: NexList<NexString>,
    ratings: NexList<DataStoreRatingInfoWithSlot>,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct DataStoreRatingInfo {
    total_value: i64,
    count: u32,
    initial_value: i64,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct DataStorePersistenceTarget {
    owner_id: u32,
    persistence_slot_id: u16,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct DataStoreGetMetaParam {
    data_id: u64,
    persistence_target: DataStorePersistenceTarget,
    result_option: u8,
    access_password: u64,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct DataStoreRatingTarget {
    data_id: u64,
    slot: i8,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct DataStoreRateObjectParam {
    rating_value: i32,
    access_password: u64,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct DataStoreRatingInitParam {
    flag: u8,
    internal_flag: u8,
    lock_type: u8,
    initial_value: i64,
    range_min: i32,
    range_max: i32,
    period_hour: i8,
    period_duration: i16,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct DataStorePermission {
    permission: u8,
    recipient_ids: NexList<u32>,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct DataStoreRatingInitParamWithSlot {
    slot: i8,
    param: DataStoreRatingInitParam,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct DataStorePersistenceInitParam {
    persistence_slot_id: u16,
    delete_last_object: bool,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct DataStorePreparePostParam {
    size: u32,
    name: NexString,
    data_type: u16,
    meta_binary: NexQBuffer,
    permission: DataStorePermission,
    del_permission: DataStorePermission,
    flag: u32,
    period: u16,
    refer_data_id: u32,
    tags: NexList<NexString>,
    rating_init_params: NexList<DataStoreRatingInitParamWithSlot>,
    persistence_init_param: DataStorePersistenceInitParam,
    extra_data: NexList<NexString>,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct PostMetaBinaryRequest {
    param: DataStorePreparePostParam,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct PostMetaBinaryResponse {
    data_id: u64,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct DataStoreChangeMetaCompareParam {
    comparison_flag: u32,
    name: NexString,
    permission: DataStorePermission,
    del_permission: DataStorePermission,
    period: u16,
    meta_binary: NexQBuffer,
    tags: NexList<NexString>,
    referred_cnt: u32,
    data_type: u16,
    status: u8,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct DataStoreChangeMetaParam {
    data_id: u64,
    modifies_flag: u32,
    name: NexString,
    permission: DataStorePermission,
    del_permission: DataStorePermission,
    period: u16,
    meta_binary: NexQBuffer,
    tags: NexList<NexString>,
    update_password: u64,
    referred_cnt: u32,
    data_type: u16,
    status: u8,
    compare_param: DataStoreChangeMetaCompareParam,
    persistence_target: DataStorePersistenceTarget,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct ChangeMetasRequest {
    data_ids: NexList<u64>,
    params: NexList<DataStoreChangeMetaParam>,
    transactional: bool,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct ChangeMetasResponse {
    p_results: NexList<ResultCode>,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct GlobalTradeStationRecordKey {
    data_id: u64,
    password: u64,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct PrepareUploadPokemonResponse {
    p_record_key: GlobalTradeStationRecordKey,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct GlobalTradeStationUploadPokemonParam {
    prepare_upload_key: GlobalTradeStationRecordKey,
    period: u16,
    index_data: NexQBuffer,
    pokemon_data: NexQBuffer,
    signature: NexQBuffer,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct UploadPokemonRequest {
    param: GlobalTradeStationUploadPokemonParam,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct GlobalTradeStationTradeKey {
    data_id: u64,
    version: u32,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct GlobalTradeStationPrepareTradePokemonParam {
    trade_key: GlobalTradeStationTradeKey,
    prepare_upload_key: GlobalTradeStationRecordKey,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct PrepareTradePokemonRequest {
    param: GlobalTradeStationPrepareTradePokemonParam,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct PrepareTradePokemonResponse {
    p_result: GlobalTradeStationPrepareTradePokemonResult,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct GlobalTradeStationDownloadPokemonResult {
    data_id: u64,
    index_data: NexQBuffer,
    pokemon_data: NexQBuffer,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct GlobalTradeStationPrepareTradePokemonResult {
    result: GlobalTradeStationDownloadPokemonResult,
    prepare_trade_key: GlobalTradeStationRecordKey,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct GlobalTradeStationTradePokemonParam {
    trade_key: GlobalTradeStationTradeKey,
    prepare_trade_key: GlobalTradeStationRecordKey,
    prepare_upload_key: GlobalTradeStationRecordKey,
    period: u16,
    index_data: NexQBuffer,
    pokemon_data: NexQBuffer,
    signature: NexQBuffer,
    need_data: bool,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct TradePokemonRequest {
    param: GlobalTradeStationTradePokemonParam,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct TradePokemonResponse {
    p_result: GlobalTradeStationTradePokemonResult,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct GlobalTradeStationTradePokemonResult {
    result: GlobalTradeStationDownloadPokemonResult,
    my_data_id: u64,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct GlobalTradeStationDownloadOtherPokemonParam {
    prepare_upload_key: GlobalTradeStationRecordKey,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct DownloadOtherPokemonRequest {
    param: GlobalTradeStationDownloadOtherPokemonParam,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct DownloadOtherPokemonResponse {
    p_result: GlobalTradeStationTradePokemonResult,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct GlobalTradeStationDownloadMyPokemonParam {
    prepare_upload_key: GlobalTradeStationRecordKey,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct GlobalTradeStationDownloadMyPokemonResult {
    result: GlobalTradeStationDownloadPokemonResult,
    is_traded: bool,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct DownloadMyPokemonRequest {
    param: GlobalTradeStationDownloadMyPokemonParam,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct DownloadMyPokemonResponse {
    p_result: GlobalTradeStationDownloadMyPokemonResult,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct GlobalTradeStationDeletePokemonParam {
    prepare_upload_key: GlobalTradeStationRecordKey,
    delete_flag: u8,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct DeletePokemonRequest {
    param: GlobalTradeStationDeletePokemonParam,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct GlobalTradeStationSearchPokemonParam {
    prepare_upload_key: GlobalTradeStationRecordKey,
    conditions: NexList<u32>,
    result_order_column: u8,
    result_order: u8,
    uploaded_after: DateTime,
    uploaded_before: DateTime,
    result_range: ResultRange,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct SearchPokemonV2Request {
    param: GlobalTradeStationSearchPokemonParam,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct GlobalTradeStationData {
    data_id: u64,
    owner_id: u32,
    updated_time: DateTime,
    index_data: NexQBuffer,
    version: u32,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct GlobalTradeStationSearchPokemonResult {
    total_count: u32,
    result: NexList<GlobalTradeStationData>,
    total_count_type: u8,
}

#[derive(Debug, EndianRead, EndianWrite)]
struct SearchPokemonV2Response {
    p_result: GlobalTradeStationSearchPokemonResult,
}

pub const DATASTORE_PROTOCOL_ID: u8 = 0x73;

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
