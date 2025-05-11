use nu_plugin::SimplePluginCommand;
use nu_protocol::{Signature, SyntaxShape, Type};

use crate::{client::DbusClient, config::DbusClientConfig, DbusSignatureUtilExt};

pub struct Wait;

impl SimplePluginCommand for Wait {
    type Plugin = crate::NuPluginDbus;

    fn name(&self) -> &str {
        "dbus wait"
    }

    fn signature(&self) -> Signature {
        Signature::build(self.name())
            .dbus_command()
            .accepts_dbus_client_options()
            .input_output_type(Type::Nothing, Type::Any)
            .required_named(
                "source",
                SyntaxShape::String,
                "The name of the connection whose signal to wait for",
                None,
            )
            .required(
                "object",
                SyntaxShape::String,
                "The path to the object whose signal to wait for",
            )
            .required(
                "interface",
                SyntaxShape::String,
                "The name of the interface the signal belongs to",
            )
            .required(
                "signal",
                SyntaxShape::String,
                "The name of the signal to wait for",
            )
    }

    fn description(&self) -> &str {
        "Block until a D-Bus signal is received and return it"
    }

    fn search_terms(&self) -> Vec<&str> {
        vec!["dbus", "signal", "wait"]
    }

    fn run(
        &self,
        _plugin: &Self::Plugin,
        _engine: &nu_plugin::EngineInterface,
        call: &nu_plugin::EvaluatedCall,
        _input: &nu_protocol::Value,
    ) -> Result<nu_protocol::Value, nu_protocol::LabeledError> {
        let config = DbusClientConfig::try_from(call)?;
        let dbus = DbusClient::new(config)?;
        dbus.wait(
            &call.get_flag("source")?.unwrap(),
            &call.req(0)?,
            &call.req(1)?,
            &call.req(2)?,
        )
    }
}
