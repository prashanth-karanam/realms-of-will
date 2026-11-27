#![no_std]

//! Realms of Will — Zero-Allocation SoA ECS + Spatial Hash + Deterministic Terrain Core
//! Compiled to `sim_core.wasm` (and ready for `wasm-pack build --target web`).

pub const MAX_ITEMS: usize = 5000;
pub const MAX_NPCS: usize = 2500;
pub const GRID_SIZE: usize = 64;
pub const GRID_CELLS: usize = GRID_SIZE * GRID_SIZE; // 4096

#[repr(C)]
pub struct SimMemory {
    // Dropped Items SoA (8 arrays x 5000 = 160 KB)
    pub item_x: [f32; MAX_ITEMS],
    pub item_y: [f32; MAX_ITEMS],
    pub item_z: [f32; MAX_ITEMS],
    pub item_vx: [f32; MAX_ITEMS],
    pub item_vy: [f32; MAX_ITEMS],
    pub item_vz: [f32; MAX_ITEMS],
    pub item_rot: [f32; MAX_ITEMS],
    pub item_type: [i32; MAX_ITEMS], // 0=Sword, 1=Gold, 2=Copper, 3=FireSword

    // NPCs SoA (10 arrays x 2500)
    pub npc_x: [f32; MAX_NPCS],
    pub npc_y: [f32; MAX_NPCS],
    pub npc_z: [f32; MAX_NPCS],
    pub npc_vx: [f32; MAX_NPCS],
    pub npc_vz: [f32; MAX_NPCS],
    pub npc_yaw: [f32; MAX_NPCS],
    pub npc_tx: [f32; MAX_NPCS],
    pub npc_tz: [f32; MAX_NPCS],
    pub npc_speed: [f32; MAX_NPCS],
    pub npc_kind: [i32; MAX_NPCS],

    // O(1) Spatial Hash Linked List
    pub grid_head: [i32; GRID_CELLS],
    pub grid_next: [i32; MAX_NPCS],
}

static mut SIM: SimMemory = SimMemory {
    item_x: [0.0; MAX_ITEMS],
    item_y: [0.0; MAX_ITEMS],
    item_z: [0.0; MAX_ITEMS],
    item_vx: [0.0; MAX_ITEMS],
    item_vy: [0.0; MAX_ITEMS],
    item_vz: [0.0; MAX_ITEMS],
    item_rot: [0.0; MAX_ITEMS],
    item_type: [0; MAX_ITEMS],
    npc_x: [0.0; MAX_NPCS],
    npc_y: [0.0; MAX_NPCS],
    npc_z: [0.0; MAX_NPCS],
    npc_vx: [0.0; MAX_NPCS],
    npc_vz: [0.0; MAX_NPCS],
    npc_yaw: [0.0; MAX_NPCS],
    npc_tx: [0.0; MAX_NPCS],
    npc_tz: [0.0; MAX_NPCS],
    npc_speed: [0.0; MAX_NPCS],
    npc_kind: [0; MAX_NPCS],
    grid_head: [-1; GRID_CELLS],
    grid_next: [-1; MAX_NPCS],
};

#[inline(always)]
fn hash2d(ix: i32, iz: i32) -> f32 {
    let mut h = (ix.wrapping_mul(374761393)).wrapping_add(iz.wrapping_mul(668265263));
    h = (h ^ (h >> 13)).wrapping_mul(1274126177);
    h ^= h >> 16;
    ((h & 0x7fff) as f32) / 32767.0
}

#[inline(always)]
fn floor_f32(v: f32) -> i32 {
    let i = v as i32;
    if (i as f32) > v { i - 1 } else { i }
}

#[inline(always)]
fn value_noise(x: f32, z: f32) -> f32 {
    let ix = floor_f32(x);
    let iz = floor_f32(z);
    let fx = x - (ix as f32);
    let fz = z - (iz as f32);
    let ux = fx * fx * (3.0 - 2.0 * fx);
    let uz = fz * fz * (3.0 - 2.0 * fz);
    let a = hash2d(ix, iz);
    let b = hash2d(ix + 1, iz);
    let c = hash2d(ix, iz + 1);
    let d = hash2d(ix + 1, iz + 1);
    let ab = a + (b - a) * ux;
    let cd = c + (d - c) * ux;
    ab + (cd - ab) * uz
}

#[no_mangle]
pub extern "C" fn terrain_height(x: f32, z: f32) -> f32 {
    let n1 = value_noise(x * 0.022, z * 0.022);
    let n2 = value_noise(x * 0.055 + 17.3, z * 0.055 - 9.1);
    let ridge = 1.0 - libm_abs(n1 * 2.0 - 1.0);
    let mut h = ridge * ridge * 6.5 + n2 * 1.8 - 1.4;
    if h < -0.5 { h = -0.5; }
    h
}

#[inline(always)]
fn libm_abs(v: f32) -> f32 {
    if v < 0.0 { -v } else { v }
}

