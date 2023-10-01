use midly::{TrackEventKind, MidiMessage, Smf, Timing};

pub struct MotorCommand {
    time: f64,
    on: bool,
    motor: u8,
    length: u8
}

pub fn main() {
    let motor_commands = parse_midi_into_motor_commands("Fur Elise.mid");
    for command in motor_commands {
        println!("time: {}, on: {}, motor: {}, speed: {}", command.time, command.on, command.motor, command.length);
    }
}

fn parse_midi_into_motor_commands(path: &str) -> Vec<MotorCommand> {
    let mut motor_commands = Vec::new();
    let mut tempo_map = Vec::new(); // New: a map of tempos over time

    let data = std::fs::read(path).unwrap();
    let smf = Smf::parse(&data).unwrap();
    let ticks_per_beat: u16;

    match smf.header.timing {
        Timing::Metrical(perbeat) => {ticks_per_beat = perbeat.into()},
        _ => {panic!("wrong time")} 
    }

    let mut tempo: f64 = 500_000.0;

    // First pass: Process all tracks for tempo changes
    for track in &smf.tracks {
        let mut absolute_time: f64 = 0.0;
        for event in track {
            let delta_time_in_seconds = (event.delta.as_int() as f64 / ticks_per_beat as f64) * (tempo / 1000000.0);
            absolute_time += delta_time_in_seconds;
            match event.kind {
                TrackEventKind::Meta(midly::MetaMessage::Tempo(new_tempo)) => {
                    tempo = new_tempo.as_int() as f64;
                    tempo_map.push((absolute_time, tempo)); // New: add each tempo change to the map
                },
                _ => {},
            }
        }
    }

    // Second pass: Process all tracks for MIDI messages
    for track in smf.tracks {
        let mut absolute_time: f64 = 0.0;
        for event in track {
            let delta_time_in_seconds = (event.delta.as_int() as f64 / ticks_per_beat as f64) * (tempo / 1000000.0);
            absolute_time += delta_time_in_seconds;
            match event.kind {
                TrackEventKind::Midi {message, ..} => {
                    // New: look up the correct tempo for this time from the map
                    if let Some((_, new_tempo)) = tempo_map.iter().take_while(|(time, _)| time <= &&absolute_time).last() {
                        tempo = *new_tempo;
                    }
                    let command = convert_to_motor_command(message);
                    motor_commands.push(MotorCommand { time: absolute_time, on: command.0, motor: command.1, length: command.2 });
                },
                _ => {},
            }
        }
    }

    motor_commands.sort_by(|a, b| a.time.partial_cmp(&b.time).unwrap());
    motor_commands
}

fn convert_to_motor_command(message: MidiMessage) -> (bool, u8, u8) {
    match message {
        MidiMessage::NoteOn { key, vel } => {
            (true, key.as_int(), 128-vel.as_int())
        },
        MidiMessage::NoteOff { key, vel } => {
            (false, key.as_int(), 128-vel.as_int())
        },
        _ => panic!(),
    }
}