use std::net::SocketAddr;
use serde::Serialize;

#[derive(Serialize)]
pub struct TcpRow {
    state: TcpState,
    local: SocketAddr,
    remote: SocketAddr,
}

#[derive(Serialize)]
enum TcpState {
    Closed,
    Listen,
    #[serde(rename = "SYN-Sent")]
    SynSent,
    #[serde(rename = "SYN-Received")]
    SynReceived,
    Established,
    #[serde(rename = "FIN-Wait-1")]
    FinWait1,
    #[serde(rename = "FIN-Wait-2")]
    FinWait2,
    #[serde(rename = "Close-Wait")]
    CloseWait,
    Closing,
    #[serde(rename = "Last-ACK")]
    LastAck,
    #[serde(rename = "Time-Wait")]
    TimeWait,
    #[serde(rename = "Delete-TCB")]
    DeleteTcb,
}