#[no_mangle]
pub unsafe extern "C" fn step_items(count: i32, dt: f32, gravity: f32) {
    let n = (count as usize).min(MAX_ITEMS);
    for i in 0..n {
        let mut x = SIM.item_x[i] + SIM.item_vx[i] * dt;
        let mut y = SIM.item_y[i] + SIM.item_vy[i] * dt;
        let mut z = SIM.item_z[i] + SIM.item_vz[i] * dt;
        let mut vy = SIM.item_vy[i] - gravity * dt;

        let floor_y = terrain_height(x, z);
        if y <= floor_y {
            y = floor_y;
            if vy < -0.8 {
                vy = -vy * 0.42;
            } else {
                vy = 0.0;
            }
            SIM.item_vx[i] *= 0.86;
            SIM.item_vz[i] *= 0.86;
        } else {
            SIM.item_rot[i] += 5.5 * dt;
        }

        SIM.item_x[i] = x;
        SIM.item_y[i] = y;
        SIM.item_z[i] = z;
        SIM.item_vy[i] = vy;
    }
}

#[no_mangle]
pub unsafe extern "C" fn step_npcs(count: i32, dt: f32, px: f32, pz: f32) -> i32 {
    let n = (count as usize).min(MAX_NPCS);
    for c in 0..GRID_CELLS {
        SIM.grid_head[c] = -1;
    }
    // 1. Spatial Hash Insert O(N)
    for i in 0..n {
        let cx = (floor_f32(SIM.npc_x[i] * 0.5) & 63) as usize;
        let cz = (floor_f32(SIM.npc_z[i] * 0.5) & 63) as usize;
        let cell = cx + (cz << 6);
        SIM.grid_next[i] = SIM.grid_head[cell];
        SIM.grid_head[cell] = i as i32;
    }

    let mut checks: i32 = 0;
    // 2. O(1) Local Crowd Separation + Target Steering
    for i in 0..n {
        let x = SIM.npc_x[i];
        let z = SIM.npc_z[i];
        let mut sep_x = 0.0_f32;
        let mut sep_z = 0.0_f32;

        let gx = floor_f32(x * 0.5);
        let gz = floor_f32(z * 0.5);

        for oz in -1..=1 {
            for ox in -1..=1 {
                let cx = ((gx + ox) & 63) as usize;
                let cz = ((gz + oz) & 63) as usize;
                let mut cur = SIM.grid_head[cx + (cz << 6)];
                let mut limit = 0;
                while cur >= 0 && limit < 12 {
                    let j = cur as usize;
                    if j != i {
                        checks += 1;
                        let dx = x - SIM.npc_x[j];
                        let dz = z - SIM.npc_z[j];
                        let d2 = dx * dx + dz * dz;
                        if d2 > 0.0001 && d2 < 1.35 {
                            let inv = 1.35 - d2;
                            sep_x += dx * inv * 3.2;
                            sep_z += dz * inv * 3.2;
                        }
                    }
                    cur = SIM.grid_next[j];
                    limit += 1;
                }
            }
        }

        // Goal seek + wrap far NPCs seamlessly around player's active horizon
        let mut tx = SIM.npc_tx[i];
        let mut tz = SIM.npc_tz[i];
        let pdx = x - px;
        let pdz = z - pz;
        if pdx * pdx + pdz * pdz > 4900.0 {
            SIM.npc_x[i] = px - pdx * 0.85;
            SIM.npc_z[i] = pz - pdz * 0.85;
            tx = px;
            tz = pz;
            SIM.npc_tx[i] = tx;
            SIM.npc_tz[i] = tz;
            continue;
        }

        let gdx = tx - x;
        let gdz = tz - z;
        let g2 = gdx * gdx + gdz * gdz;
        if g2 < 2.5 {
            // Deterministic pseudo-random retarget
            let h1 = hash2d(i as i32 + floor_f32(x), floor_f32(z)) * 2.0 - 1.0;
            let h2 = hash2d(floor_f32(z), i as i32 + floor_f32(x)) * 2.0 - 1.0;
            SIM.npc_tx[i] = px + h1 * 48.0;
            SIM.npc_tz[i] = pz + h2 * 48.0;
        }

        let spd = SIM.npc_speed[i];
        let inv_g = if g2 > 0.01 { spd / sqrt_approx(g2) } else { 0.0 };
        let vx = gdx * inv_g + sep_x;
        let vz = gdz * inv_g + sep_z;

        let nx = x + vx * dt;
        let nz = z + vz * dt;
        SIM.npc_x[i] = nx;
        SIM.npc_z[i] = nz;
        SIM.npc_y[i] = terrain_height(nx, nz);
        SIM.npc_vx[i] = vx;
        SIM.npc_vz[i] = vz;
    }
    checks
}

#[inline(always)]
fn sqrt_approx(v: f32) -> f32 {
    if v <= 0.0 { return 0.0; }
    let mut x = v * 0.5 + 0.5;
    x = 0.5 * (x + v / x);
    x = 0.5 * (x + v / x);
    x
}
