use std::error::Error;
use steam_vent::connection::Connection;
use steam_vent_proto::steammessages_gameservers_steamclient::CGameServers_GetServerList_Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();
    let (mut connection, rest) = Connection::anonymous().await?;
    
    let mut req = CGameServers_GetServerList_Request::new();
    req.set_limit(16);
    req.set_filter("\\appid\\440".into());
    let rx = connection.service_method(req).await?;
    let some_tf2_servers = rx.await??;
    
    for server in some_tf2_servers.servers {
        println!(
            "{}({}) playing {}",
            server.get_name(),
            server.get_addr(),
            server.get_map()
        );
    }

    Ok(())
}
