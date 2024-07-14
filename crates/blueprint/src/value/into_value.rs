use super::GraphValue;

pub trait IntoGraphValue {
    fn into_value(self) -> GraphValue;
}

impl IntoGraphValue for GraphValue {
    fn into_value(self) -> GraphValue {
        self
    }
}

impl IntoGraphValue for String {
    fn into_value(self) -> GraphValue {
        GraphValue::String(self)
    }
}

impl<'a> IntoGraphValue for &'a str {
    fn into_value(self) -> GraphValue {
        GraphValue::String(self.to_owned())
    }
}

impl IntoGraphValue for i32 {
    fn into_value(self) -> GraphValue {
        GraphValue::Int(self)
    }
}

impl IntoGraphValue for f32 {
    fn into_value(self) -> GraphValue {
        GraphValue::Float(self)
    }
}

impl IntoGraphValue for f64 {
    fn into_value(self) -> GraphValue {
        GraphValue::Double(self)
    }
}

impl IntoGraphValue for bevy_color::Color {
    fn into_value(self) -> GraphValue {
        let s = self.to_srgba();
        GraphValue::vec4(s.red, s.green, s.blue, s.alpha)
    }
}
