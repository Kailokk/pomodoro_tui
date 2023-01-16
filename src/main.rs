extern crate crossterm;
extern crate trata;

use std::{string, time::Duration};

use crossterm::event::{self, poll, Event, KeyCode, KeyEvent};
use crossterm::{cursor, terminal, ExecutableCommand, QueueableCommand};
use std::io::{stdout, Stdout, Write};
use std::{thread, time};
use trata::trata::{Config, TimerMode, TrataTimer};

fn main() {
    let config = Config {
        work_length_minutes: 20,
        short_break_length_minutes: 5,
        long_break_length_minutes: 30,
        has_long_break: true,
        timer_mode_will_rollover: false,
        work_sessions_before_long_break: 2,
    };

    let mut timer = TrataTimer::new(&config, draw_graphics, timer_end_alarm);
    timer.play_pause_timer();
    let mut stdout = stdout();
    stdout
        .queue(terminal::Clear(terminal::ClearType::All))
        .unwrap();
    stdout.flush().unwrap();
    terminal::enable_raw_mode().unwrap();
    draw_graphics(
        Duration::new((config.work_length_minutes as u64) * 60, 0),
        &TimerMode::Work,
        true,
    );
    loop {
        if poll(Duration::from_millis(333)).unwrap() {
            if let Ok(Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                ..
            })) = event::read()
            {
                match c {
                    'q' | 'Q' => break,
                    'p' | 'P' => timer.play_pause_timer(),
                    's' | 'S' => timer.end_section_early(),
                    _ => {}
                }
            }
        } else {
        }

        timer.pump_timer();
    }
}

fn draw_graphics(duration: Duration, mode: &TimerMode, is_running: bool) {
    let mut stdout = stdout();

    stdout.execute(cursor::Hide).unwrap();
    stdout.queue(cursor::MoveToColumn(0)).unwrap();
    stdout.queue(cursor::MoveToRow(0)).unwrap();

    let minutes = duration.as_secs() / 60;
    let seconds = duration.as_secs() % 60;

    let mode_string: String = if is_running {
        format!("Mode: {}", mode.get_string())
    } else {
        format!("Mode: {} (Paused)", mode.get_string())
    };

    stdout.write(format!("---------").as_bytes()).unwrap();
    stdout.queue(cursor::MoveToColumn(0)).unwrap();
    stdout.queue(cursor::MoveToRow(1)).unwrap();
    stdout
        .write(format!("| {:0>2}:{:0>2} | ", minutes, seconds).as_bytes())
        .unwrap();
    stdout.queue(cursor::MoveToColumn(0)).unwrap();
    stdout.queue(cursor::MoveToRow(2)).unwrap();
    stdout.write(format!("---------").as_bytes()).unwrap();
    stdout.queue(cursor::MoveToColumn(0)).unwrap();
    stdout.queue(cursor::MoveToRow(3)).unwrap();
    stdout
        .write(format!("                                                      ").as_bytes())
        .unwrap();
    stdout.queue(cursor::MoveToColumn(0)).unwrap();
    stdout.queue(cursor::MoveToRow(3)).unwrap();
    stdout.write(format!("{}", mode_string).as_bytes()).unwrap();
    stdout.queue(cursor::MoveToColumn(0)).unwrap();
    stdout.queue(cursor::MoveToRow(4)).unwrap();

    stdout.flush().unwrap();
}

fn timer_end_alarm(mode: &TimerMode) {
    print!("\x07");
}
