use scuffle_amf0::{Amf0Decoder, Amf0Value};

use super::NetConnectionCommand;
use crate::command_messages::errors::CommandError;

impl<'a> NetConnectionCommand<'a> {
    pub fn read(command_name: &str, decoder: &mut Amf0Decoder<'a>) -> Result<Option<Self>, CommandError> {
        match command_name {
            "connect" => {
                let Amf0Value::Object(command_object) = decoder.decode_with_type(scuffle_amf0::Amf0Marker::Object)? else {
                    unreachable!();
                };

                let (_, Amf0Value::String(app)) = command_object
                    .into_owned() // we have to get ownership here because we have to own the inner Cows
                    .into_iter()
                    .find(|(k, _)| k == "app")
                    .ok_or(CommandError::NoAppName)?
                else {
                    return Err(CommandError::NoAppName);
                };

                Ok(Some(Self::Connect { app }))
            }
            "call" => Ok(Some(Self::Call)),
            "close" => Ok(Some(Self::Close)),
            "createStream" => Ok(Some(Self::CreateStream)),
            _ => Ok(None),
        }
    }
}
