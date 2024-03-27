use std::time::Duration;

use hs_hackathon::car::{MotorSocket, WheelOrientation};
use hs_hackathon::drone::Camera;
use hs_hackathon::prelude::*;

use super::TeamColors;
use crate::cheats::positioning::distance;

/// This idles your car on the target
///
/// If the target is suddely moving away from the car,
/// this function returns and you should turn your car into
/// the direction of the target
pub async fn auto(
    colors: &TeamColors,
    drone: &mut Camera,
    _motor: &mut MotorSocket,
    _wheels: &mut WheelOrientation,
    led_config: &LedDetectionConfig,
) -> eyre::Result<()> {
    // todo: set a sane default for determining whether we are
    // "on" the target
    const SUCCESS_THRESHOLD: u32 = 50;

    'idle: loop {
        const IDLE_DURATION: Duration = Duration::from_secs(1);

        let (precar, pretarget) = super::internal::infer(colors, drone, led_config).await?;
        let pre = distance(&precar, &pretarget);

        tokio::time::sleep(IDLE_DURATION).await;

        let (currentcar, currenttarget) = super::internal::infer(colors, drone, led_config).await?;
        let current = distance(&currentcar, &currenttarget);

        // 1. if we were closer to the target before, recalibrate
        if pre <= current {
            return Ok(());
        }

        // 2. if the current distance is not on the target, recalibrate
        if current > SUCCESS_THRESHOLD {
            return Ok(());
        }

        // 3. stay on the target
        continue 'idle;
    }
}
