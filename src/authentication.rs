use nex_rs::client::{ClientConnection};
use nex_rs::nex_types::StructureInterface;
use nex_rs::packet::{Packet, PacketV1};
use nex_rs::stream::{StreamIn};
use no_std_io::{Cursor, Reader, StreamReader};
use num_enum::{IntoPrimitive, TryFromPrimitive};

pub const AUTHENTICATION_PROTOCOL_ID: u8 = 0xA;

#[derive(Debug, Clone, Copy, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum AuthenticationMethod {
    Login = 0x1,
    LoginEx = 0x2,
    RequestTicket = 0x3,
    GetPID = 0x4,
    GetName = 0x5,
    LoginWithParam = 0x6,
}

#[derive(Default)]
pub struct NintendoLoginData(String);

#[derive(Default)]
pub struct AuthenticationInfo {
    token: String,
    ngs_version: u32,
    token_type: u8,
    server_version: u32,
}

impl AuthenticationInfo {
    pub fn new() -> Self {
        Self::default()
    }
}

impl StructureInterface for AuthenticationInfo {
    fn extract_from_stream<T: Reader>(&mut self, stream: &mut StreamIn<T>) -> Result<(), &'static str> {
        let token = stream.read_string();
        if stream.get_slice()[stream.get_index()..].len() < 9 {
            return Err("[AuthenticationInfo::extract_from_stream] Data size too small");
        }

        self.token = token;
        self.token_type = stream.read_stream()
            .map_err(|_| "[AuthenticationInfo::extract_from_stream] Failed to read token type")?;
        self.ngs_version = stream.read_stream_le()
            .map_err(|_| "[AuthenticationInfo::extract_from_stream] Failed to read NGS version")?;
        self.server_version = stream.read_stream_le()
            .map_err(|_| "[AuthenticationInfo::extract_from_stream] Failed to read server version")?;

        Ok(())
    }
}

pub trait AuthenticationProtocol {
    fn login(&self, client: &mut ClientConnection, call_id: u32, username: String) -> Result<(), &'static str>;
    fn login_ex(&self, client: &mut ClientConnection, call_id: u32, username: String, authentication_info: Option<AuthenticationInfo>) -> Result<(), &'static str>;
    fn request_ticket(&self, client: &mut ClientConnection, call_id: u32, user_pid: u32, server_pid: u32) -> Result<(), &'static str>;
    fn get_pid(&self, client: &mut ClientConnection, call_id: u32, username: String) -> Result<(), &'static str>;
    fn get_name(&self, client: &mut ClientConnection, call_id: u32, user_pid: u32) -> Result<(), &'static str>;
    fn login_with_param(&self, client: &mut ClientConnection, call_id: u32) -> Result<(), &'static str>;

    fn handle_login(&self, client: &mut ClientConnection, packet: &PacketV1) -> Result<(), &'static str> {
        let request = packet.get_rmc_request();

        let mut parameters_stream = StreamIn::new(request.parameters.clone());

        let username = parameters_stream.read_string();

        if username.trim() == String::default() {
            return Err("Failed to read username");
        }

        self.login(client, request.call_id, username)
    }

    fn handle_login_ex(&self, client: &mut ClientConnection, packet: &PacketV1) -> Result<(), &'static str> {
        let request = packet.get_rmc_request();

        let mut parameters_stream = StreamIn::new(request.parameters.clone());

        let username = parameters_stream.read_string();

        if username.trim() != String::default() {
            return Err("Failed to read username");
        }

        let data_holder_name = parameters_stream.read_string();

        if data_holder_name.trim() == String::default() {
            return Err("Failed to read data holder name");
        }

        if data_holder_name.trim() != "AuthenticationInfo" {
            return Err("[AuthenticationProtocol::login_ex] Data holder name does not match");
        }

        let _: u32 = parameters_stream.read_stream_le().map_err(|_| "[AuthenticationProtocol::login_ex] Failed to skip misc item")?;

        let data_holder_content = parameters_stream.read_buffer();

        if data_holder_content.is_empty() {
            return Err("Data holder content is empty");
        }

        let mut data_holder_content_stream = StreamIn::new(data_holder_content);

        let mut authentication_info = AuthenticationInfo::default();
        authentication_info.extract_from_stream(&mut data_holder_content_stream)?;

        self.login_ex(client, request.call_id, username, Some(authentication_info))
    }

    fn handle_request_ticket(&self, client: &mut ClientConnection, packet: &PacketV1) -> Result<(), &'static str> {
        let request = packet.get_rmc_request();
        let parameters = &request.parameters;
        if parameters.len() != 8 {
            return Err("[AuthenticationProtocol::request_ticket] Parameters length not 8");
        }

        let mut parameters_stream = StreamIn::new(parameters.clone());

        let user_pid: u32 = parameters_stream.read_stream_le()
            .map_err(|_| "[AuthenticationProtocol::request_ticket] Failed to read user pid")?;
        let server_pid: u32 = parameters_stream.read_stream_le()
            .map_err(|_| "[AuthenticationProtocol::request_ticket] Failed to read server pid")?;

        self.request_ticket(client, request.call_id, user_pid, server_pid)
    }

    fn handle_get_pid(&self, client: &mut ClientConnection, packet: &PacketV1) -> Result<(), &'static str> {
        let request = packet.get_rmc_request();
        let mut parameters_stream = StreamIn::new(request.parameters.clone());
        let username = parameters_stream.read_string();

        if username.trim() != String::default() {
            return Err("[AuthenticationProtocol::get_pid] Failed to read username");
        }

        self.get_pid(client, request.call_id, username)
    }

    fn handle_get_name(&self, client: &mut ClientConnection, packet: &PacketV1) -> Result<(), &'static str> {
        let request = packet.get_rmc_request();

        if request.parameters.len() != 4 {
            return Err("[AuthenticationProtocol::get_name] Parameters length not 4");
        }

        let mut parameters_stream = StreamIn::new(request.parameters.clone());

        let user_pid: u32 = parameters_stream.read_stream_le().map_err(|_| "[AuthenticationProtocol::get_name] Failed to read user PID")?;

        self.get_name(client, request.call_id, user_pid)
    }
}