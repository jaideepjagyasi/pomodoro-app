use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Line, Span},
    widgets::{
        Block, BorderType, Borders, Gauge, Padding, Paragraph, Tabs,
    },
    Frame,
};

use crate::timer::{Timer, TimerState};

const COLOR_BG: Color = Color::Rgb(13, 17, 23);
const COLOR_SURFACE: Color = Color::Rgb(22, 27, 34);
const COLOR_BORDER: Color = Color::Rgb(48, 54, 61);
const COLOR_ACCENT: Color = Color::Rgb(233, 69, 96); // Pomodoro red

const COLOR_TEXT: Color = Color::Rgb(230, 237, 243);
const COLOR_TEXT_DIM: Color = Color::Rgb(139, 148, 158);
const COLOR_BLUE: Color = Color::Rgb(56, 139, 253);
const COLOR_YELLOW: Color = Color::Rgb(210, 153, 34);
const COLOR_BREAK: Color = Color::Rgb(58, 210, 140); // Mint green for breaks

pub struct App {
    pub state: TimerState,
    pub pomodoro_duration: u64,
    pub short_break_duration: u64,
    pub long_break_duration: u64,
    pub pomodoros_completed: u64,
    pub is_paused: bool,
    pub remaining: u64, // seconds
    pub selected_tab: usize,
    pub quitting: bool,
}

impl App {
    pub fn new() -> Self {
        App {
            state: TimerState::Idle,
            pomodoro_duration: 25,
            short_break_duration: 5,
            long_break_duration: 15,
            pomodoros_completed: 0,
            is_paused: false,
            remaining: 0,
            selected_tab: 0,
            quitting: false,
        }
    }

    pub fn set_state(&mut self, state: TimerState) {
        self.state = state;
    }

    pub fn set_paused(&mut self, paused: bool) {
        self.is_paused = paused;
    }

    pub fn update_remaining(&mut self, remaining: std::time::Duration) {
        self.remaining = remaining.as_secs();
    }

    pub fn reset(&mut self) {
        self.state = TimerState::Idle;
        self.is_paused = false;
        self.remaining = 0;
    }

    pub fn cycle_tab(&mut self) {
        self.selected_tab = (self.selected_tab + 1) % 3;
    }

    pub fn cycle_tab_reverse(&mut self) {
        self.selected_tab = if self.selected_tab == 0 { 2 } else { self.selected_tab - 1 };
    }

    pub fn adjust_setting(&mut self, delta: i64) {
        match self.selected_tab {
            0 => {
                // Pomodoro duration
                let new = self.pomodoro_duration as i64 + delta;
                if new >= 1 && new <= 90 {
                    self.pomodoro_duration = new as u64;
                }
            }
            1 => {
                // Short break
                let new = self.short_break_duration as i64 + delta;
                if new >= 1 && new <= 30 {
                    self.short_break_duration = new as u64;
                }
            }
            2 => {
                // Long break
                let new = self.long_break_duration as i64 + delta;
                if new >= 1 && new <= 60 {
                    self.long_break_duration = new as u64;
                }
            }
            _ => {}
        }
    }

    pub fn quit(&mut self) {
        self.quitting = true;
    }
}

pub fn draw(f: &mut Frame, app: &App, timer: &Timer) {
    let size = f.area();

    // Main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Title bar
            Constraint::Min(8),     // Main content
            Constraint::Length(3),  // Status bar
        ])
        .split(size);

    draw_title_bar(f, chunks[0]);
    draw_main_content(f, chunks[1], app, timer);
    draw_status_bar(f, chunks[2], app);
}

fn draw_title_bar(f: &mut Frame, area: Rect) {
    let title = Paragraph::new(Line::from(vec![
        Span::styled(" 🍅 ", Style::default().fg(COLOR_ACCENT).add_modifier(Modifier::BOLD)),
        Span::styled(
            "POMODORO",
            Style::default()
                .fg(COLOR_TEXT)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" v1.0", Style::default().fg(COLOR_TEXT_DIM)),
    ]))
    .block(
        Block::default()
            .borders(Borders::BOTTOM)
            .border_style(Style::default().fg(COLOR_BORDER))
            .border_type(BorderType::Plain)
            .style(Style::default().bg(COLOR_BG)),
    );

    f.render_widget(title, area);
}

