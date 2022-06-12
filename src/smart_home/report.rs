use std::io::{self, Write};

use super::home::*;

pub trait SmartHomeReport {
    fn print<W: Write>(&self, writer: &mut W) -> Result<(), io::Error>;
    fn get_state_by_id(&self, room_name: String, device_name: String) -> Option<String>;
}

pub struct Report {
    pub home: SmartHome,
}

impl Report {
    pub fn new(home: SmartHome) -> Self {
        Self { home }
    }
}

impl SmartHomeReport for Report {
    fn get_state_by_id(&self, room_name: String, device_name: String) -> Option<String> {
        self.home.rooms.get(&room_name).and_then(|room| {
            room.devices
                .get(&device_name)
                .map(|device| device.desc.to_owned())
        })
    }

    fn print<W: Write>(&self, writer: &mut W) -> Result<(), io::Error> {
        writer.write_all(b"Smart house information\n")?;
        writer.write_all(b"House: ")?;
        writer.write_all(self.home.desc.as_bytes())?;
        writer.write_all(b"\n")?;

        for (room_name, room) in &self.home.rooms {
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

pub fn get_report(home: SmartHome) -> impl SmartHomeReport {
    Report::new(home)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{Room, SmartDevice};
    use std::io::BufWriter;
    use std::str;

    #[test]
    fn new() {
        let home = SmartHome::new(String::from("some smart home"));
        let report = Report::new(home.clone());

        assert_eq!(report.home, home);
    }

    #[test]
    fn get_state_by_id() {
        let mut room = Room::new(String::from("some room"));

        let device_desc = String::from("some device");
        let device = SmartDevice::new(device_desc.clone());
        let device_id = String::from("some device");
        room.add_device(device_id.clone(), device);

        let mut home = SmartHome::new(String::from("home"));
        let room_id = String::from("room");
        home.add_room(room_id.clone(), room);
        let report = Report::new(home);

        let device_state = report.get_state_by_id(room_id.clone(), device_id);
        assert_eq!(device_state, Some(device_desc));

        let wrong_device_state = report.get_state_by_id(room_id, String::from("wrong device id"));
        assert_eq!(wrong_device_state, None);
    }

    #[test]
    fn print() {
        let mut room = Room::new(String::from("some room"));

        let device_desc = String::from("some device");
        let device = SmartDevice::new(device_desc.clone());
        let device_id = String::from("some device");
        room.add_device(device_id.clone(), device);

        let mut home = SmartHome::new(String::from("home"));
        let room_id = String::from("room");
        home.add_room(room_id.clone(), room);

        let report = Report::new(home);

        let mut buffer = BufWriter::new(Vec::new());
        report.print(&mut buffer).unwrap();
        let result = str::from_utf8(buffer.buffer()).unwrap();

        assert_eq!(result.contains(&device_desc), true);
    }
}
