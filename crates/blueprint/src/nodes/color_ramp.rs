use std::rc::Rc;

use bevy_color::Color;
use bevy_utils::hashbrown::HashMap;

use crate::{
    graph::{GraphNode, InputCollection, Node, NodeConstructor, PropertyCollection, PropertyType},
    ExecutionError, Value, ValueType,
};

pub struct ColorRamp;

impl<'a> NodeConstructor<'a> for ColorRamp {
    type Node = Self;

    fn construct() -> Self::Node {
        ColorRamp
    }
}

impl Node for ColorRamp {
    fn get_property_value<'a>(
        &self,
        node: GraphNode,
        property: &'a str,
        inputs: InputCollection,
        attrs: &'a PropertyCollection,
    ) -> Result<Option<Value>, ExecutionError> {
        if property == "color" {
            match attrs.get("breakpoints") {
                None => {
                    return Err(ExecutionError::MissingProperty {
                        node,
                        property: "breakpoints".into(),
                    })
                }
                Some(value) => {
                    let input_value = inputs.get("input").unwrap().clone().to_float()?;
                    let breakpoints = value.clone().to_list()?;
                    let mut greater_than = None;
                    let mut less_than = None;
                    for breakpoint in breakpoints.iter() {
                        let (pos, color) = breakpoint.clone().to_tuple(
                            &Rc::new(ValueType::Float),
                            &Rc::new(ValueType::Vec4(Rc::new(ValueType::Float))),
                        )?;
                        let pos = pos.to_float()?;
                        if input_value <= pos && less_than.is_none() {
                            less_than = Some((pos, color));
                        } else if input_value >= pos {
                            greater_than = Some((pos, color));
                        }
                    }
                    if let Some((less_pos, less_color)) = less_than {
                        let color = if let Some((greater_pos, greater_color)) = greater_than {
                            let g_color = greater_color.to_color()?;
                            let l_color = less_color.to_color()?;

                            let t = (input_value - less_pos) as f32 / (greater_pos - less_pos) as f32;

                            lerp_linear(l_color, g_color, t).to_srgba()
                        } else {
                            less_color.to_color()?.to_srgba()
                        };
                        return Ok(Some(Value::vec4(
                            color.red as f64,
                            color.green as f64,
                            color.blue as f64,
                            color.alpha as f64,
                        )));
                    }
                }
            }
        }
        Ok(None)
    }

    fn available_properties<'a>(&self) -> HashMap<String, PropertyType> {
        let mut map = HashMap::new();
        map.insert(
            "color".into(),
            PropertyType::Output(ValueType::Vec4(Rc::new(ValueType::Float))),
        );
        map.insert("input".into(), PropertyType::Input(ValueType::Float));
        map.insert("b".into(), PropertyType::Input(ValueType::List));
        map
    }
}

fn lerp_linear(color0: Color, color1: Color, t: f32) -> Color {
    let color0 = color0.to_linear();
    let color1 = color1.to_linear();
    let r0 = color0.red;
    let g0 = color0.green;
    let b0 = color0.blue;
    let r1 = color1.red;
    let g1 = color1.green;
    let b1 = color1.blue;

    Color::linear_rgb(r0 + t * (r1 - r0), g0 + t * (g1 - g0), b0 + t * (b1 - b0))
}
