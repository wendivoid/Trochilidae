use std::rc::Rc;

use bevy_color::Color;
use bevy_utils::HashMap;

use crate::{ConversionError, ExecutionError};

use super::{IntoGraphValue, GraphValueType};

#[derive(Debug, PartialEq, Clone)]
pub enum GraphValue {
    Int(i32),
    Float(f32),
    Double(f64),
    Bool(bool),
    String(String),
    List(Vec<GraphValue>),
    Vec2(Box<GraphValue>, Box<GraphValue>),
    Tuple(Box<GraphValue>, Box<GraphValue>),
    Dictionary(HashMap<String, GraphValue>),
    Vec3(Box<GraphValue>, Box<GraphValue>, Box<GraphValue>),
    Vec4(Box<GraphValue>, Box<GraphValue>, Box<GraphValue>, Box<GraphValue>),
}

impl GraphValue {
    pub fn as_value_type(&self) -> GraphValueType {
        match self {
            Self::Int(_) => GraphValueType::Int,
            Self::Float(_) => GraphValueType::Float,
            Self::Bool(_) => GraphValueType::Bool,
            Self::List(_) => GraphValueType::List,
            Self::Double(_) => GraphValueType::Double,
            Self::String(_) => GraphValueType::String,
            Self::Dictionary(_) => GraphValueType::Dictionary,
            Self::Vec2(a, _) => GraphValueType::Vec2(Rc::new(a.as_value_type())),
            Self::Vec3(a, _, _) => GraphValueType::Vec2(Rc::new(a.as_value_type())),
            Self::Vec4(a, _, _, _) => GraphValueType::Vec2(Rc::new(a.as_value_type())),
            Self::Tuple(a, b) => {
                GraphValueType::Tuple(Rc::new(a.as_value_type()), Rc::new(b.as_value_type()))
            }
        }
    }

    pub fn vec2<I1: IntoGraphValue, I2: IntoGraphValue>(a: I1, b: I2) -> GraphValue {
        GraphValue::Vec2(Box::new(a.into_value()), Box::new(b.into_value()))
    }

    pub fn vec3<I1: IntoGraphValue, I2: IntoGraphValue, I3: IntoGraphValue>(a: I1, b: I2, c: I3) -> GraphValue {
        GraphValue::Vec3(Box::new(a.into_value()), Box::new(b.into_value()), Box::new(c.into_value()))
    }

    pub fn vec4<I1: IntoGraphValue, I2: IntoGraphValue, I3: IntoGraphValue, I4: IntoGraphValue>(a: I1, b: I2, c: I3, d: I4) -> GraphValue {
        GraphValue::Vec4(Box::new(a.into_value()), Box::new(b.into_value()), Box::new(c.into_value()), Box::new(d.into_value()))
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
            other => panic!("Called unwrap_int on None Int GraphValue: {other:?}"),
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
        a: &Rc<GraphValueType>,
        b: &Rc<GraphValueType>,
    ) -> Result<(GraphValue, GraphValue), ConversionError> {
        if let Self::Tuple(a, b) = self.to_tuple_value(a, b)? {
            Ok((*a, *b))
        } else {
            unreachable!();
        }
    }

    pub fn to_color(self) -> Result<Color, ConversionError> {
        if let Self::Vec4(a, b, c, d) = self.to_vec4_value(&Rc::new(GraphValueType::Float))? {
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
            _ => Err(ConversionError::new(self_type, GraphValueType::Int)),
        }
    }

    pub fn to_float_value(self) -> Result<Self, ConversionError> {
        let self_type = self.as_value_type();
        match self {
            Self::Int(a) => Ok(Self::Float(a as f32)),
            Self::Float(a) => Ok(Self::Float(a)),
            Self::Double(a) => Ok(Self::Float(a as f32)),
            _ => Err(ConversionError::new(self_type, GraphValueType::Float)),
        }
    }

    pub fn to_double_value(self) -> Result<Self, ConversionError> {
        let self_type = self.as_value_type();
        match self {
            Self::Int(a) => Ok(Self::Double(a as f64)),
            Self::Float(a) => Ok(Self::Double(a as f64)),
            Self::Double(a) => Ok(Self::Double(a)),
            _ => Err(ConversionError::new(self_type, GraphValueType::Double)),
        }
    }

    pub fn to_bool_value(self) -> Result<Self, ConversionError> {
        let self_type = self.as_value_type();
        match self {
            Self::Bool(a) => Ok(Self::Bool(a)),
            _ => Err(ConversionError::new(self_type, GraphValueType::Bool)),
        }
    }

    pub fn to_string_value(self) -> Result<Self, ConversionError> {
        let self_type = self.as_value_type();
        match self {
            Self::String(a) => Ok(Self::String(a)),
            _ => Err(ConversionError::new(self_type, GraphValueType::String)),
        }
    }

