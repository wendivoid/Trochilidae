use std::rc::Rc;

use bevy_color::Color;
use bevy_utils::HashMap;

use crate::{ConversionError, ExecutionError};

use super::{IntoValue, ValueType};

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Int(i32),
    Float(f32),
    Double(f64),
    Bool(bool),
    String(String),
    List(Vec<Value>),
    Vec2(Box<Value>, Box<Value>),
    Tuple(Box<Value>, Box<Value>),
    Dictionary(HashMap<String, Value>),
    Vec3(Box<Value>, Box<Value>, Box<Value>),
    Vec4(Box<Value>, Box<Value>, Box<Value>, Box<Value>),
}

impl Value {
    pub fn as_value_type(&self) -> ValueType {
        match self {
            Self::Int(_) => ValueType::Int,
            Self::Float(_) => ValueType::Float,
            Self::Bool(_) => ValueType::Bool,
            Self::List(_) => ValueType::List,
            Self::Double(_) => ValueType::Double,
            Self::String(_) => ValueType::String,
            Self::Dictionary(_) => ValueType::Dictionary,
            Self::Vec2(a, _) => ValueType::Vec2(Rc::new(a.as_value_type())),
            Self::Vec3(a, _, _) => ValueType::Vec2(Rc::new(a.as_value_type())),
            Self::Vec4(a, _, _, _) => ValueType::Vec2(Rc::new(a.as_value_type())),
            Self::Tuple(a, b) => {
                ValueType::Tuple(Rc::new(a.as_value_type()), Rc::new(b.as_value_type()))
            }
        }
    }

    pub fn vec2<I1: IntoValue, I2: IntoValue>(a: I1, b: I2) -> Value {
        Value::Vec2(Box::new(a.into_value()), Box::new(b.into_value()))
    }

    pub fn vec3<I1: IntoValue, I2: IntoValue, I3: IntoValue>(a: I1, b: I2, c: I3) -> Value {
        Value::Vec3(Box::new(a.into_value()), Box::new(b.into_value()), Box::new(c.into_value()))
    }

    pub fn vec4<I1: IntoValue, I2: IntoValue, I3: IntoValue, I4: IntoValue>(a: I1, b: I2, c: I3, d: I4) -> Value {
        Value::Vec4(Box::new(a.into_value()), Box::new(b.into_value()), Box::new(c.into_value()), Box::new(d.into_value()))
    }

    pub fn to_int(self) -> Result<i32, ConversionError> {
        if let Self::Int(num) = self.to_int_value()? {
            Ok(num)
        } else {
            unreachable!();
        }
    }

    pub fn unwrap_int(self) -> i32 {
        match self {
            Self::Int(a) => a,
            other => panic!("Called unwrap_int on None Int value: {other:?}"),
        }
    }

    pub fn to_float(self) -> Result<f32, ConversionError> {
        if let Self::Float(num) = self.to_float_value()? {
            Ok(num)
        } else {
            unreachable!();
        }
    }

    pub fn to_double(self) -> Result<f64, ConversionError> {
        if let Self::Double(num) = self.to_double_value()? {
            Ok(num)
        } else {
            unreachable!();
        }
    }

    pub fn to_string(self) -> Result<String, ConversionError> {
        if let Self::String(s) = self.to_string_value()? {
            Ok(s)
        } else {
            unreachable!();
        }
    }

    pub fn to_bool(self) -> Result<bool, ConversionError> {
        if let Self::Bool(b) = self.to_bool_value()? {
            Ok(b)
        } else {
            unreachable!();
        }
    }

    pub fn to_tuple(
        self,
        a: &Rc<ValueType>,
        b: &Rc<ValueType>,
    ) -> Result<(Value, Value), ConversionError> {
        if let Self::Tuple(a, b) = self.to_tuple_value(a, b)? {
            Ok((*a, *b))
        } else {
            unreachable!();
        }
    }

    pub fn to_color(self) -> Result<Color, ConversionError> {
        if let Self::Vec4(a, b, c, d) = self.to_vec4_value(&Rc::new(ValueType::Float))? {
            Ok(Color::srgba(
                a.to_float()? as f32,
                b.to_float()? as f32,
                c.to_float()? as f32,
                d.to_float()? as f32,
            ))
        } else {
            unreachable!();
        }
    }

    pub fn to_int_value(self) -> Result<Self, ConversionError> {
        let self_type = self.as_value_type();
        match self {
            Self::Int(a) => Ok(Self::Int(a)),
            Self::Float(a) => Ok(Self::Int(a as i32)),
            Self::Double(a) => Ok(Self::Int(a as i32)),
            _ => Err(ConversionError::new(self_type, ValueType::Int)),
        }
    }

    pub fn to_float_value(self) -> Result<Self, ConversionError> {
        let self_type = self.as_value_type();
        match self {
            Self::Int(a) => Ok(Self::Float(a as f32)),
            Self::Float(a) => Ok(Self::Float(a)),
            Self::Double(a) => Ok(Self::Float(a as f32)),
            _ => Err(ConversionError::new(self_type, ValueType::Float)),
        }
    }

