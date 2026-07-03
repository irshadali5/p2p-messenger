// network/node.rs
use iroh::{Endpoint, NodeAddr, ProtocolHandler};
use iroh_gossip::net::Gossip;
use std::sync::Arc;

pub struct P2pNode {
    endpoint: Endpoint,
    gossip: Arc<Gossip>,
    // Custom ALPN for your messaging protocol
    alpn: Vec<u8>,
}

impl P2pNode {
    pub async fn new(secret_key: iroh::SecretKey) -> anyhow::Result<Self> {
        let endpoint = Endpoint::builder()
            .secret_key(secret_key)
            .alpns(vec![b"/p2p-messenger/1".to_vec()])
            .bind()
            .await?;

        let gossip = Arc::new(Gossip::from_endpoint(
            endpoint.clone(),
            Default::default(),
            &endpoint.node_id(),
        ));

        Ok(Self {
            endpoint,
            gossip,
            alpn: b"/p2p-messenger/1".to_vec(),
        })
    }
}
