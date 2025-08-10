use tokio::{net::TcpListener, time::{interval, Duration}};
use tokio_tungstenite::{accept_hdr_async, tungstenite::handshake::server::{Request, Response}};
use tokio_tungstenite::tungstenite::Message;
use futures::{StreamExt, SinkExt};
use anyhow::Result;
use serde_json::json;

use crate::{ecs::World, fixed::{TICK_HZ, to_f64, V3}, sim::{self, InputFrame}, types::Kind};

pub async fn run(world: World) -> Result<()> {
    let listener = TcpListener::bind(("127.0.0.1", 8080)).await?;
    println!("astral-server listening ws://127.0.0.1:8080");

    let (stream, _addr) = listener.accept().await?;
    let ws = accept_hdr_async(stream, |_req: &Request, resp: Response| Ok(resp)).await?;
    let (mut ws_tx, mut ws_rx) = ws.split();

    // give this client a ship if none exists
    let mut my_ship_id: u64 = 0;
    {
        let bodies = world.bodies.read();
        if let Some((_, b)) = bodies.iter().find(|(_, b)| matches!(b.kind, Kind::Ship)) { my_ship_id = b.id; }
    }
    if my_ship_id == 0 { my_ship_id = world.spawn_ship(); }

    let _ = ws_tx.send(Message::Text(json!({
        "type":"welcome", "ship_id": my_ship_id, "tick_hz": TICK_HZ
    }).to_string())).await;

    let mut queue: Vec<InputFrame> = Vec::new();
    let mut tick: u32 = 0;
    let mut ticker = interval(Duration::from_millis((1000 / TICK_HZ) as u64));
    loop {
        tokio::select! {
            _ = ticker.tick() => {
                tick = tick.wrapping_add(1);
                sim::step(&world, &queue);
                queue.clear();
                let snapshot = {
                    let bodies = world.bodies.read();
                    let ents: Vec<_> = bodies.values().map(|b| json!({
                        "id": b.id,
                        "kind": match b.kind { Kind::Ship=>"Ship", Kind::Projectile=>"Projectile", _=>"Unknown" },
                        "x": to_f64(b.pos.x), "y": to_f64(b.pos.y), "z": to_f64(b.pos.z)
                    })).collect();
                    json!({ "type":"snapshot", "tick": tick, "entities": ents })
                };
                let _ = ws_tx.send(Message::Text(snapshot.to_string())).await;
            }
            msg = ws_rx.next() => {
                match msg { Some(Ok(m)) => {
                    if m.is_text() {
                        if let Ok(v) = serde_json::from_str::<serde_json::Value>(&m.to_text().unwrap_or("")) {
                            if v.get("type").and_then(|t| t.as_str()) == Some("input") {
                                let id = v.get("id").and_then(|x| x.as_u64()).unwrap_or(my_ship_id);
                                let arr = v.get("thrust").and_then(|t| t.as_array()).cloned().unwrap_or_default();
                                if arr.len() == 3 {
                                    let tx = (arr[0].as_f64().unwrap_or(0.0) * 1_000_000.0) as i64;
                                    let ty = (arr[1].as_f64().unwrap_or(0.0) * 1_000_000.0) as i64;
                                    let tz = (arr[2].as_f64().unwrap_or(0.0) * 1_000_000.0) as i64;
                                    queue.push(InputFrame { id, thrust: V3 { x: tx, y: ty, z: tz }, fire:false });
                                }
                            }
                        }
                    }
                }, _ => return Ok(()) }
            }
        }
    }
}
