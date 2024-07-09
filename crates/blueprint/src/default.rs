use crate::graph::*;
use crate::{nodes, Value};
use crate::ExecutionError;
use std::any::type_name;

impl crate::Blueprint {
    pub fn simple() -> Result<crate::Blueprint, ExecutionError> {
        let mut blueprint = crate::Blueprint::with_default().unwrap();

        let coord_node = GraphNode::default();
        blueprint.add_graph_node(type_name::<nodes::Coord>(), coord_node)?;

        let uv_scale_node = GraphNode::default();
        blueprint.add_graph_node(type_name::<nodes::Math>(), uv_scale_node)?;
        blueprint.insert_attribute(uv_scale_node, "operation", "*");
        blueprint.insert_attribute(uv_scale_node, "b", 0.05);

        let perlin_node = GraphNode::default();
        blueprint.add_graph_node(type_name::<nodes::Noise>(), perlin_node)?;
        blueprint.insert_attribute(
            uv_scale_node,
            "noise_function",
            Value::String("perlin".into()),
        );

        let noise_scale_node = GraphNode::default();
        blueprint.add_graph_node(type_name::<nodes::Math>(), noise_scale_node)?;
        blueprint.insert_attribute(noise_scale_node, "operation", "*");
        blueprint.insert_attribute(noise_scale_node, "a", 6.0);

        let color_scale_node = GraphNode::default();
        blueprint.add_graph_node(type_name::<nodes::Math>(), color_scale_node)?;
        blueprint.insert_attribute(color_scale_node, "operation", "+");
        blueprint.insert_attribute(color_scale_node, "a", 1.0);

        let color_ramp_node = GraphNode::default();
        blueprint.add_graph_node(type_name::<nodes::ColorRamp>(), color_ramp_node)?;
        blueprint.insert_attribute(
            color_ramp_node,
            "breakpoints",
            Value::List(vec![
                Value::Tuple(
                    Box::new(Value::Float(0.0)),
                    Box::new(Value::vec4(1.0 / 255.0 * 30.0, 1.0 / 255.0 * 64.0, 1.0 / 255.0 * 175.0, 1.0)),//rgb(30, 64, 175)
                ),
                Value::Tuple(
                    Box::new(Value::Float(0.25)),
                    Box::new(Value::vec4(1.0 / 255.0 * 96.0, 1.0 / 255.0 * 165.0, 1.0 / 255.0 * 250.0, 1.0)),
                ),
                Value::Tuple(
                    Box::new(Value::Float(0.5)),
                    Box::new(Value::vec4(1.0 / 255.0 * 254.0, 1.0 / 255.0 * 240.0, 1.0 / 255.0 * 138.0, 1.0)),//rgb(254, 240, 138)
                ),
                Value::Tuple(
                    Box::new(Value::Float(1.0)),
                    Box::new(Value::vec4(1.0 / 255.0 * 53.0, 1.0 / 255.0 * 230.0, 1.0 / 255.0 * 53.0, 1.0)),//rgb(163, 230, 53)
                ),
                Value::Tuple(
                    Box::new(Value::Float(2.0)),
                    Box::new(Value::vec4(1.0 / 255.0 * 245.0, 1.0 / 255.0 * 245.0, 1.0 / 255.0 * 244.0, 1.0)),//rgb(245, 245, 244)
                ),
            ]),
        );

        let cell_node = GraphNode::default();
        blueprint.add_graph_node(type_name::<nodes::Cell>(), cell_node)?;

        blueprint.add_graph_edge(
            coord_node,
            uv_scale_node,
            GraphEdge {
                inputs: vec!["a".into()],
                outputs: vec!["axial".into()],
            },
        );
        blueprint.add_graph_edge(
            uv_scale_node,
            perlin_node,
            GraphEdge {
                inputs: vec!["uv".into()],
                outputs: vec!["value".into()],
            },
        );
        blueprint.add_graph_edge(
            perlin_node,
            noise_scale_node,
            GraphEdge {
                inputs: vec!["b".into()],
                outputs: vec!["value".into()],
            },
        );
        blueprint.add_graph_edge(
            noise_scale_node,
            cell_node,
            GraphEdge {
                inputs: vec!["elevation".into()],
                outputs: vec!["value".into()],
            },
        );

        blueprint.add_graph_edge(
            perlin_node,
            color_scale_node,
            GraphEdge {
                inputs: vec!["b".into()],
                outputs: vec!["value".into()],
            },
        );
        blueprint.add_graph_edge(
            color_scale_node,
            color_ramp_node,
            GraphEdge {
                inputs: vec!["input".into()],
                outputs: vec!["value".into()],
            },
        );
        blueprint.add_graph_edge(
            color_ramp_node,
            cell_node,
            GraphEdge {
                inputs: vec!["color".into()],
                outputs: vec!["color".into()],
            },
        );
        Ok(blueprint)
    }
}