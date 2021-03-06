extern crate ssz;

use ssz::Decode;

use types::{
    Attestation, AttesterSlashing, BeaconBlock, BeaconState, Deposit, MainnetEthSpec,
    ProposerSlashing, SignedBeaconBlock, SignedVoluntaryExit,
};

mod attestation;
#[inline(always)]
pub fn fuzz_lighthouse_attestation(beaconstate: BeaconState<MainnetEthSpec>, data: &[u8]) {
    // Verify that data is a correct Attestation ssz
    let attestation = match Attestation::from_ssz_bytes(&data) {
        Ok(attestation) => attestation,
        Err(_e) => return,
    };

    let _ = attestation::process_attestation(beaconstate, attestation);
}

mod attester_slashing;
#[inline(always)]
pub fn fuzz_lighthouse_attester_slashing(beaconstate: BeaconState<MainnetEthSpec>, data: &[u8]) {
    let attester_slashing = match AttesterSlashing::from_ssz_bytes(&data) {
        Ok(attester_slashing) => attester_slashing,
        Err(_e) => return,
    };

    let _ = attester_slashing::process_attester_slashing(beaconstate, attester_slashing);
}

mod block;
#[inline(always)]
pub fn fuzz_lighthouse_block(beaconstate: BeaconState<MainnetEthSpec>, data: &[u8]) {
    let block = match SignedBeaconBlock::from_ssz_bytes(&data) {
        Ok(block) => block,
        Err(_e) => return,
    };

    let _ = block::state_transition(beaconstate, block, true);
}

mod block_header;
#[inline(always)]
pub fn fuzz_lighthouse_block_header(beaconstate: BeaconState<MainnetEthSpec>, data: &[u8]) {
    let block = match BeaconBlock::from_ssz_bytes(&data) {
        Ok(block) => block,
        Err(_e) => return,
    };

    let _ = block_header::process_header(beaconstate, block);
}

mod deposit;
#[inline(always)]
pub fn fuzz_lighthouse_deposit(beaconstate: BeaconState<MainnetEthSpec>, data: &[u8]) {
    let deposit = match Deposit::from_ssz_bytes(&data) {
        Ok(deposit) => deposit,
        Err(_e) => return,
    };

    let _ = deposit::process_deposit(beaconstate, deposit);
}

mod proposer_slashing;
#[inline(always)]
pub fn fuzz_lighthouse_proposer_slashing(beaconstate: BeaconState<MainnetEthSpec>, data: &[u8]) {
    let proposer_slashing = match ProposerSlashing::from_ssz_bytes(&data) {
        Ok(proposer_slashing) => proposer_slashing,
        Err(_e) => return,
    };

    let _ = proposer_slashing::process_proposer_slashing(beaconstate, proposer_slashing);
}

mod voluntary_exit;
#[inline(always)]
pub fn fuzz_lighthouse_voluntary_exit(beaconstate: BeaconState<MainnetEthSpec>, data: &[u8]) {
    let voluntary_exit = match SignedVoluntaryExit::from_ssz_bytes(&data) {
        Ok(voluntary_exit) => voluntary_exit,
        Err(_e) => return,
    };

    let _ = voluntary_exit::process_voluntary_exit(beaconstate, voluntary_exit);
}

mod beaconstate;
#[inline(always)]
pub fn fuzz_lighthouse_beaconstate(data: &[u8]) {
    let mut beaconstate = match BeaconState::from_ssz_bytes(&data) {
        Ok(beaconstate) => beaconstate,
        _ => return,
    };
    beaconstate::fuzz_beaconstate_accessors(&mut beaconstate);
}

/* libp2p */

#[inline(always)]
pub fn fuzz_lighthouse_enr(data: &[u8]) {
    // TODO - could be improved
    // will be better to craft "enr:" + base64encode(data)
    use eth2_libp2p::Enr;
    use std::str;
    use std::str::FromStr;
    // data will be convert into str first
    let d = match str::from_utf8(&data) {
        Ok(d) => d,
        _ => return,
    };
    let _a = Enr::from_str(&d);
    //println!("{:?}", a);
}

/* BLS */

#[inline(always)]
pub fn fuzz_lighthouse_bls(data: &[u8]) {
    use bls::Signature;
    let _ = Signature::deserialize(&data);
}

/* discv5 */
pub fn fuzz_lighthouse_discv5_packet(data: &[u8]) {
    use eth2_libp2p::discv5::packet::Packet;

    // around half chance to trigger whoareyou packet
    if data.len() > 32 && data[0] > 126 {
        let mut magic_data = [0u8; 32];
        magic_data.copy_from_slice(&data[..32]);
        if let Ok(packet) = Packet::decode(&data[32..], &magic_data) {
            packet.encode();
        }
    }
}

// From beacon_node/eth2-libp2p/src/rpc/protocol.rs
// const MAX_RPC_SIZE: usize = 1_048_576; // 1M

/* snappy
pub fn fuzz_lighthouse_snappy(data: &[u8]) {
    use eth2_libp2p::rpc::{Encoding, Protocol, ProtocolId, SSZSnappyInboundCodec, Version};
    use libp2p::bytes::BytesMut;
    use tokio_util::codec::Decoder;

    let protocol = ProtocolId::new(Protocol::Status, Version::V1, Encoding::SSZSnappy);
    let mut codec = SSZSnappyInboundCodec::<MainnetEthSpec>::new(protocol, MAX_RPC_SIZE);
    let mut buffer = BytesMut::from(&data);
    let _ = codec.decode(&mut buffer);


    /// The Status protocol name.
    //Status,
    /// The Goodbye protocol name.
    //Goodbye,
    /// The `BlocksByRange` protocol name.
    //BlocksByRange,
    /// The `BlocksByRoot` protocol name.
    //BlocksByRoot,
    /// The `Ping` protocol name.
    //Ping,s
    /// The `MetaData` protocol name.
    //MetaData,

}
*/
/* gossip
pub fn fuzz_lighthouse_gossip(data: &[u8]) {
    use bytes::BytesMut;
    use eth2_libp2p::GossipTopic::{protocol::GossipsubCodec, ValidationMode};
    use futures_codec::{Decoder, Encoder};
    use unsigned_varint::codec;

    let mut codec = GossipsubCodec::new(codec::UviBytes::default(), ValidationMode::Anonymous);
    let mut buf: BytesMut = data.into();
    let _ = codec.decode(&mut buf);
} */
