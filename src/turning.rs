use std::time::Duration;

use hs_hackathon::car::{Angle, MotorSocket, Velocity, WheelOrientation};
use hs_hackathon::drone::Camera;
use hs_hackathon::prelude::eyre;
use crate::cheats::approaching::Hint;
use crate::cheats::positioning::distance;
use crate::cheats::TeamColors;

/// Hacked-up turning function
pub async fn auto(
    colors: &TeamColors,
    drone: &mut Camera,
    motor: &mut MotorSocket,
    wheels: &mut WheelOrientation,
) -> eyre::Result<Hint> {
    unimplemented!("TODO");
    // const TURNING_DURATION: Duration = Duration::from_secs(1);
    //
    // 'turning: loop {
    //     let (precar, pretarget) = crate::cheats::internal::infer(colors, drone).await?;
    //     let pre = distance(&precar, &pretarget);
    //
    //     wheels.set(Angle::straight()).await?;
    //     motor
    //         .move_for(Velocity::forward(), APPROACHING_DURATION)
    //         .await?;
    //
    //     let (currentcar, currenttarget) = crate::cheats::internal::infer(colors, drone).await?;
    //     let current = distance(&currentcar, &currenttarget);
    //
    //     // 1. if current is in a success threshold difference
    //     if current <= SUCCESS_THRESHOLD {
    //         return Ok(Hint::TargetWasHit);
    //     }
    //
    //     // 2. if we were closer before approaching or didnt move, calibrate
    //     if pre <= current {
    //         return Ok(Hint::OrientationIsOff);
    //     }
    //
    //     // 3. continue with approaching
    //     continue 'turning;
    // }
}
