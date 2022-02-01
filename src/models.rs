use crate::layers::ILayer;

pub fn build_model_by_name(model_name: String) -> Box<dyn ILayer> {
    if model_name.eq("model1") {
        model_1::build()
    } else {
        panic!("El modelo no existe");
    }
}

pub mod model_1 {
    use crate::{
        layers::{self, ILayer},
        types::{Dimension, Domain},
    };

    pub fn build() -> Box<dyn ILayer> {

        let broker_agent_states_domain: Domain = Domain(
            "states".to_string(),
            vec![
                Dimension(
                    "MarketState".to_string(),
                    vec!["empty".to_string(), "open".to_string(), "close".to_string()],
                ),
                Dimension(
                    "PositionState".to_string(),
                    vec![
                        "empty".to_string(),
                        "tryopen".to_string(),
                        "open".to_string(),
                        "close".to_string(),
                    ],
                ),
            ],
        );

        let broker_agent_instructions_domain: Domain = Domain(
            "instructions".to_string(),
            vec![Dimension(
                "ConnectionState".to_string(),
                vec![
                    "empty".to_string(),
                    "open_position".to_string(),
                    "close_position".to_string(),
                ],
            )],
        );

        let broker_layer: Box<dyn ILayer> = layers::specific_broker_layer(
            "broker_mock".to_string(),
            broker_agent_instructions_domain,
            broker_agent_states_domain,
        );

        let internal_agent_states_domain: Domain = Domain(
            "states".to_string(),
            vec![
                Dimension(
                    "MarketState".to_string(),
                    vec!["empty".to_string(), "open".to_string(), "close".to_string()],
                ),
                Dimension(
                    "PositionState".to_string(),
                    vec![
                        "empty".to_string(),
                        "tryopen".to_string(),
                        "open".to_string(),
                        "close".to_string(),
                    ],
                ),
            ],
        );
        
        let internal_agent_instructions_domain: Domain = Domain(
            "instructions".to_string(),
            vec![Dimension(
                "ConnectionState".to_string(),
                vec![
                    "empty".to_string(),
                    "open_position".to_string(),
                    "close_position".to_string(),
                ],
            )],
        );

        let input_layer = layers::dr_layer(
            "input_layer".to_string(),
            internal_agent_instructions_domain,
            internal_agent_states_domain,
            vec![broker_layer],
        );
        input_layer
    }
}
