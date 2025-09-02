use std::fmt::{Display, Formatter};
use bdk_wallet::bitcoin::bip32::Xpriv;
use bdk_wallet::bitcoin::Network;
use bdk_wallet::KeychainKind;
use bdk_wallet::template::{Bip86, DescriptorTemplate};
use rand::RngCore;

pub struct Descriptor {
    pub privkey: String,
    pub change_privkey: String,
    pub pubkey: String,
    pub change_pubkey: String,
}

impl Display for Descriptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "pubkey: {}\nchange_pubkey: {}",
            self.pubkey,
            self.change_pubkey
        )
    }
}

impl Descriptor {
    pub fn new() -> Descriptor {
        let mut seed: [u8; 32] = [0u8; 32];
        rand::thread_rng().fill_bytes(&mut seed);

        let network: Network = Network::Signet;
        let xprv: Xpriv = Xpriv::new_master(network, &seed).unwrap();
        println!("Generated Master Private Key:\n{}\nWarning: be very careful with private keys when using MainNet! We are logging these values for convenience only because this is an example on SigNet.\n", xprv);

        let (descriptor, key_map, _) = Bip86(xprv, KeychainKind::External)
            .build(network)
            .expect("Failed to build external descriptor");

        let (change_descriptor, change_key_map, _) = Bip86(xprv, KeychainKind::Internal)
            .build(network)
            .expect("Failed to build internal descriptor");

        let descriptor_string_priv = descriptor.to_string_with_secret(&key_map);
        let change_descriptor_string_priv = change_descriptor.to_string_with_secret(&change_key_map);

        // println!(
        //     "----------------  Descriptors  ------------------------------\nPrivate Key, External:\n{:?}\nPrivate Key, Internal:\n{:?}\nPublic Key, External:\n{:?}\nPublic Key, Internal:\n{:?}\n",
        //     descriptor_string_priv, // privkey
        //     change_descriptor_string_priv,
        //     descriptor.to_string(), // pubkey
        //     change_descriptor.to_string()
        // );

        Descriptor {
            privkey: descriptor_string_priv,
            change_privkey: change_descriptor_string_priv,
            pubkey: descriptor.to_string(),
            change_pubkey: change_descriptor.to_string()
        }
    }
}