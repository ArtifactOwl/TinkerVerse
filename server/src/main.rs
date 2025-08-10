mod fixed; mod types; mod ecs; mod sim; mod seed; mod checksum;

// Default to JSON wire for the quickstart
mod net_json; use net_json as net;

// When you switch to FlatBuffers, replace the above with:
// mod net_fb; use net_fb as net;
// mod astral_generated; // created by flatc
// mod world_generated;  // created by flatc

use ecs::World;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let world = World::new();
    let ship_id = world.spawn_ship();
    println!("spawned ship {}", ship_id);
    net::run(world).await
}
