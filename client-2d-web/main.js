const cv = document.getElementById('cv');
const ctx = cv.getContext('2d');
const hudShip = document.getElementById('ship');
function resize(){ cv.width=window.innerWidth; cv.height=window.innerHeight; }
window.addEventListener('resize',resize); resize();

// Camera (very simple): center at origin, scale 1 px per meter initially
let cam = { x:0, y:0, scale: 0.2 }; // 0.2 px/m → zoomed out; scroll to zoom later
cv.addEventListener('wheel', (e)=>{ cam.scale *= (e.deltaY>0?0.9:1.1); cam.scale = Math.max(0.02, Math.min(5, cam.scale)); });

// Input state
const keys = new Set();
const block = new Set(['arrowup','arrowdown','arrowleft','arrowright',' ']);
window.addEventListener('keydown', e=>{
  const k = e.key.toLowerCase();
  const c = e.code;            // 'KeyW', 'ArrowUp', etc.
  keys.add(k); keys.add(c);
  if (block.has(k)) e.preventDefault();
});
window.addEventListener('keyup', e=>{
  const k = e.key.toLowerCase();
  const c = e.code;
  keys.delete(k); keys.delete(c);
  if (block.has(k)) e.preventDefault();
});

// Ship id assigned by server
let myId = 0;

// Entity state map
const ents = new Map(); // id -> {x,y,z,kind}

const ws = new WebSocket('ws://127.0.0.1:8080');
ws.addEventListener('open', ()=>console.log('ws open'));
ws.addEventListener('message', (ev)=>{
  try {
    const m = JSON.parse(ev.data);
    if (m.type === 'welcome') {
      myId = m.ship_id; hudShip.textContent = String(myId);
    } else if (m.type === 'snapshot') {
      for (const e of m.entities) { ents.set(e.id, e); }
      draw();
    }
  } catch (_e) {
    // ignore non-JSON (e.g., FlatBuffers) in this client
  }
});

// Send input at ~20 Hz
const SEND_MS = 16; // ~60 Hz
setInterval(()=>{
if (!myId) return;
if (ws.readyState !== WebSocket.OPEN) return;
let thrust = [0,0,0];
const up    = keys.has('arrowup')    || keys.has('KeyW') || keys.has('w');
const down  = keys.has('arrowdown')  || keys.has('KeyS') || keys.has('s');
const left  = keys.has('arrowleft')  || keys.has('KeyA') || keys.has('a');
const right = keys.has('arrowright') || keys.has('KeyD') || keys.has('d');
// Y inverted so pressing Up moves up on screen
if (up)    thrust[1] -= 1;
if (down)  thrust[1] += 1;
if (right) thrust[0] += 1;
if (left)  thrust[0] -= 1;
const mag = Math.hypot(thrust[0], thrust[1], thrust[2]) || 1;
thrust = thrust.map(v=>v/mag);
ws.send(JSON.stringify({ type:'input', id: myId, thrust }));
}, SEND_MS);



function draw(){
  const W = cv.width, H = cv.height;
  const ctx = cv.getContext('2d');
  ctx.clearRect(0,0,W,H);
  // grid
  ctx.globalAlpha = 0.3;
  ctx.strokeStyle = '#263238';
  ctx.beginPath();
  for (let x = -1000; x <= 1000; x+=100) {
    const sx1 = W/2 + (x-cam.x)*cam.scale; const sy1 = 0;
    const sx2 = W/2 + (x-cam.x)*cam.scale; const sy2 = H;
    ctx.moveTo(sx1, sy1); ctx.lineTo(sx2, sy2);
  }
  for (let y = -1000; y <= 1000; y+=100) {
    const sx1 = 0; const sy1 = H/2 + (y-cam.y)*cam.scale;
    const sx2 = W; const sy2 = H/2 + (y-cam.y)*cam.scale;
    ctx.moveTo(sx1, sy1); ctx.lineTo(sx2, sy2);
  }
  ctx.stroke(); ctx.globalAlpha = 1;

  for (const e of ents.values()){
    const x = e.x, y = e.y; // ignore z for 2D view
    const sx = W/2 + (x - cam.x) * cam.scale;
    const sy = H/2 + (y - cam.y) * cam.scale;
    const r = 6 + Math.max(0, 0.04 * (10 - Math.abs(e.z||0))); // hint of altitude

    ctx.fillStyle = (e.id === myId) ? '#80cbc4' : (e.kind==='Ship' ? '#90caf9' : '#ffcc80');
    ctx.beginPath(); ctx.arc(sx, sy, r, 0, Math.PI*2); ctx.fill();
  }
}
