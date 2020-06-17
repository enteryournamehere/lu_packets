use std::io::Result as Res;

use endio::{Deserialize, LERead};
use endio::LittleEndian as LE;
use lu_packets_derive::ServiceMessage;

use crate::common::ServiceId;

#[derive(Debug, ServiceMessage)]
#[repr(u32)]
pub enum GeneralMessage {
	Handshake(Handshake)
}

#[derive(Debug)]
pub struct Handshake {
	pub network_version: u32,
	pub service_id: ServiceId,
}

impl<R: LERead> Deserialize<LE, R> for Handshake
	where    u8: Deserialize<LE, R>,
	        u16: Deserialize<LE, R>,
	  ServiceId: Deserialize<LE, R>,
	        u32: Deserialize<LE, R> {
	fn deserialize(reader: &mut R) -> Res<Self> {
		let network_version = reader.read()?;
		let _: u32          = reader.read()?;
		let service_id      = reader.read()?;
		let _: u16          = reader.read()?;
		Ok(Self { network_version, service_id })
	}
}
