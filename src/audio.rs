use std::process::{Command, Stdio};

/// Play the Cristiano Ronaldo "Sui" sound effect
fn play_sui_sound() {
    let sound_data = include_bytes!("../assets/sui.wav");

    match Command::new("paplay")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn() 
    {
        Ok(mut child) => {
            if let Some(mut stdin) = child.stdin.take() {
                if let Err(e) = std::io::Write::write_all(&mut stdin, sound_data) {
                    eprintln!("Failed to write audio data to paplay: {}", e);
                }
            }
        }
        Err(e) => eprintln!("Failed to play sound: {}", e),
    }
}

pub fn play_pomodoro_start() {
    play_sui_sound();
}

pub fn play_pomodoro_end() {
    play_sui_sound();
}

pub fn play_break_start() {
    play_sui_sound();
}

pub fn play_break_end() {
    play_sui_sound();
}
