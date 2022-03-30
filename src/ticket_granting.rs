use nex_rs::client::ClientConnection;
use nex_rs::nex_types::{DataHolder, NexString};
use nex_rs::packet::{Packet, PacketV1};
use no_std_io::{EndianRead, EndianWrite, StreamContainer, StreamReader};
use num_enum::{IntoPrimitive, TryFromPrimitive};

pub const AUTHENTICATION_PROTOCOL_ID: u8 = 0xA;

#[derive(Debug, Clone, Copy, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum TicketGrantingMethod {
    Login = 0x1,
    LoginEx = 0x2,
    RequestTicket = 0x3,
    GetPID = 0x4,
    GetName = 0x5,
    LoginWithParam = 0x6,
}

#[derive(Default, EndianRead, EndianWrite)]
pub struct AuthenticationInfo {
    token: NexString,
    ngs_version: u32,
    token_type: u8,
    server_version: u32,
}

impl From<DataHolder<AuthenticationInfo>> for AuthenticationInfo {
    fn from(dh: DataHolder<AuthenticationInfo>) -> Self {
        dh.into()
    }
}

impl AuthenticationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}

pub trait TicketGrantingProtocol {
    fn login(
        &self,
        client: &mut ClientConnection,
        call_id: u32,
        username: String,
    ) -> Result<(), &'static str>;
    fn login_ex(
        &self,
        client: &mut ClientConnection,
        call_id: u32,
        username: String,
        ticket_granting_info: Option<AuthenticationInfo>,
    ) -> Result<(), &'static str>;
    fn request_ticket(
        &self,
        client: &mut ClientConnection,
        call_id: u32,
        user_pid: u32,
        server_pid: u32,
    ) -> Result<(), &'static str>;
    fn get_pid(
        &self,
        client: &mut ClientConnection,
        call_id: u32,
        username: String,
    ) -> Result<(), &'static str>;
    fn get_name(
        &self,
        client: &mut ClientConnection,
        call_id: u32,
        user_pid: u32,
    ) -> Result<(), &'static str>;
    fn login_with_param(
        &self,
        client: &mut ClientConnection,
        call_id: u32,
    ) -> Result<(), &'static str>;

    fn handle_login(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str> {
        let request = packet.get_rmc_request();
        let parameters = request.parameters.as_slice();
        let mut parameters_stream = StreamContainer::new(parameters);

        let username: String = parameters_stream
            .read_stream_le::<NexString>()
            .map_err(|_| "Can not read username")?
            .into();

        if username.trim() == String::default() {
            return Err("Failed to read username");
        }

        self.login(client, request.call_id, username)
    }

    fn handle_login_ex(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str> {
        let request = packet.get_rmc_request();
        let parameters = request.parameters.as_slice();
        let mut parameters_stream = StreamContainer::new(parameters);

        let username: String = parameters_stream
            .read_stream_le::<NexString>()
            .map_err(|_| "Can not read username")?
            .into();

        if username.trim() != String::default() {
            return Err("Failed to read username");
        }

        let data_holder = parameters_stream
            .read_stream_le::<DataHolder<AuthenticationInfo>>()
            .map_err(|_| "Can not read data holder")?;

        let data_holder_name: String = data_holder.get_name().into();

        if data_holder_name != "AuthenticationInfo" {
            return Err("Data holder name mismatch");
        }

        self.login_ex(client, request.call_id, username, Some(data_holder.into()))
    }

    fn handle_request_ticket(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str> {
        let request = packet.get_rmc_request();
        let parameters = request.parameters.as_slice();
        if parameters.len() != 8 {
            return Err("[TicketGrantingProtocol::request_ticket] Parameters length not 8");
        }

        let mut parameters_stream = StreamContainer::new(parameters);

        let user_pid: u32 = parameters_stream
            .read_stream_le()
            .map_err(|_| "[TicketGrantingProtocol::request_ticket] Failed to read user pid")?;
        let server_pid: u32 = parameters_stream
            .read_stream_le()
            .map_err(|_| "[TicketGrantingProtocol::request_ticket] Failed to read server pid")?;

        self.request_ticket(client, request.call_id, user_pid, server_pid)
    }

    fn handle_get_pid(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str> {
        let request = packet.get_rmc_request();
        let parameters = request.parameters.as_slice();
        let mut parameters_stream = StreamContainer::new(parameters);
        let username: String = parameters_stream
            .read_stream_le::<NexString>()
            .map_err(|_| "Can not read username")?
            .into();

        if username.trim() != String::default() {
            return Err("[TicketGrantingProtocol::get_pid] Failed to read username");
        }

        self.get_pid(client, request.call_id, username)
    }

    fn handle_get_name(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str> {
        let request = packet.get_rmc_request();
        let parameters = request.parameters.as_slice();

        if parameters.len() != 4 {
            return Err("[TicketGrantingProtocol::get_name] Parameters length not 4");
        }

        let mut parameters_stream = StreamContainer::new(parameters);

        let user_pid: u32 = parameters_stream
            .read_stream_le()
            .map_err(|_| "[TicketGrantingProtocol::get_name] Failed to read user PID")?;

        self.get_name(client, request.call_id, user_pid)
    }
}
