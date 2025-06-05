mod common;
use common::multi_addr;
use play2play::{
    libp2p_node,
    redis_cache,
    AsyncCommands,
};

#[tokio::main]
async fn main() {
    let local_key = libp2p::identity::Keypair::generate_ed25519();
    let public_key = local_key.public();

    let mut swarm = libp2p_node::create_swarm(local_key.clone()).await;
    let mut redis_conn = redis_cache::redis_connect().await;
    
    let _ = redis_conn.set::<_, _, ()>("event:null", format!("{:?}",public_key)).await;
    swarm.listen_on(multi_addr.parse().unwrap()).unwrap();
    //tokio::spawn(async move {
        libp2p_node::listen_for_events(&mut swarm, &mut redis_conn, public_key.into()).await;
    //});
}