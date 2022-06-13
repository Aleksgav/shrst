use super::room::*;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub struct SmartHome {
    pub desc: String,
    pub rooms: HashMap<String, Room>,
}

impl SmartHome {
    pub fn new(name: String) -> SmartHome {
        SmartHome {
            desc: name,
            rooms: HashMap::new(),
        }
    }

    pub fn add_room(&mut self, name: String, room: Room) {
        self.rooms.insert(name, room);
    }

    pub fn list_rooms(&self) -> Vec<&Room> {
        let mut res = Vec::new();

        for room in self.rooms.values() {
            res.push(room)
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let desc = String::from("some desc");
        let home = SmartHome::new(desc.clone());

        assert_eq!(home.desc, desc);
        assert_eq!(home.rooms, HashMap::new());
    }

    #[test]
    fn add_room() {
        let mut home = SmartHome::new(String::from("some desc"));

        assert_eq!(home.rooms.len(), 0);

        let room_id = String::from("room");
        let room = Room::new(String::from("room desc"));
        home.add_room(room_id.clone(), room.clone());

        let another_room_id = String::from("another room");
        let another_room = Room::new(String::from("another room desc"));
        home.add_room(another_room_id.clone(), another_room.clone());

        assert_eq!(
            home.rooms,
            HashMap::from([(room_id, room), (another_room_id, another_room)])
        );
    }

    #[test]
    fn list_rooms() {
        let mut home = SmartHome::new(String::from("some desc"));

        let room = Room::new(String::from("room desc"));
        let another_room = Room::new(String::from("another room desc"));

        home.add_room(String::from("room"), room.clone());
        home.add_room(String::from("another room"), another_room.clone());

        let mut rooms = home.list_rooms();
        rooms.sort_by(|a, b| a.desc.cmp(&b.desc));
        assert_eq!(rooms, vec![&another_room, &room]);
    }
}
