use crate::fixed::*;
use crate::types::*;
use crate::ecs::*;
use crate::feeds::*;

pub struct InputFrame { pub id: EntityId, pub thrust: V3, pub fire: bool }

pub fn step(world: &crate::ecs::World, inputs: &[InputFrame]) {
  // thrust + damping
  { let mut bodies = world.bodies.write();
    for inp in inputs {
      if let Some(b) = bodies.get_mut(&inp.id) {
        let a = to_fix(8.0);
        let dv = inp.thrust.scale(mul(a, DT));
        b.vel = b.vel.add(dv);
        b.vel = b.vel.scale(to_fix(0.99));
      }
    }
  }
  // simple collisions
  { use crate::types::Kind;
    let snap: Vec<_> = world.bodies.write().values().copied().collect();
    let ship_r = to_fix(10.0); let ast_r = to_fix(14.0);
    for i in 0..snap.len(){ for j in (i+1)..snap.len(){
      let (a,b) = (snap[i], snap[j]);
      let ra = if matches!(a.kind,Kind::Asteroid) {ast_r} else {ship_r};
      let rb = if matches!(b.kind,Kind::Asteroid) {ast_r} else {ship_r};
      let dx = to_f64(sub(b.pos.x,a.pos.x)); let dy = to_f64(sub(b.pos.y,a.pos.y));
      let dist = (dx*dx+dy*dy).sqrt(); let rsum = to_f64(add(ra,rb));
      if dist>0.0 && dist<rsum {
        let nx=dx/dist; let ny=dy/dist; let push=(rsum-dist)*0.5;
        let px=to_fix(nx*push); let py=to_fix(ny*push);
        let mut map=world.bodies.write();
        if let Some(ma)=map.get_mut(&a.id){ ma.pos.x=sub(ma.pos.x,px); ma.pos.y=sub(ma.pos.y,py); }
        if let Some(mb)=map.get_mut(&b.id){ mb.pos.x=add(mb.pos.x,px); mb.pos.y=add(mb.pos.y,py); }
      }
    }}}
  // mining (1 kg/s when close)
  { let mining = world.mining.read().clone();
    for (ship_id,node_id) in mining {
      let (sp,np) = { let b=world.bodies.read();
        match (b.get(&ship_id), b.get(&node_id)) { (Some(s),Some(n)) => (s.pos,n.pos), _=>continue } };
      let dx=to_f64(sub(np.x,sp.x)); let dy=to_f64(sub(np.y,sp.y)); let dist=(dx*dx+dy*dy).sqrt();
      if dist>30.0 { continue; } // 30 m range
      let dt=to_f64(DT); let mut take = 1.0*dt;
      let mut nodes = world.resource_nodes.write();
      if let Some(node)=nodes.get_mut(&node_id){
        if node.remaining_kg<=0.0 { continue; }
        if take>node.remaining_kg { take=node.remaining_kg; }
        node.remaining_kg -= take;
        let mut invs = world.inventories.write();
        let inv = invs.entry(ship_id).or_default();
        for i in 0..FEED_DIM { inv.feeds[i] += take * node.feeds_frac[i]; }
      }
    }
  }
  // integrate
  { let mut bodies = world.bodies.write();
    for (_,b) in bodies.iter_mut(){ let dp=b.vel.scale(DT); b.pos=b.pos.add(dp); } }
}

pub fn craft_food(inv: &mut Inventory) -> bool {
  let (c,n,w) = (0.45, 0.08, 0.47);
  if inv.feeds[FE_CARBON]>=c && inv.feeds[FE_NITROGEN]>=n && inv.feeds[FE_WATER]>=w {
    inv.feeds[FE_CARBON]-=c; inv.feeds[FE_NITROGEN]-=n; inv.feeds[FE_WATER]-=w; inv.food_kg += 1.0; true
  } else { false }
}
