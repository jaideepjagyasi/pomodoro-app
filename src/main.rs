mod audio;
mod timer;
mod ui;

use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use std::time::Duration;

use timer::Timer;
use ui::App;

fn main() -> io::Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let mut timer = Timer::new();

    let res = run_app(&mut terminal, &mut app, &mut timer);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        eprintln!("Error: {:?}", err);
    }

    Ok(())
}

fn run_app(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    app: &mut App,
    timer: &mut Timer,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app, timer))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            app.quit();
                            break;
                        }
                        KeyCode::Char(' ') | KeyCode::Enter => {
                            if timer.is_idle() {
                                timer.start_pomodoro(app.pomodoro_duration);
                                app.set_state(timer::TimerState::Pomodoro);
                                audio::play_pomodoro_start();
                            } else if timer.is_running() {
                                timer.pause();
                                app.set_paused(true);
                            } else if timer.is_paused() {
                                timer.resume();
                                app.set_paused(false);
                            }
                        }
                        KeyCode::Char('s') => {
                            timer.stop();
                            app.reset();
                        }
                        KeyCode::Char('k') => {
                            // Skip to next phase
                            timer.stop();
                            match app.state {
                                timer::TimerState::Pomodoro => {
                                    let break_dur = if app.pomodoros_completed % 4 == 0 {
                                        app.long_break_duration
                                    } else {
                                        app.short_break_duration
                                    };
                                    timer.start_break(break_dur);
                                    if app.pomodoros_completed % 4 == 0 {
                                        app.set_state(timer::TimerState::LongBreak);
                                    } else {
                                        app.set_state(timer::TimerState::ShortBreak);
                                    }
                                    audio::play_break_start();
                                }
                                _ => {
                                    timer.start_pomodoro(app.pomodoro_duration);
                                    app.set_state(timer::TimerState::Pomodoro);
                                    audio::play_pomodoro_start();
                                }
                            }
                        }
                        KeyCode::Right | KeyCode::Char('l') => {
                            app.cycle_tab();
                        }
                        KeyCode::Left | KeyCode::Char('h') => {
                            app.cycle_tab_reverse();
                        }
                        // Settings adjustments
                        KeyCode::Char('+') | KeyCode::Char('=') => {
                            app.adjust_setting(1);
                        }
                        KeyCode::Char('-') | KeyCode::Char('_') => {
                            app.adjust_setting(-1);
                        }
                        _ => {}
                    }
                }
            }
        }

        // Check timer state
        if timer.check_finished() {
            match app.state {
                timer::TimerState::Pomodoro => {
                    app.pomodoros_completed += 1;
                    audio::play_pomodoro_end();
                    let break_dur = if app.pomodoros_completed % 4 == 0 {
                        app.long_break_duration
                    } else {
                        app.short_break_duration
                    };
                    let is_long = app.pomodoros_completed % 4 == 0;
                    timer.start_break(break_dur);
                    if is_long {
                        app.set_state(timer::TimerState::LongBreak);
                    } else {
                        app.set_state(timer::TimerState::ShortBreak);
                    }
                    audio::play_break_start();
                }
                timer::TimerState::ShortBreak | timer::TimerState::LongBreak => {
                    audio::play_break_end();
                    timer.start_pomodoro(app.pomodoro_duration);
                    app.set_state(timer::TimerState::Pomodoro);
                    audio::play_pomodoro_start();
                }
                _ => {}
            }
        }

        // Update remaining time
        app.update_remaining(timer.remaining());
    }

    Ok(())
}
