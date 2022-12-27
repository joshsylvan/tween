declare_tween!(
    /// An quartic tween in. Go [here](https://easings.net/#easeInQuart) for a visual demonstration.
    pub struct QuartIn;

    /// Creates a new [QuartIn] Tweener.
    pub fn quart_in;

    pub fn tween<Value: crate::TweenValue>(&mut self, value_delta: Value, percent: f64) -> Value {
        value_delta.scale(percent * percent * percent * percent)
    }
);

declare_tween!(
    /// An quartic tween out. Go [here](https://easings.net/#easeOutQuart) for a visual demonstration.
    pub struct QuartOut;

    /// Creates a new [QuartOut] Tweener.
    pub fn quart_out;

    pub fn tween<Value: crate::TweenValue>(&mut self, value_delta: Value, mut percent: f64) -> Value {
        percent -= 1.0;
        value_delta.scale(-(percent * percent * percent * percent - 1.0))
    }
);

declare_tween!(
    /// An quartic tween in and out. Go [here](https://easings.net/#easeInOutQuart) for a visual demonstration.
    pub struct QuartInOut;

    /// Creates a new [QuartInOut] Tweener.
    pub fn quart_in_out;

    pub fn tween<Value: crate::TweenValue>(&mut self, value_delta: Value, mut percent: f64) -> Value {
        percent *= 2.0;

        let scalar = if percent < 1.0 {
            percent * percent * percent * percent
        } else {
            let p = percent - 2.0;
            -(p * p * p * p - 2.0)
        };
        value_delta.scale(scalar / 2.0)
    }
);

test_tween!(Quart);
