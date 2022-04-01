use nex_rs::client::ClientConnection;
use nex_rs::packet::{Packet, PacketV1};
use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Clone, Copy, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum HealthMethod {
    PingDaemon = 0x1,
    PingDatabase = 0x2,
    RunSanityCheck = 0x3,
    FixSanityErrors = 0x4,
}

pub trait HealthProtocol {
    fn ping_daemon(&self, client: &mut ClientConnection, call_id: u32) -> Result<(), &'static str>;
    fn ping_database(&self, client: &mut ClientConnection, call_id: u32) -> Result<(), &'static str>;
    fn run_sanity_check(&self, client: &mut ClientConnection, call_id: u32) -> Result<(), &'static str>;
    fn fix_sanity_errors(&self, client: &mut ClientConnection, call_id: u32) -> Result<(), &'static str>;

    fn handle_ping_daemon(&self, client: &mut ClientConnection, packet: &PacketV1) -> Result<(), &'static str> {
        self.ping_daemon(client, packet.get_rmc_request().call_id)
    }

    fn handle_ping_database(&self, client: &mut ClientConnection, packet: &PacketV1) -> Result<(), &'static str> {
        self.ping_database(client, packet.get_rmc_request().call_id)
    }

    fn handle_run_sanity_check(&self, client: &mut ClientConnection, packet: &PacketV1) -> Result<(), &'static str> {
        self.run_sanity_check(client, packet.get_rmc_request().call_id)
    }

    fn handle_fix_sanity_errors(&self, client: &mut ClientConnection, packet: &PacketV1) -> Result<(), &'static str> {
        self.fix_sanity_errors(client, packet.get_rmc_request().call_id)
    }
}