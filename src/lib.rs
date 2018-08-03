#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(dead_code)]

extern crate rand;

use std::collections::HashMap;
use std::ops::Add;
use rand::ThreadRng;
use std::collections::hash_map::*;

pub const TILE_GRID_WIDTH: usize = 66;
pub const TILE_GRID_HEIGHT: usize = 50;

#[derive(Debug, Clone)]
pub struct GameWorld {
    pub player_location: Location,
    pub creatures: HashMap<Location, Creature>,
    pub terrain: HashMap<Location, Terrain>,
    pub rng: ThreadRng,
}

impl GameWorld {
    pub fn move_player(&mut self, delta: Location) {
        let player_move_target = self.player_location + delta;
        if self.creatures.contains_key(&player_move_target) {
            // LATER: combat will go here
        } else {
            match *self
                .terrain
                .entry(player_move_target)
                .or_insert(Terrain::Floor)
            {
                Terrain::Wall => return,
                Terrain::Floor => {
                    let player = self
                        .creatures
                        .remove(&self.player_location)
                        .expect("The player wasn't where they should be!");
                    let old_creature = self.creatures.insert(player_move_target, player);
                    debug_assert!(old_creature.is_none());
                    self.player_location = player_move_target;
                }
            }
        }
        // LATER: other creatures act now that the player is resolved.
    }

    pub fn new(seed: u64) -> Self {
        let mut out = Self {
            player_location: Location { x: 5, y: 5 },
            creatures: HashMap::new(),
            terrain: HashMap::new(),
            rng: rand::thread_rng(),
        };
        out.creatures.insert(Location { x: 5, y: 5 }, Creature {});
        
        let caves = make_cellular_caves(TILE_GRID_WIDTH, TILE_GRID_HEIGHT, &mut out.rng);
        for (x, y, tile) in caves.iter() {
            out.terrain
            .insert(Location { x: x as i32, y: y as i32 }, if *tile { Terrain::Wall } else { Terrain::Floor });
        }
        out
    }
}

fn make_cellular_caves(width: usize, height: usize, rng: &mut ThreadRng) -> VecImage<bool> {
    let mut buffer_a: VecImage<bool> = VecImage::new(width, height);
    let mut buffer_b: VecImage<bool> = VecImage::new(width, height);
    // fill the initial buffer, all cells 45% likely.
    for (_x,_y,mut_ref) in buffer_a.iter_mut(){
        if rng.gen_weighted_bool(n) d100.roll_with(gen) <= 45 {
            *mut_ref = true;
        }
    }
    unimplemented!()
}

#[derive(Debug, Clone, Copy)]
pub struct Creature {}

#[derive(Debug, Clone, Copy)]
pub enum Terrain {
    Wall,
    Floor,
}

impl Default for Terrain {
    fn default() -> Self {
        Terrain::Wall
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, Hash)]
pub struct Location {
    pub x: i32,
    pub y: i32,
}

impl Location {
    fn as_usize(self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }
}

impl Add for Location {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Location {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
