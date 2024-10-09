use cpal::traits::{ DeviceTrait, HostTrait, StreamTrait };
use std::{ error::Error, time::Duration };
use hound;

fn run_audio<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig
) -> Result<cpal::Stream, Box<dyn Error>>
    where T: cpal::Sample + From<f32> + cpal::SizedSample
{
    let err_fn = |err| eprintln!("An error occurred on the output audio stream: {}", err);
    let mut sample_clock = 0f32;
    let sample_rate = config.sample_rate.0 as f32;
    let mut next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        ((sample_clock * 2.0 * 3.141592) / sample_rate).sin()
    };
    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            for sample in data.iter_mut() {
                *sample = T::from(next_value());
            }
        },
        err_fn,
        Some(Duration::from_secs(30))
    )?;

    Ok(stream)
}

pub fn play_audio() -> Result<(), Box<dyn Error>> {
    let host = cpal::default_host();
    let device = host.default_output_device().ok_or("No output device available")?;
    let config = device.default_output_config()?;

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => run_audio::<f32>(&device, &config.into())?,
        _ => todo!(),
    };
    stream.play()?;
    println!("Playing audio...");
    std::thread::sleep(std::time::Duration::from_secs(10));
    Ok(())
}

pub fn play_audio_from_file(file_path: &str) -> Result<(), Box<dyn Error>> {
    let reader = hound::WavReader::open(file_path).unwrap();
    let spec = reader.spec();

    println!("Playing audio from file: {:?}", spec);

    let host = cpal::default_host();
    let device = host.default_output_device().ok_or("No output device available")?;
    let config = cpal::StreamConfig {
        channels: spec.channels as u16,
        sample_rate: cpal::SampleRate(spec.sample_rate),
        buffer_size: cpal::BufferSize::Fixed(spec.sample_rate as u32),
    };

    let stream = match spec.sample_format {
        hound::SampleFormat::Float => run_audio::<f32>(&device, &config)?,
        // hound::SampleFormat::Int => run_audio::<i16>(&device, &config)?,
        _ => todo!(),
    };

    stream.play()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_audio() {
        let path = "assets/sample_speech.wav";
        let result = play_audio_from_file(&path);
        assert!(result.is_ok());
    }
}
