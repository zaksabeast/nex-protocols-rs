use nex_rs::client::ClientConnection;
use nex_rs::packet::{Packet, PacketV1};
use no_std_io::{StreamContainer, StreamReader};
use num_enum::{IntoPrimitive, TryFromPrimitive};

pub const MATCHMAKE_EXTENSION_PROTOCOL_ID: u8 = 0x6D;

#[derive(Debug, Clone, Copy, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MatchmakeExtensionMethod {
    GetAttractionStatus = 0x31,
    SimpleMatchmake = 0x33
}

pub trait MatchmakeExtensionProtocol {
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