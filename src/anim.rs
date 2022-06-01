use std::{borrow::Borrow, time::Duration};

use keyframe::EasingFunction;

#[inline]
pub fn run<F: EasingFunction>(
    function: impl Borrow<F>,
    seconds: f32,
    offset: f32,
    time: Duration,
) -> f32 {
    return keyframe::ease(
        function,
        0.0,
        1.0,
        ((time.as_secs_f32() - offset) / seconds).clamp(0.0, 1.0),
    );
}
