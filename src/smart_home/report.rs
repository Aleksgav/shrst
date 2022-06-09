use std::io::{self, Write};

use super::home::*;

pub trait SmartHouseReport {
    fn print<W: Write>(&self, writer: &mut W) -> Result<(), io::Error>;
    fn get_state_by_id(&self, room_name: String, device_name: String) -> Option<String>;
}

pub struct Report {
    house: SmartHouse,
}

impl Report {
    pub fn new(house: SmartHouse) -> Self {
        Self { house }
    }
}

impl SmartHouseReport for Report {
    fn get_state_by_id(&self, room_name: String, device_name: String) -> Option<String> {
        self.house.rooms.get(&room_name).and_then(|room| {
            room.devices
                .get(&device_name)
                .map(|device| device.desc.to_owned())
        })
    }

    fn print<W: Write>(&self, writer: &mut W) -> Result<(), io::Error> {
        writer.write_all(b"Smart house information\n")?;
        writer.write_all(b"House: ")?;
        writer.write_all(self.house.desc.as_bytes())?;
        writer.write_all(b"\n")?;

        for (room_name, room) in &self.house.rooms {
            writer.write_all(b"\troom: ")?;
            writer.write_all(room_name.as_bytes())?;
            writer.write_all(b"\n")?;

            if room.devices.is_empty() {
                writer.write_all(b"\tdevices not found\n")?;

                continue;
            }

            writer.write_all(b"\tdevices: \n")?;

            for (device_name, device) in &room.devices {
                writer.write_all(b"\t\t")?;
                writer.write_all(device_name.as_bytes())?;
                writer.write_all(b"\n")?;
                writer.write_all(b"\t\t")?;
                writer.write_all(b"device info: ")?;
                writer.write_all(device.desc.as_bytes())?;
                writer.write_all(b"\n")?;
            }

            writer.write_all(b"\n")?;
        }

        Ok(())
    }
}

pub fn get_report(house: SmartHouse) -> impl SmartHouseReport {
    Report::new(house)
}
