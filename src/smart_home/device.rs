#[derive(Clone, Debug, PartialEq)]
pub struct SmartDevice {
    pub desc: String,
}

impl SmartDevice {
    pub fn new(desc: String) -> SmartDevice {
        SmartDevice { desc }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let desc = String::from("some desc");
        let device = SmartDevice::new(desc.clone());

        assert_eq!(device.desc, desc);
    }
}