    pub fn to_double_value(self) -> Result<Self, ConversionError> {
        let self_type = self.as_value_type();
        match self {
            Self::Int(a) => Ok(Self::Double(a as f64)),
            Self::Float(a) => Ok(Self::Double(a as f64)),
            Self::Double(a) => Ok(Self::Double(a)),
            _ => Err(ConversionError::new(self_type, ValueType::Double)),
        }
    }

    pub fn to_bool_value(self) -> Result<Self, ConversionError> {
        let self_type = self.as_value_type();
        match self {
            Self::Bool(a) => Ok(Self::Bool(a)),
            _ => Err(ConversionError::new(self_type, ValueType::Bool)),
        }
    }

    pub fn to_string_value(self) -> Result<Self, ConversionError> {
        let self_type = self.as_value_type();
        match self {
            Self::String(a) => Ok(Self::String(a)),
            _ => Err(ConversionError::new(self_type, ValueType::String)),
        }
    }

    pub fn to_dictionary_value(self) -> Result<Self, ConversionError> {
        let self_type = self.as_value_type();
        match self {
            Self::Dictionary(a) => Ok(Self::Dictionary(a)),
            _ => Err(ConversionError::new(self_type, ValueType::Dictionary)),
        }
    }

    pub fn to_vec2_value(self, into: &Rc<ValueType>) -> Result<Value, ConversionError> {
        match self {
            Self::Vec2(from1, from2) => Ok(Value::vec2(
                from1.into_type(into)?,
                from2.into_type(into)?,
            )),
            _ => Err(ConversionError {
                from: self.as_value_type(),
                to: ValueType::Vec2(into.clone()),
            }),
        }
    }
    pub fn to_vec3_value(self, into: &Rc<ValueType>) -> Result<Value, ConversionError> {
        match self {
            Self::Vec3(from1, from2, from3) => Ok(Value::vec3(
                from1.into_type(into)?,
                from2.into_type(into)?,
                from3.into_type(into)?,
            )),
            _ => Err(ConversionError {
                from: self.as_value_type(),
                to: ValueType::Vec3(into.clone()),
            }),
        }
    }
    pub fn to_vec4_value(self, inner: &Rc<ValueType>) -> Result<Value, ConversionError> {
        match self {
            Self::Vec4(from1, from2, from3, from4) => Ok(Value::vec4(
                from1.into_type(inner)?,
                from2.into_type(inner)?,
                from3.into_type(inner)?,
                from4.into_type(inner)?,
            )),
            _ => Err(ConversionError {
                from: self.as_value_type(),
                to: ValueType::Vec4(inner.clone()),
            }),
        }
    }

    pub fn to_tuple_value(
        self,
        a: &Rc<ValueType>,
        b: &Rc<ValueType>,
    ) -> Result<Value, ConversionError> {
        match self {
            Self::Tuple(from1, from2) => Ok(Value::Tuple(
                Box::new(from1.into_type(a)?),
                Box::new(from2.into_type(b)?),
            )),
            _ => Err(ConversionError {
                from: self.as_value_type(),
                to: ValueType::Tuple(a.clone(), b.clone()),
            }),
        }
    }

    pub fn to_list_value(self) -> Result<Value, ConversionError> {
        match self {
            Self::List(a) => Ok(Self::List(a)),
            other => Ok(Self::List(vec![other])),
        }
    }

    pub fn to_list(self) -> Result<Vec<Value>, ConversionError> {
        match self {
            Self::List(a) => Ok(a),
            other => Ok(vec![other]),
        }
    }

    pub fn into_type(self, ty: &Rc<ValueType>) -> Result<Value, ConversionError> {
        match ty.as_ref() {
            ValueType::Any => Ok(self.into_value()),
            ValueType::Int => self.to_int_value(),
            ValueType::Float => self.to_float_value(),
            ValueType::Double => self.to_double_value(),
            ValueType::Bool => self.to_bool_value(),
            ValueType::List => self.to_list_value(),
            ValueType::String => self.to_string_value(),
            ValueType::Dictionary => self.to_dictionary_value(),
            ValueType::Tuple(a, b) => self.to_tuple_value(a, b),
            ValueType::Vec2(inner) => self.to_vec2_value(inner),
            ValueType::Vec3(inner) => self.to_vec3_value(inner),
            ValueType::Vec4(inner) => self.to_vec4_value(inner),
        }
    }

