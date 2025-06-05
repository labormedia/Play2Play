use crate::event::Event;
use libp2p::{
    *, 
    swarm::{
        NetworkBehaviour,
        SwarmBuilder,
        SwarmEvent::Behaviour
    },
    futures::StreamExt,
};
use crate::redis_cache::{
    cache_event,
    AsyncCommands,
};
use tokio::select;


#[derive(NetworkBehaviour)]
pub struct MyBehaviour {
    kademlia: kad::Kademlia<kad::store::MemoryStore>,
    mdns: mdns::async_io::Behaviour,
}

pub async fn create_swarm(local_key: identity::Keypair) -> Swarm<MyBehaviour> {
    let peer_id = PeerId::from(local_key.public());
    let transport = development_transport(local_key.clone()).await.unwrap();

    let store = kad::store::MemoryStore::new(peer_id.clone());
    let behaviour = MyBehaviour {
        kademlia: kad::Kademlia::new(peer_id.clone(), store),
        mdns: mdns::Mdns::new(Default::default()).unwrap(),
    };

    SwarmBuilder::new(transport, behaviour, peer_id)
        //.executor(Box::new(|fut| { tokio::spawn(fut); }))
        .build()
}

pub async fn listen_for_events(swarm: &mut Swarm<MyBehaviour>, redis_conn: &mut redis::aio::MultiplexedConnection, public_key: identity::PublicKey) {
    loop {
        select! {
            event = swarm.select_next_some() => {
                // Process incoming libp2p events
                match event {
                    event => println!("Incoming Event {:?}", event),           
                }
            }            
        }

    }
}