    pub fn to_dictionary_value(self) -> Result<Self, ConversionError> {
        let self_type = self.as_value_type();
        match self {
            Self::Dictionary(a) => Ok(Self::Dictionary(a)),
            _ => Err(ConversionError::new(self_type, GraphValueType::Dictionary)),
        }
    }

    pub fn to_vec2_value(self, into: &Rc<GraphValueType>) -> Result<GraphValue, ConversionError> {
        match self {
            Self::Vec2(from1, from2) => Ok(GraphValue::vec2(
                from1.into_type(into)?,
                from2.into_type(into)?,
            )),
            _ => Err(ConversionError {
                from: self.as_value_type(),
                to: GraphValueType::Vec2(into.clone()),
            }),
        }
    }
    pub fn to_vec3_value(self, into: &Rc<GraphValueType>) -> Result<GraphValue, ConversionError> {
        match self {
            Self::Vec3(from1, from2, from3) => Ok(GraphValue::vec3(
                from1.into_type(into)?,
                from2.into_type(into)?,
                from3.into_type(into)?,
            )),
            _ => Err(ConversionError {
                from: self.as_value_type(),
                to: GraphValueType::Vec3(into.clone()),
            }),
        }
    }
    pub fn to_vec4_value(self, inner: &Rc<GraphValueType>) -> Result<GraphValue, ConversionError> {
        match self {
            Self::Vec4(from1, from2, from3, from4) => Ok(GraphValue::vec4(
                from1.into_type(inner)?,
                from2.into_type(inner)?,
                from3.into_type(inner)?,
                from4.into_type(inner)?,
            )),
            _ => Err(ConversionError {
                from: self.as_value_type(),
                to: GraphValueType::Vec4(inner.clone()),
            }),
        }
    }

    pub fn to_tuple_value(
        self,
        a: &Rc<GraphValueType>,
        b: &Rc<GraphValueType>,
    ) -> Result<GraphValue, ConversionError> {
        match self {
            Self::Tuple(from1, from2) => Ok(GraphValue::Tuple(
                Box::new(from1.into_type(a)?),
                Box::new(from2.into_type(b)?),
            )),
            _ => Err(ConversionError {
                from: self.as_value_type(),
                to: GraphValueType::Tuple(a.clone(), b.clone()),
            }),
        }
    }

    pub fn to_list_value(self) -> Result<GraphValue, ConversionError> {
        match self {
            Self::List(a) => Ok(Self::List(a)),
            other => Ok(Self::List(vec![other])),
        }
    }

    pub fn to_list(self) -> Result<Vec<GraphValue>, ConversionError> {
        match self {
            Self::List(a) => Ok(a),
            other => Ok(vec![other]),
        }
    }

    pub fn into_type(self, ty: &Rc<GraphValueType>) -> Result<GraphValue, ConversionError> {
        match ty.as_ref() {
            GraphValueType::Any => Ok(self.into_value()),
            GraphValueType::Int => self.to_int_value(),
            GraphValueType::Float => self.to_float_value(),
            GraphValueType::Double => self.to_double_value(),
            GraphValueType::Bool => self.to_bool_value(),
            GraphValueType::List => self.to_list_value(),
            GraphValueType::String => self.to_string_value(),
            GraphValueType::Dictionary => self.to_dictionary_value(),
            GraphValueType::Tuple(a, b) => self.to_tuple_value(a, b),
            GraphValueType::Vec2(inner) => self.to_vec2_value(inner),
            GraphValueType::Vec3(inner) => self.to_vec3_value(inner),
            GraphValueType::Vec4(inner) => self.to_vec4_value(inner),
        }
    }

    pub fn add(self, other: GraphValue) -> Result<GraphValue, ExecutionError> {
        match self {
            GraphValue::Int(a) => Ok(GraphValue::Int(a + other.to_int()?)),
            GraphValue::Float(a) => Ok(GraphValue::Float(a + other.to_float()?)),
            GraphValue::Double(a) => Ok(GraphValue::Double(a + other.to_double()?)),
            GraphValue::Vec2(a1, b1) => {
                let inner_type = a1.as_value_type();
                match other {
                    GraphValue::Int(a2) => match inner_type {
                        GraphValueType::Int => Ok(GraphValue::vec2(
                            GraphValue::Int((a1.unwrap_int() + a2).into()),
                            GraphValue::Int(b1.unwrap_int() + a2),
                        )),
                        GraphValueType::Float => Ok(GraphValue::vec2(
                            GraphValue::Float(a1.to_float().unwrap() + a2 as f32),
                            GraphValue::Float(b1.to_float().unwrap() + a2 as f32),
                        )),
                        other_type => Err(ExecutionError::Conversion(ConversionError::new(
                            other_type,
                            GraphValueType::Float,
                        ))),
                    },
                    GraphValue::Float(a2) => match inner_type {
                        GraphValueType::Int => Ok(GraphValue::vec2(
                            GraphValue::Int(a1.unwrap_int() + a2 as i32),
                            GraphValue::Int(b1.unwrap_int() + a2 as i32),
                        )),
                        GraphValueType::Float => Ok(GraphValue::vec2(
                            GraphValue::Float(a1.to_float().unwrap() as f32 + a2),
                            GraphValue::Float(b1.to_float().unwrap() as f32 + a2),
                        )),
                        other_type => Err(ExecutionError::Conversion(ConversionError::new(
                            other_type,
                            GraphValueType::Float,
                        ))),
                    },
                    _ => todo!(),
                }
            }
            _ => todo!(),
        }
    }

