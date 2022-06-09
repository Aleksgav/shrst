use super::device::*;
use std::collections::HashMap;

pub struct Room {
    pub desc: String,
    pub devices: HashMap<String, SmartDevice>,
}

impl Room {
    pub fn new(name: String) -> Room {
        Room {
            desc: name,
            devices: HashMap::new(),
        }
    }

    pub fn add_device(&mut self, name: String, device: SmartDevice) {
        self.devices.insert(name, device);
    }

    #[allow(dead_code)]
    pub fn list_devices(&self) -> Vec<&SmartDevice> {
        let mut res = Vec::new();

        for device in self.devices.values() {
            res.push(device)
        }

        res
    }
}

pub struct SmartHouse {
    pub desc: String,
    pub rooms: HashMap<String, Room>,
}

impl SmartHouse {
    pub fn new(name: String) -> SmartHouse {
        SmartHouse {
            desc: name,
            rooms: HashMap::new(),
        }
    }

    pub fn add_room(&mut self, name: String, room: Room) {
        self.rooms.insert(name, room);
    }

    #[allow(dead_code)]
    pub fn list_rooms(&self) -> Vec<&Room> {
        let mut res = Vec::new();

        for room in self.rooms.values() {
            res.push(room)
        }

        res
    }
}
