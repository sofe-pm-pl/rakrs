// rakrs
// Copyright (C) SOFe
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[allow(unused_imports)]
use crate::prelude::*;

use std::io::{Read, Result, Write};

pub use can_io::{CanIo, Little};

dirmod!();

macro_rules! packets {
    ($($mod:ident $name:ident $id:literal;)*) => {
        pub mod packets {
            $(pub use super::$mod::$name;)*
        }

        #[repr(u8)]
        pub enum Packet { $($name($mod::$name) = $id),* }

        impl Packet {
            pub fn write<W: Write>(&self, mut w: W) -> Result<()> {
                match self {
                    $(
                        Packet::$name(var) => {
                            w.write_all(&[$id])?;
                            var.write(w)
                        },
                    )*
                }
            }

            pub fn read<R: Read>(mut r: R) -> Result<Option<Packet>> {
                let mut id = [0u8];
                r.read_exact(&mut id)?;
                match id[0] {
                    $(
                        $id => Ok(Some(Packet::$name($mod::$name::read(r)?))),
                    )*
                    _ => Ok(None),
                }
            }
        }
    };
}

// Required methods on packets:
// fn read<R: Read>(r: R) -> Result<Self>;
// fn write<W: Write>(&self, w: W) -> Result<()>;

packets! [
    ack Ack 0xc0;
    ack Nack 0xa0;
    advertise_system AdvertiseSystem 0x1d;
    connected_ping ConnectedPing 0x00;
    connected_pong ConnectedPong 0x03;
    connection_request ConnectionRequest 0x09;
    connection_request_accepted ConnectionRequestAccepted 0x10;
    disconnection_notification DisconnectionNotification 0x15;
    incompatible_protocol_version IncompatibleProtocolVersion 0x19;
    new_incoming_connection NewIncomingConnection 0x13;
    open_connection_request_1 OpenConnectionRequest1 0x05;
    open_connection_reply_1 OpenConnectionReply1 0x06;
    open_connection_request_2 OpenConnectionRequest2 0x07;
    open_connection_reply_2 OpenConnectionReply2 0x08;
    unconnected_ping UnconnectedPing 0x01;
    unconnected_ping_open_connections UnconnectedPingOpenConnections 0x02;
    unconnected_pong UnconnectedPong 0x1c;
];