    pub fn sub(self, other: GraphValue) -> Result<GraphValue, ExecutionError> {
        match self {
            GraphValue::Int(a) => Ok(GraphValue::Int(a - other.to_int()?)),
            GraphValue::Float(a) => Ok(GraphValue::Float(a - other.to_float()?)),
            GraphValue::Double(a) => Ok(GraphValue::Double(a - other.to_double()?)),
            GraphValue::Vec2(a1, b1) => {
                let inner_type = a1.as_value_type();
                match other {
                    GraphValue::Int(a2) => match inner_type {
                        GraphValueType::Int => Ok(GraphValue::vec2(
                            a1.unwrap_int() - a2,
                            b1.unwrap_int() - a2,
                        )),
                        GraphValueType::Float => Ok(GraphValue::vec2(
                            a1.to_float().unwrap() - a2 as f32,
                            b1.to_float().unwrap() - a2 as f32,
                        )),
                        other_type => Err(ExecutionError::Conversion(ConversionError::new(
                            other_type,
                            GraphValueType::Float,
                        ))),
                    },
                    GraphValue::Float(a2) => match inner_type {
                        GraphValueType::Int => Ok(GraphValue::vec2(
                            GraphValue::Int(a1.unwrap_int() - a2 as i32),
                            GraphValue::Int(b1.unwrap_int() - a2 as i32),
                        )),
                        GraphValueType::Float => Ok(GraphValue::vec2(
                            GraphValue::Float(a1.to_float().unwrap() - a2),
                            GraphValue::Float(b1.to_float().unwrap() - a2),
                        )),
                        other_type => Err(ExecutionError::Conversion(ConversionError::new(
                            other_type,
                            GraphValueType::Float,
                        ))),
                    },
                    _ => todo!(),
                }
            }
            _ => todo!(),
        }
    }

    pub fn mul(self, other: GraphValue) -> Result<GraphValue, ExecutionError> {
        match self {
            GraphValue::Int(a) => Ok(GraphValue::Int(a * other.to_int()?)),
            GraphValue::Float(a) => Ok(GraphValue::Float(a * other.to_float()?)),
            GraphValue::Double(a) => Ok(GraphValue::Double(a * other.to_double()?)),
            GraphValue::Vec2(a1, b1) => {
                let inner_type = a1.as_value_type();
                match other {
                    GraphValue::Int(a2) => match inner_type {
                        GraphValueType::Int => Ok(GraphValue::Vec2(
                            Box::new(GraphValue::Int(a1.to_int()? * a2)),
                            Box::new(GraphValue::Int(b1.to_int()? * a2)),
                        )),
                        GraphValueType::Float => Ok(GraphValue::vec2(
                            a1.to_int()? * a2,
                            b1.to_int()? * a2,
                        )),
                        other_type => Err(ExecutionError::Conversion(ConversionError::new(
                            other_type,
                            GraphValueType::Float,
                        ))),
                    },
                    GraphValue::Float(a2) => match inner_type {
                        GraphValueType::Int => Ok(GraphValue::vec2(
                            a1.to_float()? * a2,
                            b1.to_float()? * a2,
                        )),
                        GraphValueType::Float => Ok(GraphValue::vec2(
                            a1.to_float()? * a2,
                            b1.to_float()? * a2,
                        )),
                        other_type => Err(ExecutionError::Conversion(ConversionError::new(
                            other_type,
                            GraphValueType::Float,
                        ))),
                    },
                    GraphValue::Double(a2) => match inner_type {
                        GraphValueType::Int => Ok(GraphValue::vec2(
                            a1.to_double()? * a2,
                            b1.to_double()? * a2,
                        )),
                        GraphValueType::Float => Ok(GraphValue::vec2(
                            a1.to_double()? * a2,
                            b1.to_double()? * a2,
                        )),
                        other_type => Err(ExecutionError::Conversion(ConversionError::new(
                            other_type,
                            GraphValueType::Float,
                        ))),
                    },
                    GraphValue::Vec2(a2, b2) => {
                        Ok(GraphValue::vec2(a1.mul(*a2)?, b1.mul(*b2)?))
                    }
                    _ => todo!(),
                }
            }
            _ => todo!(),
        }
    }
}
