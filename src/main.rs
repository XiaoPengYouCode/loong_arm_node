use dora_node_api::{
    arrow::array::{BooleanArray, Float64Array},
    dora_core::config::DataId,
    DoraNode, Event, MetadataParameters, Parameter,
};
use openloong_sdk_basic_usage_example_rust::sdk::LoongManiSdk;
use std::error::Error;
use tracing::{error, info, Level};

pub mod utils;
use utils::MetadataExt;

fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let (mut node, mut events) = DoraNode::init_from_env()?;

    let mut loong_mani_sdk = LoongManiSdk::default();

    while let Some(event) = events.recv() {
        match event {
            Event::Input { id, metadata, data } => {
                if id.to_string() == "pose_r" || id.to_string() == "pose_l" {
                    let encoding = metadata.get_string("encoding", "xyzrpy");
                    let wait = metadata.get_bool("bool", true);
                    let _duration = metadata.get_float("duration", 1.0);
                    let _grasp = metadata.get_bool("grasp", true);
                    let arm = metadata.get_string("arm", "right");
                    let values = data
                        .as_any()
                        .downcast_ref::<Float64Array>()
                        .ok_or("Excepted Float64Array")?
                        .values()
                        .to_vec();

                    if !wait {
                        if encoding == "xyzrpys" {
                            if arm == "right" || arm == "left" {
                                let outputid = if arm == "right" {
                                    DataId::from("response_r_arm".to_string())
                                } else {
                                    DataId::from("response_l_arm".to_string())
                                };
                                match loong_mani_sdk.handle_xyzrpy(arm.as_str(), values) {
                                    Ok(()) => {
                                        node.send_output(
                                            outputid,
                                            MetadataParameters::new(),
                                            BooleanArray::from(vec![true]),
                                        )
                                        .unwrap();
                                        loong_mani_sdk.send().unwrap();
                                    }
                                    Err(e) => {
                                        let mut metadata = MetadataParameters::new();
                                        metadata.insert(
                                            "error".to_string(),
                                            Parameter::String(
                                                format!("error: {e}. Failed to grasp").to_string(),
                                            ),
                                        );
                                        node.send_output(
                                            outputid,
                                            metadata,
                                            BooleanArray::from(vec![false]),
                                        )
                                        .unwrap();
                                    }
                                };
                            } else {
                                error!("Unsupported arm: {}", arm);
                            }
                        } else if encoding == "jointstate" {
                            // loong_mani_sdk.handle_jointstate(arm, values)?;
                            todo!("impl joint ctrl data handle in loong sdk");
                        }
                    } else {
                        info!("waiting!");
                    }
                } else {
                    continue;
                }
            }
            _ => {}
        }
    }

    Ok(())
}