    pub fn add(self, other: Value) -> Result<Value, ExecutionError> {
        match self {
            Value::Int(a) => Ok(Value::Int(a + other.to_int()?)),
            Value::Float(a) => Ok(Value::Float(a + other.to_float()?)),
            Value::Double(a) => Ok(Value::Double(a + other.to_double()?)),
            Value::Vec2(a1, b1) => {
                let inner_type = a1.as_value_type();
                match other {
                    Value::Int(a2) => match inner_type {
                        ValueType::Int => Ok(Value::vec2(
                            Value::Int((a1.unwrap_int() + a2).into()),
                            Value::Int(b1.unwrap_int() + a2),
                        )),
                        ValueType::Float => Ok(Value::vec2(
                            Value::Float(a1.to_float().unwrap() + a2 as f32),
                            Value::Float(b1.to_float().unwrap() + a2 as f32),
                        )),
                        other_type => Err(ExecutionError::Conversion(ConversionError::new(
                            other_type,
                            ValueType::Float,
                        ))),
                    },
                    Value::Float(a2) => match inner_type {
                        ValueType::Int => Ok(Value::vec2(
                            Value::Int(a1.unwrap_int() + a2 as i32),
                            Value::Int(b1.unwrap_int() + a2 as i32),
                        )),
                        ValueType::Float => Ok(Value::vec2(
                            Value::Float(a1.to_float().unwrap() as f32 + a2),
                            Value::Float(b1.to_float().unwrap() as f32 + a2),
                        )),
                        other_type => Err(ExecutionError::Conversion(ConversionError::new(
                            other_type,
                            ValueType::Float,
                        ))),
                    },
                    _ => todo!(),
                }
            }
            _ => todo!(),
        }
    }

    pub fn sub(self, other: Value) -> Result<Value, ExecutionError> {
        match self {
            Value::Int(a) => Ok(Value::Int(a - other.to_int()?)),
            Value::Float(a) => Ok(Value::Float(a - other.to_float()?)),
            Value::Double(a) => Ok(Value::Double(a - other.to_double()?)),
            Value::Vec2(a1, b1) => {
                let inner_type = a1.as_value_type();
                match other {
                    Value::Int(a2) => match inner_type {
                        ValueType::Int => Ok(Value::vec2(
                            a1.unwrap_int() - a2,
                            b1.unwrap_int() - a2,
                        )),
                        ValueType::Float => Ok(Value::vec2(
                            a1.to_float().unwrap() - a2 as f32,
                            b1.to_float().unwrap() - a2 as f32,
                        )),
                        other_type => Err(ExecutionError::Conversion(ConversionError::new(
                            other_type,
                            ValueType::Float,
                        ))),
                    },
                    Value::Float(a2) => match inner_type {
                        ValueType::Int => Ok(Value::vec2(
                            Value::Int(a1.unwrap_int() - a2 as i32),
                            Value::Int(b1.unwrap_int() - a2 as i32),
                        )),
                        ValueType::Float => Ok(Value::vec2(
                            Value::Float(a1.to_float().unwrap() - a2),
                            Value::Float(b1.to_float().unwrap() - a2),
                        )),
                        other_type => Err(ExecutionError::Conversion(ConversionError::new(
                            other_type,
                            ValueType::Float,
                        ))),
                    },
                    _ => todo!(),
                }
            }
            _ => todo!(),
        }
    }

    pub fn mul(self, other: Value) -> Result<Value, ExecutionError> {
        match self {
            Value::Int(a) => Ok(Value::Int(a * other.to_int()?)),
            Value::Float(a) => Ok(Value::Float(a * other.to_float()?)),
            Value::Double(a) => Ok(Value::Double(a * other.to_double()?)),
            Value::Vec2(a1, b1) => {
                let inner_type = a1.as_value_type();
                match other {
                    Value::Int(a2) => match inner_type {
                        ValueType::Int => Ok(Value::Vec2(
                            Box::new(Value::Int(a1.to_int()? * a2)),
                            Box::new(Value::Int(b1.to_int()? * a2)),
                        )),
                        ValueType::Float => Ok(Value::vec2(
                            a1.to_int()? * a2,
                            b1.to_int()? * a2,
                        )),
                        other_type => Err(ExecutionError::Conversion(ConversionError::new(
                            other_type,
                            ValueType::Float,
                        ))),
                    },
                    Value::Float(a2) => match inner_type {
                        ValueType::Int => Ok(Value::vec2(
                            a1.to_float()? * a2,
                            b1.to_float()? * a2,
                        )),
                        ValueType::Float => Ok(Value::vec2(
                            a1.to_float()? * a2,
                            b1.to_float()? * a2,
                        )),
                        other_type => Err(ExecutionError::Conversion(ConversionError::new(
                            other_type,
                            ValueType::Float,
                        ))),
                    },
                    Value::Double(a2) => match inner_type {
                        ValueType::Int => Ok(Value::vec2(
                            a1.to_double()? * a2,
                            b1.to_double()? * a2,
                        )),
                        ValueType::Float => Ok(Value::vec2(
                            a1.to_double()? * a2,
                            b1.to_double()? * a2,
                        )),
                        other_type => Err(ExecutionError::Conversion(ConversionError::new(
                            other_type,
                            ValueType::Float,
                        ))),
                    },
                    Value::Vec2(a2, b2) => {
                        Ok(Value::vec2(a1.mul(*a2)?, b1.mul(*b2)?))
                    }
                    _ => todo!(),
                }
            }
            _ => todo!(),
        }
    }
}
