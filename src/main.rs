// Deny usage of print and eprint as it won't have same result
// in WASI as if doing in standard program, you must really know
// what you are doing to disable that lint (and you don't know)
#![deny(clippy::print_stdout)]
#![deny(clippy::print_stderr)]

use anyhow::Result;
use lapce_plugin::{
    psp_types::{
        lsp_types::{request::Initialize, DocumentFilter, DocumentSelector, InitializeParams, Url, MessageType},
        Request,
    },
    register_plugin, LapcePlugin, PLUGIN_RPC,
};
use serde_json::Value;

#[derive(Default)]
struct State {}

register_plugin!(State);

// Define the macro rule "volt_config!" which retrieves default config values from volt.toml:
include!(concat!(env!("OUT_DIR"), "/volt_config.rs"));

fn initialize(params: InitializeParams) -> Result<()> {
    let document_selector: DocumentSelector = vec![DocumentFilter {
        language: Some(String::from("php")),
        pattern: Some(String::from("**/*.php")),
        scheme: None,
    }];

    let server_path = params
        .initialization_options
        .as_ref()
        .and_then(|options| options.get("lsp"))
        .and_then(|lsp| lsp.get("serverPath"))
        .and_then(|server_path| server_path.as_str())
        .and_then(|server_path| (!server_path.is_empty()).then_some(server_path))
        .unwrap_or(volt_config!("lsp.serverPath"));

    PLUGIN_RPC.start_lsp(
        Url::parse(&format!("urn:{}", server_path))?,
        vec!["--stdio".to_string()],
        document_selector,
        params.initialization_options,
    );

    Ok(())
}

impl LapcePlugin for State {
    fn handle_request(&mut self, _id: u64, method: String, params: Value) {
        #[allow(clippy::single_match)]
        match method.as_str() {
            Initialize::METHOD => {
                let params: InitializeParams = serde_json::from_value(params).unwrap();
                if let Err(e) = initialize(params) {
                    PLUGIN_RPC.window_show_message(MessageType::ERROR, format!("plugin returned with error: {e}"))
                }
            }
            _ => {}
        }
    }
}
