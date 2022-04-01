use nex_rs::client::ClientConnection;
use nex_rs::packet::{Packet, PacketV1};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Clone, Copy, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum MonitoringMethod {
    PingDaemon = 0x1,
    GetClusterMembers = 0x2,
}

pub trait MonitoringProtocol {
    fn ping_daemon(&self, client: &mut ClientConnection, call_id: u32) -> Result<(), &'static str>;
    fn get_cluster_members(&self, client: &mut ClientConnection, call_id: u32) -> Result<(), &'static str>;

    fn handle_ping_daemon(&self, client: &mut ClientConnection, packet: &PacketV1) -> Result<(), &'static str> {
        self.ping_daemon(client, packet.get_rmc_request().call_id)
    }

    fn handle_get_cluster_members(&self, client: &mut ClientConnection, packet: &PacketV1) -> Result<(), &'static str> {
        self.get_cluster_members(client, packet.get_rmc_request().call_id)
    }
}