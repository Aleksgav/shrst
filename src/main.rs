mod smart_home;

use smart_home::device::*;
use smart_home::home::*;
use smart_home::report::*;
use smart_home::room::*;
use std::io::{self, BufWriter};

fn main() -> Result<(), io::Error> {
    let thermometer = SmartDevice::new(String::from("Very precision thermometer"));
    let humidifier = SmartDevice::new(String::from("Best ever humidifier"));
    let purifier = SmartDevice::new(String::from("Cleaning air is my job"));

    let mut kitchen = Room::new(String::from("Kitchen, when I cook my food!"));
    kitchen.add_device(String::from("thermometer"), thermometer);

    let mut living_room = Room::new(String::from("Living room for all my homies!"));
    living_room.add_device(String::from("humidifier"), humidifier);
    living_room.add_device(String::from("purifier"), purifier);

    let mut smart_house = SmartHome::new(String::from("Very smart house"));
    smart_house.add_room(String::from("living room"), living_room);
    smart_house.add_room(String::from("kitchen"), kitchen);

    println!();
    println!("Display house rooms information:");
    let rooms = smart_house.list_rooms();
    for room in rooms {
        println!("\troom: {}", room.desc);
    }

    let report = get_report(smart_house);

    let handle = io::stdout();
    let mut std_out = BufWriter::new(handle);

    report.print(&mut std_out)?;

    println!();
    println!("Now we are searching by id's:");
    let room_id = String::from("kitchen");
    let device_id = String::from("thermometer");
    println!(
        "search by room id: {}, and device id: {}",
        room_id, device_id
    );
    let device_state = report.get_state_by_id(room_id, device_id);
    let res = match device_state {
        Some(state) => state,
        None => String::from("nothing found"),
    };
    println!("result: {}", res);

    println!();

    println!("Trying to search by wrong device id:");
    let room_id = String::from("kitchen");
    let device_id = String::from("wrong device");
    println!(
        "search by room id: {}, and device id: {}",
        room_id, device_id
    );

    let device_state = report.get_state_by_id(room_id, device_id);
    let res = match device_state {
        Some(state) => state,
        None => String::from("nothing found"),
    };
    println!("result: {}", res);
    println!();

    Ok(())
}
