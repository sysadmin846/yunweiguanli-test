// HTTP API 模块 - 提供在线客户端查询接口
// 端口默认为 21119 (RustDesk 主端口 + 3)

use crate::peer::PeerMap;
use axum::{
    extract::Extension,
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use hbb_common::{log, ResultType};
use serde::Serialize;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};

const REG_TIMEOUT_SECS: u64 = 30;

#[derive(Serialize)]
pub struct OnlinePeer {
    pub id: String,
    pub ip: String,
    pub port: u16,
    pub online: bool,
    pub last_seen_seconds_ago: u64,
}

#[derive(Serialize)]
pub struct OnlinePeersResponse {
    pub peers: Vec<OnlinePeer>,
    pub total: usize,
    pub online_count: usize,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

/// 启动 HTTP API 服务器
pub async fn start_http_api(port: i32, pm: PeerMap) -> ResultType<()> {
    let api_port = (port + 4) as u16; // 默认 21120 (避免与 hbbr 的 21119 WebSocket 冲突)
    
    let pm = Arc::new(pm);
    
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);
    
    let app = Router::new()
        .route("/api/online-peers", get(get_online_peers))
        .route("/api/health", get(health_check))
        .layer(cors)
        .layer(Extension(pm));
    
    let addr = SocketAddr::from(([0, 0, 0, 0], api_port));
    log::info!("HTTP API listening on http://{}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    
    Ok(())
}

/// 获取在线客户端列表
async fn get_online_peers(
    Extension(pm): Extension<Arc<PeerMap>>,
) -> impl IntoResponse {
    let map = pm.map.read().await;
    
    let mut peers = Vec::new();
    let mut online_count = 0;
    
    for (id, lock_peer) in map.iter() {
        let peer = lock_peer.read().await;
        let elapsed = peer.last_reg_time.elapsed();
        let elapsed_secs = elapsed.as_secs();
        let is_online = elapsed_secs < REG_TIMEOUT_SECS;
        
        if is_online {
            online_count += 1;
        }
        
        // 只返回最近 1 小时内有活动的客户端
        if elapsed_secs < 3600 {
            // 处理 IPv6 映射的 IPv4 地址 (::ffff:x.x.x.x -> x.x.x.x)
            let ip = peer.info.ip.clone();
            let clean_ip = if ip.starts_with("::ffff:") {
                ip.strip_prefix("::ffff:").unwrap_or(&ip).to_string()
            } else {
                ip
            };
            
            peers.push(OnlinePeer {
                id: id.clone(),
                ip: clean_ip,
                port: peer.socket_addr.port(),
                online: is_online,
                last_seen_seconds_ago: elapsed_secs,
            });
        }
    }
    
    // 按在线状态和最后活跃时间排序
    peers.sort_by(|a, b| {
        match (a.online, b.online) {
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            _ => a.last_seen_seconds_ago.cmp(&b.last_seen_seconds_ago),
        }
    });
    
    let total = peers.len();
    
    Json(OnlinePeersResponse {
        peers,
        total,
        online_count,
    })
}

/// 健康检查端点
async fn health_check() -> impl IntoResponse {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}
