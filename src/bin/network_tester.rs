use std::env;
use futures::{prelude::*, future};
use libp2p::{PeerId, Transport};
use libp2p::tcp::TcpConfig;
use libp2p::identity::Keypair;
use libp2p::mplex::MplexConfig;
use libp2p::core::upgrade::SelectUpgrade;
use libp2p::core::transport::upgrade::{Builder, Version};
use libp2p::yamux;
use libp2p_plaintext;

fn main() {
    env_logger::init();

    // Create a random PeerId.
    let id_keys = Keypair::generate_ed25519();
    let peer_id = PeerId::from(id_keys.public());
    println!("Local peer id: {:?}", peer_id);

    let transport = TcpConfig::new();
    let builder = transport.upgrade(Version::V1)
        .authenticate(libp2p_plaintext::PlainText2Config{local_public_key: id_keys.public()});
    let res = builder.multiplex(MplexConfig::new());
}
