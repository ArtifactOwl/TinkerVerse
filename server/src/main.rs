mod fixed; mod feeds; mod types; mod ecs; mod sim; mod net_json;
use ecs::World; use fixed::*; use feeds::*; use net_json as net;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let world = World::new();
  let ship_id = world.spawn_ship();
  println!("spawned ship {}", ship_id);

  // resource asteroid
  let pos = V3 { x: to_fix(150.0), y: to_fix(0.0), z: to_fix(0.0) };
  let vel = V3 { x: to_fix(-0.2), y: to_fix(0.15), z: to_fix(0.0) };
  let mut comp = [0.0f64; FEED_DIM]; comp[FE_CARBON]=0.5; comp[FE_WATER]=0.4; comp[FE_NITROGEN]=0.1;
  let node_id = world.spawn_asteroid_node(pos, vel, comp, 1000.0, to_fix(14.0));
  println!("spawned resource asteroid {}", node_id);

  net::run(world).await
}
