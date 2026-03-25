use std::f32::consts::PI;
use rand_distr::Distribution;

fn main() -> anyhow::Result<()> {

    let timestam_as_string = format!("{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());
    let recording = rerun::RecordingStreamBuilder::new("rerun_playground").save(format!("iq_constellation_{}.rrd", timestam_as_string))?;

    let frequency_hz = 5.0f32;
    let samples_per_seconds = 200.0f32;
    let normal = rand_distr::Normal::new(0.0f32, 0.05f32).unwrap(); // mean=0, std_dev=0.05
    for frame in 0..200{


        let t = frame as f32 / samples_per_seconds;
        let phase = 2.0f32 * PI * frequency_hz * t;
        let radius = 3.0f32;

        let authentic_points_iq : Vec<(f32,f32)> = (0..100).map(
            |_| {
                let i = radius* f32::cos(phase) + normal.sample(&mut rand::rng());
                let q = radius* f32::sin(phase) + normal.sample(&mut rand::rng());
                (i, q)
            }
        ).collect();

        let spoofed_points_iq : Vec<(f32,f32)> = (0..100).map(
            |_| {
                let i = radius* f32::cos(phase + PI / 2.0) + 3.0f32 * normal.sample(&mut rand::rng());
                let q = radius* f32::sin(phase + PI / 2.0) + 3.0f32 * normal.sample(&mut rand::rng());
                (i, q)
            }
        ).collect();

        recording.set_duration_secs("time", t);

        recording.log(
            "constellation/authentic",
            &rerun::Points2D::new(authentic_points_iq)
                .with_colors([rerun::Color::from_rgb(0, 200, 0)])
                .with_radii([0.001]),
        )?;
        recording.log(
            "constellation/spoofed",
            &rerun::Points2D::new(spoofed_points_iq)
                .with_colors([rerun::Color::from_rgb(200, 0, 0)])
                .with_radii([0.001]),
        )?;
    }

    Ok(())
}