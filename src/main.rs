use std::{
    error::Error, io, sync::mpsc::channel, thread, time::{Duration, Instant}
};

use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event, KeyCode},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use invaders::{
    frame::{self, new_frame, Drawable}, invaders::InvadersManager, player::Player, render
};
use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
    // Audio
    let mut audio = Audio::new();
    audio.add("explode", "audios/explode.wav");
    audio.add("lose", "audios/lose.wav");
    audio.add("win", "audios/win.wav");
    audio.add("pew", "audios/pew.wav");
    audio.add("startup", "audios/startup.wav");
    audio.add("move", "audios/move.wav");

    audio.play("startup");

    // Terminal
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // Render Loop
    let (render_tx, render_rx) = channel();
    let _render_handle = thread::spawn(move || {
        let mut last_frame = frame::new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &last_frame, &last_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render::render(&mut stdout, &last_frame, &curr_frame, false);
            last_frame = curr_frame;
        }
    });
    // Game Loop
    let mut invaders = InvadersManager::new();
    let mut player = Player::new();
    let mut instant = Instant::now();

    'gameloop: loop {
        // Per-frame Init
        let delta = instant.elapsed();
        instant = Instant::now();
        let mut curr_frame = new_frame();
        // Input
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => {
                        audio.play("lose");
                        break 'gameloop;
                    }
                    KeyCode::Char(' ') | KeyCode::Enter => {
                        if player.shoot() {
                            audio.play("pew");
                        }
                    }
                    KeyCode::Right => player.move_right(),
                    KeyCode::Left => player.move_left(),
                    _ => {}
                }
            }
        }
        // Update 
        player.update(delta);
        if invaders.update(delta) {
            audio.play("move")
        }
        if player.detect_hits(&mut invaders){
            audio.play("explode");
        }

        // Draw & Render
        let drawables: Vec<&dyn Drawable> = vec![&player, &invaders];
        for drawable in drawables {
            drawable.draw(&mut curr_frame)
        }
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));


        // Win or Loose 
        if invaders.all_killed(){
            audio.play("win");
            break 'gameloop;
        }
        if invaders.reached_bottom(){
            audio.play("lose");
            break 'gameloop;
        }
    }

    // Cleanup
    audio.wait();
    stdout.execute(LeaveAlternateScreen)?;
    stdout.execute(Show)?;
    Ok(())
}
