#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(dead_code)]

extern crate dwarf_term;
extern crate rand;

use dwarf_term::*;
use rand::{SeedableRng, XorShiftRng};

mod lib;

use lib::*;
use std::collections::HashMap;

const WALL_TILE: u8 = b'#';

fn main() {
    let mut term = unsafe {
        DwarfTerm::new(TILE_GRID_WIDTH, TILE_GRID_HEIGHT, "Dungeons of Rust")
            .expect("Failed to open terminal.")
    };
    term.set_all_foregrounds(rgb32!(128, 255, 20));
    term.set_all_backgrounds(0);

    // Main loop
    let mut game = GameWorld::new();
    game.creatures.insert(Location { x: 5, y: 5 }, Creature {});
    game.terrain
        .insert(Location { x: 10, y: 10 }, Terrain::Wall);

    let mut running = true;
    let mut pending_keys = vec![];
    'game: loop {
        term.poll_events(|event| match event {
            Event::WindowEvent {
                event: win_event, ..
            } => match win_event {
                WindowEvent::CloseRequested
                | WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                } => {
                    running = false;
                }
                WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(key),
                            ..
                        },
                    ..
                } => {
                    pending_keys.push(key);
                }
                _ => {}
            },
            _ => {}
        });

        if !running {
            break 'game;
        }

        for key in pending_keys.drain(..) {
            match key {
                VirtualKeyCode::Up => game.move_player(Location { x: 0, y: 1 }),
                VirtualKeyCode::Down => game.move_player(Location { x: 0, y: -1 }),
                VirtualKeyCode::Left => game.move_player(Location { x: -1, y: 0 }),
                VirtualKeyCode::Right => game.move_player(Location { x: 1, y: 0 }),
                _ => {}
            }
        }

        {
            let (mut fgs, mut bgs, mut ids) = term.layer_slices_mut();
            for (scr_x, scr_y, id_mut) in ids.iter_mut() {
                let loc_for_this_screen_position = Location {
                    x: scr_x as i32,
                    y: scr_y as i32,
                };
                match game.creatures.get(&loc_for_this_screen_position) {
                    Some(ref creature) => {
                        *id_mut = b'@';
                        fgs[(scr_x, scr_y)] = rgb32!(255, 255, 255);
                    }
                    None => match game.terrain.get(&loc_for_this_screen_position) {
                        Some(Terrain::Wall) => {
                            *id_mut = WALL_TILE;
                            fgs[(scr_x, scr_y)] = rgb32!(155, 75, 0);
                        }
                        Some(Terrain::Floor) => {
                            *id_mut = b'.';
                            fgs[(scr_x, scr_y)] = rgb32!(128, 128, 128);
                        }
                        None => {
                            *id_mut = b' ';
                        }
                    },
                }
            }
        }

        unsafe {
            term.clear_draw_swap()
                .map_err(|err_vec| {
                    for e in err_vec {
                        eprintln!("clear_draw_swap error: {:?}", e);
                    }
                })
                .ok();
        }
    }
}
