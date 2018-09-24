extern crate chrono;
extern crate clap;
#[macro_use]
extern crate failure;
extern crate fern;
#[macro_use]
extern crate log;
extern crate rogue_gym_core;
extern crate termion;
extern crate tuple_map;

pub mod error;
#[macro_use]
pub mod screen;
use error::*;
use rogue_gym_core::dungeon::{Coord, Positioned};
use rogue_gym_core::ui::{MordalKind, UiState};
use rogue_gym_core::{GameConfig, GameMsg, Reaction, RunTime};
use screen::Screen;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use termion::input::TermRead;

pub fn play_game(config: GameConfig, is_default: bool) -> GameResult<()> {
    let (w, h) = (config.width, config.height);
    let mut screen = Screen::from_raw(w, h)?;
    screen.welcome()?;
    if is_default {
        screen.default_config()?;
    }
    let mut runtime = config.build()?;
    thread::sleep(Duration::from_secs(1));
    draw_dungeon(&mut screen, &mut runtime)?;
    screen.status(runtime.player_status())?;
    let stdin = io::stdin();
    // let's receive keyboard inputs(out main loop)
    let mut player_cursor = None;
    for keys in stdin.keys() {
        screen.clear_notification()?;
        if let Some(cd) = player_cursor {
            screen.cursor(cd);
        }
        let key = keys.into_chained(|| "in play_game")?;
        let res = runtime.react_to_key(key.into());
        let res = match res {
            Ok(r) => r,
            Err(e) => {
                // STUB
                notify!(screen, "{}", e)?;
                continue;
            }
        };

        for reaction in res {
            let result = process_reaction(&mut screen, &mut runtime, reaction)
                .chain_err(|| "in play_game")?;
            if let Some(transition) = result {
                match transition {
                    Transition::Exit => return Ok(()),
                    Transition::PlayerCursor(cd) => player_cursor = Some(cd),
                }
            }
        }
    }

    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Transition {
    PlayerCursor(Coord),
    Exit,
}

pub fn process_reaction<W: Write>(
    screen: &mut Screen<W>,
    runtime: &mut RunTime,
    reaction: Reaction,
) -> GameResult<Option<Transition>> {
    match reaction {
        Reaction::Notify(msg) => {
            match msg {
                // GameMsg::CantMove(d) => notify!(screen, "your {} way is blocked", d),
                GameMsg::CantMove(_) => Ok(()),
                // TODO: Display for ItemKind
                GameMsg::CantGetItem(kind) => notify!(screen, "You walk onto {:?}", kind),
                GameMsg::NoDownStair => notify!(screen, "Hmm... there seems to be no downstair"),
                GameMsg::GotItem { kind, num } => {
                    notify!(screen, "Now you have {} {:?}", num, kind)
                }
                GameMsg::SecretDoor => notify!(screen, "you found a secret door"),
                GameMsg::Quit => {
                    notify!(screen, "Thank you for playing!")?;
                    return Ok(Some(Transition::Exit));
                }
            }.chain_err(|| "in devui::process_reaction")?;
            Ok(None)
        }
        Reaction::Redraw => {
            let cd = draw_dungeon(screen, runtime)
                .chain_err(|| "in process_action attempt to draw dungeon")?;
            Ok(cd.map(|cd| Transition::PlayerCursor(cd)))
        }
        Reaction::StatusUpdated => {
            let status = runtime.player_status();
            screen.status(status)?;
            Ok(None)
        }
        Reaction::UiTransition(ui_state) => {
            if let UiState::Mordal(kind) = ui_state {
                match kind {
                    MordalKind::Quit => notify!(screen, "You really quit game?(y/n)").map(|_| None),
                }
            } else {
                Ok(None)
            }
        }
    }
}

pub fn draw_dungeon<W: Write>(
    screen: &mut Screen<W>,
    runtime: &mut RunTime,
) -> GameResult<Option<Coord>> {
    // screen.clear_dungeon()?;
    let mut player_pos = None;
    runtime.draw_screen(|Positioned(cd, tile)| {
        if tile.to_byte() == b'@' {
            debug!("player: {:?}", cd);
            player_pos = Some(cd);
        }
        screen.draw_tile(cd, tile)
    })?;
    if let Some(cd) = player_pos {
        screen.cursor(cd)?;
    }
    screen.flush()?;
    Ok(player_pos)
}