fn draw_main_content(f: &mut Frame, area: Rect, app: &App, timer: &Timer) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(7),  // Timer display
            Constraint::Length(3),  // Progress bar
            Constraint::Min(4),     // Info / settings
        ])
        .split(area);

    draw_timer_display(f, chunks[0], app, timer);
    draw_progress_bar(f, chunks[1], app, timer);
    draw_bottom_section(f, chunks[2], app);
}

fn draw_timer_display(f: &mut Frame, area: Rect, app: &App, _timer: &Timer) {
    let minutes = app.remaining / 60;
    let seconds = app.remaining % 60;
    let time_str = format!("{:02}:{:02}", minutes, seconds);

    let (state_label, state_color, emoji) = match app.state {
        TimerState::Idle => ( "READY", COLOR_TEXT_DIM, "⏸" ),
        TimerState::Pomodoro if app.is_paused => ( "PAUSED", COLOR_YELLOW, "⏸" ),
        TimerState::Pomodoro => ( "FOCUS", COLOR_ACCENT, "🔥" ),
        TimerState::ShortBreak | TimerState::LongBreak if app.is_paused => {
            ( "PAUSED", COLOR_YELLOW, "⏸" )
        }
        TimerState::ShortBreak => ( "SHORT BREAK", COLOR_BREAK, "☕" ),
        TimerState::LongBreak => ( "LONG BREAK", COLOR_BLUE, "🌿" ),
    };

    let timer_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(state_color))
        .border_type(BorderType::Rounded)
        .padding(Padding::uniform(1))
        .style(Style::default().bg(COLOR_SURFACE));

    let inner = timer_block.inner(area);
    f.render_widget(timer_block, area);

    // Split inner area for state label and time
    let inner_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1),
            Constraint::Min(3),
        ])
        .split(inner);

    // State label
    let state_line = Paragraph::new(Line::from(vec![
        Span::styled(format!(" {} ", emoji), Style::default().fg(state_color)),
        Span::styled(
            state_label,
            Style::default()
                .fg(state_color)
                .add_modifier(Modifier::BOLD),
        ),
    ]))
    .alignment(Alignment::Center);
    f.render_widget(state_line, inner_chunks[0]);

    // Big time display using block characters
    let time_display = Paragraph::new(Line::from(Span::styled(
        time_str,
        Style::default()
            .fg(state_color)
            .add_modifier(Modifier::BOLD),
    )))
    .alignment(Alignment::Center);
    f.render_widget(time_display, inner_chunks[1]);
}

fn draw_progress_bar(f: &mut Frame, area: Rect, app: &App, timer: &Timer) {
    let progress = timer.progress();
    let (label, color) = match app.state {
        TimerState::Idle => (" 0% ".into(), COLOR_TEXT_DIM),
        TimerState::Pomodoro => {
            let pct = (progress * 100.0) as u16;
            (format!(" {}% ", pct), COLOR_ACCENT)
        }
        TimerState::ShortBreak => {
            let pct = (progress * 100.0) as u16;
            (format!(" {}% ", pct), COLOR_BREAK)
        }
        TimerState::LongBreak => {
            let pct = (progress * 100.0) as u16;
            (format!(" {}% ", pct), COLOR_BLUE)
        }
    };

    let gauge = Gauge::default()
        .gauge_style(Style::default().fg(color).bg(COLOR_SURFACE))
        .ratio(progress)
        .label(label)
        .style(Style::default().fg(COLOR_TEXT));

    f.render_widget(gauge, area);
}

