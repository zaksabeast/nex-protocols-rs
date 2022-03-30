use nex_rs::nex_types::{DateTime, NexList, NexQBuffer, NexString, ResultCode, ResultRange};
use no_std_io::{EndianRead, EndianWrite};

#[derive(Debug, EndianRead, EndianWrite)]
pub struct GetMetasRequest {
    data_ids: NexList<u64>,
    param: DataStoreGetMetaParam,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct GetMetasResponse {
    p_meta_info: NexList<DataStoreMetaInfo>,
    p_results: NexList<ResultCode>,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct RateObjectRequest {
    target: DataStoreRatingTarget,
    param: DataStoreRateObjectParam,
    fetch_ratings: bool,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct RateObjectResponse {
    p_rating: DataStoreRatingInfo,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct DataStoreRatingInfoWithSlot {
    slot: i8,
    rating: DataStoreRatingInfo,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct DataStoreMetaInfo {
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
pub struct DataStoreRatingInfo {
    total_value: i64,
    count: u32,
    initial_value: i64,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct DataStorePersistenceTarget {
    owner_id: u32,
    persistence_slot_id: u16,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct DataStoreGetMetaParam {
    data_id: u64,
    persistence_target: DataStorePersistenceTarget,
    result_option: u8,
    access_password: u64,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct DataStoreRatingTarget {
    data_id: u64,
    slot: i8,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct DataStoreRateObjectParam {
    rating_value: i32,
    access_password: u64,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct DataStoreRatingInitParam {
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
pub struct DataStorePermission {
    permission: u8,
    recipient_ids: NexList<u32>,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct DataStoreRatingInitParamWithSlot {
    slot: i8,
    param: DataStoreRatingInitParam,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct DataStorePersistenceInitParam {
    persistence_slot_id: u16,
    delete_last_object: bool,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct DataStorePreparePostParam {
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
pub struct PostMetaBinaryRequest {
    param: DataStorePreparePostParam,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct PostMetaBinaryResponse {
    data_id: u64,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct DataStoreChangeMetaCompareParam {
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
pub struct DataStoreChangeMetaParam {
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
pub struct ChangeMetasRequest {
    data_ids: NexList<u64>,
    params: NexList<DataStoreChangeMetaParam>,
    transactional: bool,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct ChangeMetasResponse {
    p_results: NexList<ResultCode>,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct GlobalTradeStationRecordKey {
    data_id: u64,
    password: u64,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct PrepareUploadPokemonResponse {
    p_record_key: GlobalTradeStationRecordKey,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct GlobalTradeStationUploadPokemonParam {
    prepare_upload_key: GlobalTradeStationRecordKey,
    period: u16,
    index_data: NexQBuffer,
    pokemon_data: NexQBuffer,
    signature: NexQBuffer,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct UploadPokemonRequest {
    param: GlobalTradeStationUploadPokemonParam,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct GlobalTradeStationTradeKey {
    data_id: u64,
    version: u32,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct GlobalTradeStationPrepareTradePokemonParam {
    trade_key: GlobalTradeStationTradeKey,
    prepare_upload_key: GlobalTradeStationRecordKey,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct PrepareTradePokemonRequest {
    param: GlobalTradeStationPrepareTradePokemonParam,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct PrepareTradePokemonResponse {
    p_result: GlobalTradeStationPrepareTradePokemonResult,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct GlobalTradeStationDownloadPokemonResult {
    data_id: u64,
    index_data: NexQBuffer,
    pokemon_data: NexQBuffer,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct GlobalTradeStationPrepareTradePokemonResult {
    result: GlobalTradeStationDownloadPokemonResult,
    prepare_trade_key: GlobalTradeStationRecordKey,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct GlobalTradeStationTradePokemonParam {
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
pub struct TradePokemonRequest {
    param: GlobalTradeStationTradePokemonParam,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct TradePokemonResponse {
    p_result: GlobalTradeStationTradePokemonResult,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct GlobalTradeStationTradePokemonResult {
    result: GlobalTradeStationDownloadPokemonResult,
    my_data_id: u64,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct GlobalTradeStationDownloadOtherPokemonParam {
    prepare_upload_key: GlobalTradeStationRecordKey,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct DownloadOtherPokemonRequest {
    param: GlobalTradeStationDownloadOtherPokemonParam,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct DownloadOtherPokemonResponse {
    p_result: GlobalTradeStationTradePokemonResult,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct GlobalTradeStationDownloadMyPokemonParam {
    prepare_upload_key: GlobalTradeStationRecordKey,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct GlobalTradeStationDownloadMyPokemonResult {
    result: GlobalTradeStationDownloadPokemonResult,
    is_traded: bool,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct DownloadMyPokemonRequest {
    param: GlobalTradeStationDownloadMyPokemonParam,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct DownloadMyPokemonResponse {
    p_result: GlobalTradeStationDownloadMyPokemonResult,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct GlobalTradeStationDeletePokemonParam {
    prepare_upload_key: GlobalTradeStationRecordKey,
    delete_flag: u8,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct DeletePokemonRequest {
    param: GlobalTradeStationDeletePokemonParam,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct GlobalTradeStationSearchPokemonParam {
    prepare_upload_key: GlobalTradeStationRecordKey,
    conditions: NexList<u32>,
    result_order_column: u8,
    result_order: u8,
    uploaded_after: DateTime,
    uploaded_before: DateTime,
    result_range: ResultRange,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct SearchPokemonV2Request {
    param: GlobalTradeStationSearchPokemonParam,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct GlobalTradeStationData {
    data_id: u64,
    owner_id: u32,
    updated_time: DateTime,
    index_data: NexQBuffer,
    version: u32,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct GlobalTradeStationSearchPokemonResult {
    total_count: u32,
    result: NexList<GlobalTradeStationData>,
    total_count_type: u8,
}

#[derive(Debug, EndianRead, EndianWrite)]
pub struct SearchPokemonV2Response {
    p_result: GlobalTradeStationSearchPokemonResult,
}
