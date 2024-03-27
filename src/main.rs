mod cheats;
use std::time::Duration;
use hs_hackathon::prelude::*;

use cheats::angles::Vector;
use cheats::approaching::Hint;
use cheats::positioning::Position;
use cheats::TeamColors;

const CAR: Color = Color::Blue;
const TARGET: Color = Color::Green;

#[allow(unused)]
struct MapState {
    car: Position,
    target: Position,
}

#[allow(unused)]
impl MapState {
    pub async fn infer(drone: &mut Camera) -> eyre::Result<Self> {
        let frame = drone.snapshot().await?;
        let leds = detect(&frame.0, &LedDetectionConfig::default())?;

        let target = leds.iter().find(|led| led.color == TARGET).expect("Found the target");
        let car = leds.iter().find(|led| led.color == CAR).expect("Found the car");

        let target = Position {
            x: target.bbox.x_min(),
            y: target.bbox.y_min()
        };

        let car = Position {
            x: car.bbox.x_min(),
            y: car.bbox.y_min(),
        };

        Ok(Self {
            car,
            target,
        })
    }

    async fn car_orientation(
        current: Position,
        drone: &mut Camera,
        motor: &mut MotorSocket,
        wheels: &mut WheelOrientation,
    ) -> eyre::Result<Vector> {
        unimplemented!()
    }
}

#[derive(Debug)]
#[allow(unused)]
enum State {
    /// Turn the cars direction by doing consecutive front and back movements
    /// until the angle between the cars orientation and the target converges to be under
    /// a specified threshold
    Turning,
    /// Approach the car by doing incremental actions of approaching and measuring interleaved.
    /// So we approach the target a bit, measure if we decreased the distance, if yes repeat, if no
    /// then calibrate. We do this until we hit the target.
    Approaching,
    /// Simply idling on the target and identifying when the target moves away from our current
    /// position.
    Idle,
}

impl State {
    async fn execute(
        &mut self,
        drone: &mut Camera,
        motor: &mut MotorSocket,
        wheels: &mut WheelOrientation,
    ) -> eyre::Result<()> {
        match self {
            State::Turning => loop {
                // Calculate the current orientation of the car
                // position of the target
                // and the angle between the two
                // align the car to the target
                // move forward a bit
                
                wheels.set(Angle::left()).await?;
                motor.move_for(Velocity::forward(), Duration::from_secs(1)).await?;
                wheels.set(Angle::straight()).await?;
                motor.move_for(Velocity::forward(), Duration::from_millis(1)).await?;
                
                
                
                *self = Self::Approaching;
            },
            State::Approaching => {
                let hint = cheats::approaching::auto(
                    &TeamColors {
                        car: CAR,
                        target: TARGET,
                    },
                    drone,
                    motor,
                    wheels,
                )
                .await?;

                *self = match hint {
                    Hint::TargetWasHit => Self::Idle,
                    Hint::OrientationIsOff => Self::Turning,
                };
            }
            State::Idle => {
                cheats::idling::auto(
                    &TeamColors {
                        car: CAR,
                        target: TARGET,
                    },
                    drone,
                    motor,
                    wheels,
                )
                .await?;

                *self = Self::Turning;
            }
        }

        Ok(())
    }
}

#[hs_hackathon::main]
async fn main() -> eyre::Result<()> {
    let mut wheels = WheelOrientation::new().await?;
    let mut motor = MotorSocket::open().await?;
    let mut drone = Camera::connect().await?;

    let mut machine = State::Turning;

    loop {
        machine.execute(&mut drone, &mut motor, &mut wheels).await?;
        tracing::debug!("{:?}", machine);
    }
}


#[cfg(test)]
mod tests {
    use eyre::Context;

    use std::path::{Path, PathBuf};
    use hs_hackathon::prelude::{detect, LedDetectionConfig};
    use image::{codecs::jpeg::JpegDecoder, DynamicImage};

    #[test]
    fn test_position_gathering() {
        let p = PathBuf::from("./assets/33.jpg");
        let img = read_image(&p).unwrap();

        let mut config = LedDetectionConfig::default();
        config.max_size = (40, 40);
        config.width = 960;
        config.height = 720;

        let leds = detect(&img, &config).unwrap();

        println!("leds: {:?}", leds);
    }

    fn read_image(p: &Path) -> eyre::Result<DynamicImage> {
        let bytes = std::fs::read(p).wrap_err("read file")?;
        let decoder = JpegDecoder::new(bytes.as_slice()).wrap_err("launch decoder")?;
        let img = DynamicImage::from_decoder(decoder).wrap_err("decode frame")?;

        Ok(img)
    }
}