fn draw_bottom_section(f: &mut Frame, area: Rect, app: &App) {
    let tabs = Tabs::new(vec![" 🍅 Focus ", " ☕ Short Break ", " 🌿 Long Break "])
        .select(app.selected_tab)
        .style(Style::default().fg(COLOR_TEXT_DIM).bg(COLOR_SURFACE))
        .highlight_style(
            Style::default()
                .fg(COLOR_TEXT)
                .add_modifier(Modifier::BOLD)
                .bg(COLOR_BG),
        )
        .divider(symbols::line::VERTICAL);

    f.render_widget(tabs, area);

    // Settings info below tabs
    let settings_area = Rect {
        x: area.x,
        y: area.y + 3,
        width: area.width,
        height: area.height.saturating_sub(3),
    };

    if settings_area.height > 0 {
        let (setting_name, setting_value, setting_color) = match app.selected_tab {
            0 => ("Focus Duration", format!("{} min", app.pomodoro_duration), COLOR_ACCENT),
            1 => (
                "Short Break Duration",
                format!("{} min", app.short_break_duration),
                COLOR_BREAK,
            ),
            2 => (
                "Long Break Duration",
                format!("{} min", app.long_break_duration),
                COLOR_BLUE,
            ),
            _ => ("", String::new(), COLOR_TEXT),
        };

        let info_text = if app.state == TimerState::Idle {
            Line::from(vec![
                Span::styled("  ", Style::default()),
                Span::styled(
                    setting_name,
                    Style::default()
                        .fg(setting_color)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled(": ", Style::default().fg(COLOR_TEXT_DIM)),
                Span::styled(
                    setting_value,
                    Style::default().fg(COLOR_TEXT),
                ),
                Span::styled("  (use +/- to adjust)", Style::default().fg(COLOR_TEXT_DIM)),
            ])
        } else {
            Line::from(vec![
                Span::styled("  Sessions completed: ", Style::default().fg(COLOR_TEXT_DIM)),
                Span::styled(
                    format!("{} 🍅", app.pomodoros_completed),
                    Style::default()
                        .fg(COLOR_ACCENT)
                        .add_modifier(Modifier::BOLD),
                ),
                if app.pomodoros_completed > 0 && app.pomodoros_completed % 4 == 0 {
                    Span::styled("  🎉", Style::default().fg(COLOR_YELLOW))
                } else {
                    Span::styled("", Style::default())
                },
            ])
        };

        let info = Paragraph::new(info_text)
            .style(Style::default().bg(COLOR_BG))
            .alignment(Alignment::Left);
        f.render_widget(info, settings_area);
    }
}

fn draw_status_bar(f: &mut Frame, area: Rect, _app: &App) {
    let controls = vec![
        Span::styled(" Space ", Style::default().fg(COLOR_BG).bg(COLOR_TEXT).add_modifier(Modifier::BOLD)),
        Span::styled(" Start/Pause ", Style::default().fg(COLOR_TEXT)),
        Span::styled(" │ ", Style::default().fg(COLOR_BORDER)),
        Span::styled(" S ", Style::default().fg(COLOR_BG).bg(COLOR_TEXT).add_modifier(Modifier::BOLD)),
        Span::styled(" Stop ", Style::default().fg(COLOR_TEXT)),
        Span::styled(" │ ", Style::default().fg(COLOR_BORDER)),
        Span::styled(" K ", Style::default().fg(COLOR_BG).bg(COLOR_TEXT).add_modifier(Modifier::BOLD)),
        Span::styled(" Skip ", Style::default().fg(COLOR_TEXT)),
        Span::styled(" │ ", Style::default().fg(COLOR_BORDER)),
        Span::styled(" ←→ ", Style::default().fg(COLOR_BG).bg(COLOR_TEXT).add_modifier(Modifier::BOLD)),
        Span::styled(" Tabs ", Style::default().fg(COLOR_TEXT)),
        Span::styled(" │ ", Style::default().fg(COLOR_BORDER)),
        Span::styled(" +/- ", Style::default().fg(COLOR_BG).bg(COLOR_TEXT).add_modifier(Modifier::BOLD)),
        Span::styled(" Adjust ", Style::default().fg(COLOR_TEXT)),
        Span::styled(" │ ", Style::default().fg(COLOR_BORDER)),
        Span::styled(" Q ", Style::default().fg(COLOR_BG).bg(COLOR_ACCENT).add_modifier(Modifier::BOLD)),
        Span::styled(" Quit ", Style::default().fg(COLOR_TEXT)),
    ];

    let bar = Paragraph::new(Line::from(controls))
        .style(Style::default().bg(COLOR_SURFACE))
        .alignment(Alignment::Center);

    f.render_widget(bar, area);
}
