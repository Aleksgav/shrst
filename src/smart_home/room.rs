use super::device::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new() {
        let room_desc = String::from("room desc");
        let room = Room::new(room_desc.clone());

        assert_eq!(room.desc, room_desc);
        assert_eq!(room.devices, HashMap::new());
    }

    #[test]
    fn add_device() {
        let mut room = Room::new(String::from("some room"));

        assert_eq!(room.devices, HashMap::new());

        let device_id = String::from("some device id");
        let device = SmartDevice::new(String::from("some device"));
        room.add_device(device_id.clone(), device.clone());

        let another_device_id = String::from("some another device id");
        let another_device = SmartDevice::new(String::from("some another device"));
        room.add_device(another_device_id.clone(), another_device.clone());

        assert_eq!(
            room.devices,
            HashMap::from([(device_id, device), (another_device_id, another_device)])
        )
    }

    #[test]
    fn list_devices() {
        let mut room = Room::new(String::from("some room"));

        let device_id = String::from("some device id");
        let device = SmartDevice::new(String::from("some device"));
        room.add_device(device_id.clone(), device.clone());

        let another_device_id = String::from("some another device id");
        let another_device = SmartDevice::new(String::from("some another device"));
        room.add_device(another_device_id.clone(), another_device.clone());

        let mut devices = room.list_devices();
        devices.sort_by(|a, b| a.desc.cmp(&b.desc));
        assert_eq!(devices, vec![&another_device, &device]);
    }
}
