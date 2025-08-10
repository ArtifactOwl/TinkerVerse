const cv = document.getElementById('cv');
const ctx = cv.getContext('2d');
const hudShip = document.getElementById('ship');
const invEl = document.getElementById('inv');
const btnMine = document.getElementById('btnMine');
const btnStop = document.getElementById('btnStop');
const btnFood = document.getElementById('btnFood');

function resize(){ cv.width=window.innerWidth; cv.height=window.innerHeight; }
window.addEventListener('resize',resize); resize();

let cam = { x:0, y:0, scale: 0.25 };
cv.addEventListener('wheel', (e)=>{ cam.scale *= (e.deltaY>0?0.9:1.1); cam.scale = Math.max(0.02, Math.min(5, cam.scale)); });

const keys = new Set();
const block = new Set(['arrowup','arrowdown','arrowleft','arrowright',' ']);
window.addEventListener('keydown', e=>{ const k=e.key.toLowerCase(), c=e.code; keys.add(k); keys.add(c); if(block.has(k)) e.preventDefault(); });
window.addEventListener('keyup', e=>{ const k=e.key.toLowerCase(), c=e.code; keys.delete(k); keys.delete(c); if(block.has(k)) e.preventDefault(); });

let myId = 0;
const ents = new Map();

const ws = new WebSocket('ws://127.0.0.1:8080');
ws.addEventListener('message', (ev)=>{
  try {
    const m = JSON.parse(ev.data);
    if (m.type === 'welcome') { myId = m.ship_id; hudShip.textContent = String(myId); }
    else if (m.type === 'snapshot') { for (const e of m.entities) ents.set(e.id, e); draw(); }
    else if (m.type === 'inv') { invEl.textContent = `C: ${m.feeds.C.toFixed(2)}  H2O: ${m.feeds.H2O.toFixed(2)}  N: ${m.feeds.N.toFixed(2)}  Food: ${m.food.toFixed(2)}`; }
  } catch {}
});

function nearestAsteroidId(){
  let best=null, bestd=1e9; const me=ents.get(myId); if(!me) return null;
  for (const e of ents.values()) { if (e.kind!=='Asteroid') continue;
    const d=Math.hypot(e.x-me.x,e.y-me.y); if(d<bestd){ bestd=d; best=e.id; } }
  return best;
}
btnMine.onclick = ()=>{ const id=nearestAsteroidId(); if(id) ws.send(JSON.stringify({ type:'mine', node:id })); };
btnStop.onclick = ()=> ws.send(JSON.stringify({ type:'mine', stop:true }));
btnFood.onclick = ()=> ws.send(JSON.stringify({ type:'craft', kind:'food', kg:1 }));

setInterval(()=>{
  if (!myId || ws.readyState !== WebSocket.OPEN) return;
  let thrust = [0,0,0];
  const up = keys.has('arrowup')||keys.has('KeyW')||keys.has('w');
  const down = keys.has('arrowdown')||keys.has('KeyS')||keys.has('s');
  const left = keys.has('arrowleft')||keys.has('KeyA')||keys.has('a');
  const right = keys.has('arrowright')||keys.has('KeyD')||keys.has('d');
  if (up) thrust[1] -= 1; if (down) thrust[1] += 1; if (right) thrust[0] += 1; if (left) thrust[0] -= 1;
  const mag = Math.hypot(thrust[0], thrust[1], thrust[2]) || 1;
  thrust = thrust.map(v=>v/mag);
  ws.send(JSON.stringify({ type:'input', id: myId, thrust }));
}, 16);

let follow = true;
function draw(){
  const W=cv.width, H=cv.height;
  const me=ents.get(myId);
  if (follow && me) { cam.x += (me.x - cam.x) * 0.15; cam.y += (me.y - cam.y) * 0.15; }
  ctx.clearRect(0,0,W,H);
  ctx.globalAlpha=0.3; ctx.strokeStyle='#263238'; ctx.beginPath();
  for(let x=-1000; x<=1000; x+=100){ const sx=W/2+(x-cam.x)*cam.scale; ctx.moveTo(sx,0); ctx.lineTo(sx,H); }
  for(let y=-1000; y<=1000; y+=100){ const sy=H/2-(y-cam.y)*cam.scale; ctx.moveTo(0,sy); ctx.lineTo(W,sy); }
  ctx.stroke(); ctx.globalAlpha=1;
  for(const e of ents.values()){
    const sx=W/2+(e.x-cam.x)*cam.scale, sy=H/2-(e.y-cam.y)*cam.scale;
    let r=6, color='#90caf9'; if(e.id===myId) color='#80cbc4'; if(e.kind==='Asteroid'){ color='#ffab91'; r=12; }
    ctx.fillStyle=color; ctx.beginPath(); ctx.arc(sx,sy,r,0,Math.PI*2); ctx.fill();
    if(e.kind==='Asteroid'){ ctx.fillStyle='#cfd8dc'; ctx.font='12px system-ui'; ctx.fillText(`Asteroid ${e.id}`, sx+14, sy-8); }
  }
}
