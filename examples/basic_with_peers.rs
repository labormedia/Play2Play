use play2play::{
    libp2p_node,
    redis_cache,
    AsyncCommands,
    identity::parse_legacy_multiaddr,
};
use libp2p::Multiaddr;
mod common;
use common::multi_addr;

#[tokio::main]
async fn main() {
    let local_key = libp2p::identity::Keypair::generate_ed25519();
    let public_key = local_key.public();

    let mut swarm = libp2p_node::create_swarm(local_key.clone()).await;
    let mut redis_conn = redis_cache::redis_connect().await;
    
    let _ = redis_conn.set::<_, _, ()>("event:null", format!("{:?}",public_key)).await;
    
    let dial_list = [multi_addr];
    
    // Reach out to other nodes if specified
    for to_dial in dial_list {
        let addr: Multiaddr = parse_legacy_multiaddr(&to_dial).unwrap();
        swarm.dial(addr).unwrap();
        println!("Dialed {to_dial:?}")
    }
    libp2p_node::listen_for_events(&mut swarm, &mut redis_conn, public_key.into()).await;
}