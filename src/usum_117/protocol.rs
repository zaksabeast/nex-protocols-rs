use async_trait::async_trait;
use nex_rs::client::ClientConnection;
use nex_rs::nex_types::ResultCode;
use nex_rs::packet::{Packet, PacketV1};
use nex_rs::server::Server;
use num_enum::{IntoPrimitive, TryFromPrimitive};

pub const USUM_117_PROTOCOL_ID: u8 = 0x75;

#[derive(Debug, Clone, Copy, PartialEq, TryFromPrimitive, IntoPrimitive)]
#[repr(u32)]
pub enum USUM117Method {
    Unknown1 = 0x1,
    Unknown7 = 0x7,
    Unknown9 = 0x9,
    Unknown10 = 0xA,
    Unknown15 = 0xF,
}

#[async_trait(?Send)]
pub trait USUM117Protocol: Server {
    async fn unknown_1(&self, client: &mut ClientConnection) -> Result<Vec<u8>, ResultCode>;
    async fn unknown_7(&self, client: &mut ClientConnection) -> Result<Vec<u8>, ResultCode>;
    async fn unknown_9(&self, client: &mut ClientConnection) -> Result<Vec<u8>, ResultCode>;
    async fn unknown_10(&self, client: &mut ClientConnection) -> Result<Vec<u8>, ResultCode>;
    async fn unknown_15(&self, client: &mut ClientConnection) -> Result<Vec<u8>, ResultCode>;

    async fn handle_unknown_1(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str> {
        let request = packet.get_rmc_request();

        match self.unknown_1(client).await {
            Ok(data) => {
                self.send_success(
                    client,
                    request.protocol_id,
                    request.method_id,
                    request.call_id,
                    data,
                )
                .await?
            }
            Err(error_code) => {
                self.send_error(
                    client,
                    request.protocol_id,
                    request.method_id,
                    request.call_id,
                    error_code.into(),
                )
                .await?
            }
        }
        Ok(())
    }

    async fn handle_unknown_7(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str> {
        let request = packet.get_rmc_request();

        match self.unknown_7(client).await {
            Ok(data) => {
                self.send_success(
                    client,
                    request.protocol_id,
                    request.method_id,
                    request.call_id,
                    data,
                )
                .await?
            }
            Err(error_code) => {
                self.send_error(
                    client,
                    request.protocol_id,
                    request.method_id,
                    request.call_id,
                    error_code.into(),
                )
                .await?
            }
        }
        Ok(())
    }

    async fn handle_unknown_9(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str> {
        let request = packet.get_rmc_request();

        match self.unknown_9(client).await {
            Ok(data) => {
                self.send_success(
                    client,
                    request.protocol_id,
                    request.method_id,
                    request.call_id,
                    data,
                )
                .await?
            }
            Err(error_code) => {
                self.send_error(
                    client,
                    request.protocol_id,
                    request.method_id,
                    request.call_id,
                    error_code.into(),
                )
                .await?
            }
        }
        Ok(())
    }

    async fn handle_unknown_10(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str> {
        let request = packet.get_rmc_request();

        match self.unknown_10(client).await {
            Ok(data) => {
                self.send_success(
                    client,
                    request.protocol_id,
                    request.method_id,
                    request.call_id,
                    data,
                )
                .await?
            }
            Err(error_code) => {
                self.send_error(
                    client,
                    request.protocol_id,
                    request.method_id,
                    request.call_id,
                    error_code.into(),
                )
                .await?
            }
        }
        Ok(())
    }

    async fn handle_unknown_15(
        &self,
        client: &mut ClientConnection,
        packet: &PacketV1,
    ) -> Result<(), &'static str> {
        let request = packet.get_rmc_request();

        match self.unknown_15(client).await {
            Ok(data) => {
                self.send_success(
                    client,
                    request.protocol_id,
                    request.method_id,
                    request.call_id,
                    data,
                )
                .await?
            }
            Err(error_code) => {
                self.send_error(
                    client,
                    request.protocol_id,
                    request.method_id,
                    request.call_id,
                    error_code.into(),
                )
                .await?
            }
        }
        Ok(())
    }
}
