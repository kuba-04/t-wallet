mod descriptors;

use crate::descriptors::Descriptor;
use bdk_esplora::esplora_client::Builder;
use bdk_esplora::{EsploraExt, esplora_client};
use bdk_wallet::bitcoin::{Address, Amount, FeeRate, Network, Psbt};
use bdk_wallet::chain::spk_client::{FullScanRequestBuilder, FullScanResponse};
use bdk_wallet::rusqlite::Connection;
use bdk_wallet::{AddressInfo, KeychainKind, SignOptions, Wallet};
use std::io;
use std::process::exit;
use std::str::FromStr;

const DB_PATH: &str = "./my_db.db3";
const STOP_GAP: usize = 10;
const PARALLEL_REQUESTS: usize = 1;

fn main() {
    println!("=================");
    println!("##  T-Wallet   ##");
    println!("=================");
    println!();
    let mut descriptor_str = String::new();
    let mut change_descriptor_str = String::new();
    println!("Create new or load from descriptor:");
    println!("-new (n)");
    println!("-load (l)");
    let mut select = String::new();
    let _ = io::stdin().read_line(&mut select);
    match select.trim() {
        "n" => {
            let descriptor = Descriptor::new();
            println!("Creating a new Descriptor: {}", &descriptor.to_string());
            descriptor_str = descriptor.privkey;
            change_descriptor_str = descriptor.change_privkey;
        }
        "l" => {
            println!("Enter descriptor:");
            let mut descriptor_str_in = String::new();
            let _ = io::stdin().read_line(&mut descriptor_str_in);
            descriptor_str = descriptor_str_in.trim().to_string();

            println!("Enter descriptor for change addresses:");
            let mut change_descriptor_str_in = String::new();
            let _ = io::stdin().read_line(&mut change_descriptor_str_in);
            change_descriptor_str = change_descriptor_str_in.trim().to_string();
        }
        _ => exit(0),
    };

    // init the connection to the db
    let mut conn = Connection::open(DB_PATH).expect("Can't open the database");

    // create wallet
    let mut load_params = Wallet::load()
        .check_network(Network::Signet)
        .descriptor(KeychainKind::External, Some(descriptor_str.clone()))
        .descriptor(KeychainKind::Internal, Some(change_descriptor_str.clone()));
    if descriptor_str.contains("tprv") && change_descriptor_str.contains("tprv") {
        load_params = load_params.extract_keys();
    }
    let wallet_opt = load_params.load_wallet(&mut conn).unwrap();

    let mut wallet = if let Some(loaded_wallet) = wallet_opt {
        loaded_wallet
    } else {
        Wallet::create(descriptor_str, change_descriptor_str)
            .network(Network::Signet)
            .create_wallet(&mut conn)
            .unwrap()
    };

    // sync the wallet: (request transaction history for the wallet)
    let client: esplora_client::BlockingClient =
        Builder::new("https://blockstream.info/signet/api/").build_blocking();

    println!("Syncing wallet...");
    let full_scan_request: FullScanRequestBuilder<KeychainKind> = wallet.start_full_scan();
    let update: FullScanResponse<KeychainKind> = client
        .full_scan(full_scan_request, STOP_GAP, PARALLEL_REQUESTS)
        .unwrap();

    // apply the update from the full scan to the wallet

    // In cases where you are using new descriptors that do not have a balance yet,
    // the example will request a new address from the wallet and print it out so you can fund the wallet.
    wallet.apply_update(update).unwrap();

    let balance = wallet.balance();
    println!("Wallet Balance: {} sat", balance.total().to_sat());

    if balance.total().to_sat() < 5000 {
        println!("Your wallet does not have sufficient balance for the following steps!");
        // reveal a new address from your external keychain
        let address: AddressInfo = wallet.reveal_next_address(KeychainKind::External);
        println!(
            "Send Signet coins to {} (address generated at index {})",
            address.address, address.index
        );
        wallet.persist(&mut conn).unwrap();
        exit(0);
    }

    // use a faucet return address
    let faucet_address =
        Address::from_str("tb1p4tp4l6glyr2gs94neqcpr5gha7344nfyznfkc8szkreflscsdkgqsdent4")
            .unwrap()
            .require_network(Network::Signet)
            .unwrap();

    let send_amount: Amount = Amount::from_sat(5000);

    // broadcast the transaction
    let mut psbt = {
        let mut builder = wallet.build_tx();
        builder.add_recipient(faucet_address.script_pubkey(), send_amount);
        // builder.fee_rate(FeeRate::from_sat_per_vb(4).unwrap());
        builder.finish().unwrap()
    };

    let finalized = wallet.sign(&mut psbt, SignOptions::default()).unwrap();
    assert!(finalized, "Unable to finalize transaction");

    let tx = psbt.extract_tx().unwrap();
    client.broadcast(&tx).unwrap();
    println!("Transaction broadcasted! Txid: {}", tx.compute_txid());

    let balance = wallet.balance();
    println!("Wallet Balance: {} sat", balance.total().to_sat());
}
