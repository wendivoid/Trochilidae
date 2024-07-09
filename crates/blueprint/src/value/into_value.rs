use super::Value;

pub trait IntoValue {
    fn into_value(self) -> Value;
}

impl IntoValue for Value {
    fn into_value(self) -> Value {
        self
    }
}

impl IntoValue for String {
    fn into_value(self) -> Value {
        Value::String(self)
    }
}

impl <'a>IntoValue for &'a str {
    fn into_value(self) -> Value {
        Value::String(self.to_owned())
    }
}

impl IntoValue for i32 {
    fn into_value(self) -> Value {
        Value::Int(self)
    }
}

impl IntoValue for f32 {
    fn into_value(self) -> Value {
        Value::Float(self)
    }
}

impl IntoValue for f64 {
    fn into_value(self) -> Value {
        Value::Double(self)
    }
}

impl IntoValue for bevy_color::Color {
    fn into_value(self) -> Value {
        let s = self.to_srgba();
        Value::vec4(s.red, s.green, s.blue, s.alpha)
    }
}