# Play2Play
P2P Playground tools.

## Requirements

Run Redis:
```bash
$ docker run -d --name redis -p 6379:6379 redis
```

## Run example

```bash
$ cargo run --example basic
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.12s
     Running `target/debug/examples/basic`
Incoming Event NewListenAddr { listener_id: ListenerId(14235683702915656305), address: "/ip4/127.0.0.1/tcp/4001" }}
$ cargo run --example basic_with_peers

    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.94s
     Running `target/debug/examples/basic_with_peers`
Dialed "/ip4/127.0.0.1/tcp/4001"
Incoming Event ConnectionEstablished { peer_id: PeerId("12D3KooWAkuPGQMqmSwEdEmx3NP7VrirucEQtjVonvjHtvHmAK2E"), endpoint: Dialer { address: "/ip4/127.0.0.1/tcp/4001", role_override: Dialer }, num_established: 1, concurrent_dial_errors: Some([]) }
Incoming Event Behaviour(Mdns(Discovered(DiscoveredAddrsIter)))
```