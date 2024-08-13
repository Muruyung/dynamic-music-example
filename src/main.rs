use console::{Key, Term};

use std::{error::Error, time::Duration};

use kira::{
	effect::filter::FilterBuilder, manager::{backend::DefaultBackend, AudioManager, AudioManagerSettings}, modulator::tweener::TweenerBuilder, sound::static_sound::{StaticSoundData, StaticSoundSettings}, track::TrackBuilder, tween::{ModulatorMapping, Tween, Value}, Volume
};

fn main() -> Result<(), Box<dyn Error>> {
    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?;

    let music_duration = 29.2 + 1.0 / 3.0;
    let common_settings = 
        StaticSoundSettings::new().loop_region(music_duration/2.0..music_duration);

    let mut lead_tweener = manager.add_modulator(TweenerBuilder {initial_value: 0.0})?;
    let mut electro_tweener = manager.add_modulator(TweenerBuilder {initial_value: 0.0})?;
    let mut funkot_tweener = manager.add_modulator(TweenerBuilder {initial_value: 0.0})?;
    let mut koplo_tweener = manager.add_modulator(TweenerBuilder {initial_value: 0.0})?;

    let lead_track = manager.add_sub_track(
        TrackBuilder::new()
            .with_effect(FilterBuilder::new().cutoff(Value::from_modulator(
                &lead_tweener,
                ModulatorMapping {
                    input_range: (1.0, 0.0),
                    output_range: (20_000.0, 2000.0),
                    ..Default::default()
                },
            ))),
    )?;

    let common_effect = manager.add_sub_track(
        TrackBuilder::new()
            .with_effect(FilterBuilder::new().cutoff(Value::Fixed(20_000.0)))
    )?;

    let drum_electro_track = StaticSoundData::from_file(
        "assets/Chill days-Drum Electro.mp3",
    )?.with_settings(common_settings.volume(Value::from_modulator(
        &electro_tweener,
        ModulatorMapping {
            input_range: (0.0, 1.0),
            output_range: (Volume::Amplitude(0.0), Volume::Amplitude(0.5)),
            ..Default::default()
        }
    )).output_destination(&common_effect));

    let drum_funkot_track = StaticSoundData::from_file(
        "assets/Chill days-Drum Funkot.mp3",
    )?.with_settings(common_settings.volume(Value::from_modulator(
        &funkot_tweener,
        ModulatorMapping {
            input_range: (0.0, 1.0),
            output_range: (Volume::Amplitude(0.0), Volume::Amplitude(1.2)),
            ..Default::default()
        }
    )).output_destination(&common_effect));

    let flute_track = StaticSoundData::from_file(
        "assets/Chill days-Flute.mp3",
    )?.with_settings(common_settings.volume(Value::from_modulator(
        &koplo_tweener,
        ModulatorMapping {
            input_range: (0.0, 1.0),
            output_range: (Volume::Amplitude(0.0), Volume::Amplitude(0.3)),
            ..Default::default()
        }
    )));

    let drum_koplo_track = StaticSoundData::from_file(
        "assets/Chill days-Koplo.mp3",
    )?.with_settings(common_settings.volume(Value::from_modulator(
        &koplo_tweener,
        ModulatorMapping {
            input_range: (0.0, 1.0),
            output_range: (Volume::Amplitude(0.0), Volume::Amplitude(0.5)),
            ..Default::default()
        }
    )).output_destination(&common_effect));

    let piano_track = StaticSoundData::from_file(
        "assets/Chill days-Piano.mp3",
    )?.with_settings(common_settings.output_destination(&lead_track));

    manager.play(piano_track)?;
    manager.play(drum_electro_track)?;
    manager.play(drum_funkot_track)?;
    manager.play(flute_track)?;
    manager.play(drum_koplo_track)?;

    let term = Term::stdout();
    let _ = term.write_line("Press 1, 2, 3, or 4 to change music. Esc or Enter to exit");
    loop {
        match press_key(term.clone()) {
            Key::Char('1') => {
                let _ = term.write_line(&format!("Playing basic music (1)"));
                lead_tweener.set(
                    1.0,
                    Tween {
                        duration: Duration::from_secs(3),
                        ..Default::default()
                    },
                );
                electro_tweener.set(
                    0.0,
                    Tween {
                        duration: Duration::from_secs(3),
                        ..Default::default()
                    },
                );
                funkot_tweener.set(
                    0.0,
                    Tween {
                        duration: Duration::from_secs(3),
                        ..Default::default()
                    },
                );
                koplo_tweener.set(
                    0.0,
                    Tween {
                        duration: Duration::from_secs(3),
                        ..Default::default()
                    },
                );
            },
            Key::Char('2') => {
                let _ = term.write_line(&format!("Playing electro music (2)"));
                lead_tweener.set(
                    0.0,
                    Tween {
                        duration: Duration::from_secs(3),
                        ..Default::default()
                    },
                );
                electro_tweener.set(
                    1.0,
                    Tween {
                        duration: Duration::from_secs(3),
                        ..Default::default()
                    },
                );
                funkot_tweener.set(
                    0.0,
                    Tween {
                        duration: Duration::from_secs(3),
                        ..Default::default()
                    },
                );
                koplo_tweener.set(
                    0.0,
                    Tween {
                        duration: Duration::from_secs(3),
                        ..Default::default()
                    },
                );
            },
            Key::Char('3') => {
                let _ = term.write_line(&format!("Playing funkot music (3)"));
                lead_tweener.set(
                    0.0,
                    Tween {
                        duration: Duration::from_secs(3),
                        ..Default::default()
                    },
                );
                electro_tweener.set(
                    0.0,
                    Tween {
                        duration: Duration::from_secs(3),
                        ..Default::default()
                    },
                );
                funkot_tweener.set(
                    1.0,
                    Tween {
                        duration: Duration::from_secs(3),
                        ..Default::default()
                    },
                );
                koplo_tweener.set(
                    0.0,
                    Tween {
                        duration: Duration::from_secs(3),
                        ..Default::default()
                    },
                );
            },
            Key::Char('4') => {
                let _ = term.write_line(&format!("Playing koplo music (4)"));
                lead_tweener.set(
                    0.0,
                    Tween {
                        duration: Duration::from_secs(3),
                        ..Default::default()
                    },
                );
                electro_tweener.set(
                    0.0,
                    Tween {
                        duration: Duration::from_secs(3),
                        ..Default::default()
                    },
                );
                funkot_tweener.set(
                    0.0,
                    Tween {
                        duration: Duration::from_secs(3),
                        ..Default::default()
                    },
                );
                koplo_tweener.set(
                    1.0,
                    Tween {
                        duration: Duration::from_secs(3),
                        ..Default::default()
                    },
                );
            },
            Key::Escape | Key::Enter => {
                break;
            }
            _ =>{}
        }
    }
    Ok(())
}

fn press_key(term: Term) -> Key {
    let key = match term.read_key(){
        Ok(key) => key,
        Err(_) => todo!(),
    };

    key
    // if key == Key::Escape {
    //     break;
    // }

    // match key {
    //     Key::Char('1') | Key::Char('2') | Key::Char('3') | Key::Char('4') => {
    //         let _ = term.write_line(&format!("Music option: {:?}", key));
    //     },
    //     _ => {
    //         let _ = term.write_line(&format!("Invalid key"));
    //     }
    // }
}
