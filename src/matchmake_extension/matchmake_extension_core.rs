use crate::matchmake_extension::MatchmakeSessionSearchCriteria;
use nex_rs::client::ClientConnection;
use nex_rs::nex_types::ResultRange;
use nex_rs::packet::{Packet, PacketV1};
use no_std_io::{StreamContainer, StreamReader};
use num_enum::{IntoPrimitive, TryFromPrimitive};

pub const MATCHMAKE_EXTENSION_PROTOCOL_ID: u8 = 0x6D;

#[derive(Debug, Clone, Copy, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MatchmakeExtensionMethod {
    CloseParticipation = 0x1,
    OpenParticipation = 0x2,
    BrowseMatchmakeSession = 0x4,
    BrowseMatchmakeSessionWithHostUrls = 0x5,
    GetAttractionStatus = 0x31,
    SimpleMatchmake = 0x33,
}

pub trait MatchmakeExtensionProtocol {
    fn close_participation(
        &self,
        client: &mut ClientConnection,
        call_id: u32,
        gid: u32,
    ) -> Result<(), &'static str>;
    fn open_participation(
        &self,
        client: &mut ClientConnection,
        call_id: u32,
        gid: u32,
    ) -> Result<(), &'static str>;
    fn browse_matchmake_session(
        &self,
        client: &mut ClientConnection,
        call_id: u32,
        matchmake_session_search_criteria: MatchmakeSessionSearchCriteria,
        result_range: ResultRange,
    ) -> Result<(), &'static str>;
    fn browse_matchmake_session_with_host_urls(
        &self,
        client: &mut ClientConnection,
        call_id: u32,
        matchmake_session_search_criteria: MatchmakeSessionSearchCriteria,
        result_range: ResultRange,
    ) -> Result<(), &'static str>;
    fn get_attraction_status(
        &self,
        client: &mut ClientConnection,
        call_id: u32,
    ) -> Result<(), &'static str>;
    fn simple_matchmake(
        &self,
        client: &mut ClientConnection,
        call_id: u32,
        group_id: u32,
    ) -> Result<(), &'static str>;

    fn handle_close_participation(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str> {
        let request = packet.get_rmc_request();
        let parameters = request.parameters.as_slice();
        let mut parameters_stream = StreamContainer::new(parameters);

        let gid = parameters_stream
            .read_stream_le::<u32>()
            .map_err(|_| "Can not read group id")?;

        self.close_participation(client, packet.get_rmc_request().call_id, gid)
    }

    fn handle_open_participation(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str> {
        let request = packet.get_rmc_request();
        let parameters = request.parameters.as_slice();
        let mut parameters_stream = StreamContainer::new(parameters);

        let gid = parameters_stream
            .read_stream_le::<u32>()
            .map_err(|_| "Can not read group id")?;

        self.open_participation(client, packet.get_rmc_request().call_id, gid)
    }

    fn handle_browse_matchmake_session(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str> {
        let request = packet.get_rmc_request();
        let parameters = request.parameters.as_slice();
        let mut parameters_stream = StreamContainer::new(parameters);

        let matchmake_session_search_criteria = parameters_stream
            .read_stream_le::<MatchmakeSessionSearchCriteria>()
            .map_err(|_| "Can not read matchmake session search criteria")?;

        let result_range = parameters_stream
            .read_stream_le::<ResultRange>()
            .map_err(|_| "Can not read result range")?;

        self.browse_matchmake_session(
            client,
            packet.get_rmc_request().call_id,
            matchmake_session_search_criteria,
            result_range,
        )
    }

    fn handle_browse_matchmake_session_with_host_urls(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str> {
        let request = packet.get_rmc_request();
        let parameters = request.parameters.as_slice();
        let mut parameters_stream = StreamContainer::new(parameters);

        let matchmake_session_search_criteria = parameters_stream
            .read_stream_le::<MatchmakeSessionSearchCriteria>()
            .map_err(|_| "Can not read matchmake session search criteria")?;

        let result_range = parameters_stream
            .read_stream_le::<ResultRange>()
            .map_err(|_| "Can not read result range")?;

        self.browse_matchmake_session_with_host_urls(
            client,
            packet.get_rmc_request().call_id,
            matchmake_session_search_criteria,
            result_range,
        )
    }

    fn handle_get_attraction_status(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str> {
        self.get_attraction_status(client, packet.get_rmc_request().call_id)
    }

    fn handle_simple_matchmake(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str> {
        let request = packet.get_rmc_request();
        let parameters = request.parameters.as_slice();
        let mut parameters_stream = StreamContainer::new(parameters);

        let group_id = parameters_stream
            .read_stream_le::<u32>()
            .map_err(|_| "Can not read group id")?;

        self.simple_matchmake(client, packet.get_rmc_request().call_id, group_id)
    }
}
