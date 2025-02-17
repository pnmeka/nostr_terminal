use bech32::{FromBase32};
use futures_util::{SinkExt, StreamExt};
use secp256k1::{Secp256k1, KeyPair, Message};
use serde::{Serialize, Deserialize};
use serde_json::{json};
use sha2::{Digest};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message as WsMessage};
use anyhow::{Result, anyhow};
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
struct Event {
    id: String,
    pubkey: String, // hex-encoded string of XOnlyPublicKey
    created_at: i64,
    kind: u64,
    tags: Vec<Vec<String>>,
    content: String,
    sig: String, // hex-encoded string of Signature
}

impl Event {
    fn new(secret_key: &[u8], kind: u64, tags: Vec<Vec<String>>, content: String) -> Result<Self> {
        let secp = Secp256k1::new();
        let keypair = KeyPair::from_secret_key(&secp, &secp256k1::SecretKey::from_slice(secret_key)?);
        let (pubkey, _parity) = keypair.x_only_public_key();
        let created_at = Utc::now().timestamp();
        
        let serialized_event = json!([
            0,
            hex::encode(pubkey.serialize()),
            created_at,
            kind,
            tags,
            content
        ]);
        
        let event_hash = sha2::Sha256::digest(serialized_event.to_string().as_bytes());
        let msg = Message::from_slice(&event_hash)?;
        
        let sig = secp.sign_schnorr(&msg, &keypair);
        
        Ok(Self {
            id: hex::encode(event_hash),
            pubkey: hex::encode(pubkey.serialize()),
            created_at,
            kind,
            tags,
            content,
            sig: hex::encode(sig.as_ref()),
        })
    }
}

pub fn nsec_to_hex(nsec: &str) -> Result<String> {
    // Add nsec1 prefix if not present
    let clean_nsec = if nsec.starts_with("nsec1") {
        nsec.to_string()
    } else {
        format!("nsec1{}", nsec)
    };

    // Decode bech32
    let (_hrp, data, _variant) = bech32::decode(&clean_nsec)
        .map_err(|e| anyhow!("Invalid bech32 format: {}", e))?;

    // Convert from 5-bit to 8-bit
    let decoded = Vec::<u8>::from_base32(&data)
        .map_err(|e| anyhow!("Error converting bits: {}", e))?;

    // Convert to hex
    let hex_key = hex::encode(&decoded);
    println!("Secret key (hex): {}", hex_key);
    
    Ok(hex_key)
}

async fn send_hello_nostr_with_message(secret_hex: &str, message: &str) -> Result<()> {
    // Decode the secret key
    let secret_bytes = hex::decode(secret_hex)?;
    
    // Create the event
    let event = Event::new(
        &secret_bytes,
        1,
        vec![],
        message.to_string()
    )?;

    // Connect to relay
    let (mut ws_stream, _) = connect_async("wss://relay.damus.io").await?;
    
    // Send event
    let message = json!(["EVENT", event]);
    ws_stream.send(WsMessage::Text(message.to_string())).await?;
    println!("Event sent successfully!");

    // Wait for confirmation
    if let Some(msg) = ws_stream.next().await {
        let response = msg?;
        println!("Relay response: {}", response);
    }

    // Small delay
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    
    // Check if message argument is provided
    if args.len() < 2 {
        println!("Usage: {} <message>", args[0]);
        std::process::exit(1);
    }

    // Join all arguments after the program name to form the message
    let message = args[1..].join(" ");
    
    let secret_hex = nsec_to_hex("nsec1pjh5ggul8nufpax4wq224rgzddx3kxs3t5cpjj2vnavfz0mra45s49m0k6")?;
    send_hello_nostr_with_message(&secret_hex, &message).await?;
    Ok(())
}