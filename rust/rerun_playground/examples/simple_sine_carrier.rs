
use std::f64::consts::PI;

fn main() -> anyhow::Result<()> {
    let recording = rerun::RecordingStreamBuilder::new("rerun_playground").save("simple_sine_carrier.rrd")?;

    let frequency_hz = 5.0;
    let samples_per_seconds = 1000.0;

    for i in 0..1000{
        let t = i as f64 / samples_per_seconds;
        let amplitude_1 = f64::sin(2.0 * PI * frequency_hz * t);
        let amplitude_2 = f64::sin(2.0 * PI * frequency_hz * t + PI / 3.0);
        recording.set_time_sequence("sample_idx", i);
        recording.set_duration_secs("time", t);
        recording.log("signal/satellite_1", &rerun::Scalars::single(amplitude_1))?;
        recording.log("signal/satellite_2", &rerun::Scalars::single(amplitude_2))?;
    }

    Ok(())
}