mod smart_home;

use smart_home::device::*;
use smart_home::home::*;
use smart_home::report::*;
use std::io::{self, BufWriter};

// Вопрос: Что он должен из себя представлять?
// Ответ: Достаточно, чтобы этот источник информации умел предоставлять состояние устройства по некоторому идентификатору.

// Вопрос: Какой уникальный идентификатор устройства знает дом?
// Ответ: Сочетание двух строк: имени комнаты и имени устройства.

// Вопрос: Как нам выразить эти идеи на языке программирования?
// Ответ:
// 1) Нам надо описать сущность, которая умеет принимать идентификатор (пару строк) и возвращать состояние соответствующего устройства.
// Для этого нам не обязательно фиксировать какой-то конкретный тип. Достаточно описать интерфейс.
// В раст это делается с помощью трейтов.
// Значит, нам нужен трейт, с одним методом: принимающим пару строк, и возвращающий строку - состояние устройства.
//
// 2) Нам нужно принимать в метод построения отчёта любой тип, реализующий наш трейт.
// В Rust это описывается с помощью generic типов и trait bounds для них.
// Значит, метод построения отчёта о доме должен принимать источник информации об устройствах в виде generic типа, реализующего наш трейт из пункта 1.
// Теперь пользователь может определять свои типы источников информации об устройствах.
// Реализовывать для них трейт из библиотеки. И передавать их в функцию построения отчёта.

fn main() -> Result<(), io::Error> {
    let thermometer = SmartDevice::new(String::from("Very precision thermometer"));
    let humidifier = SmartDevice::new(String::from("Best ever humidifier"));
    let purifier = SmartDevice::new(String::from("Cleaning air is my job"));

    let mut kitchen = Room::new(String::from("Kitchen, when I cook my food!"));
    let mut living_room = Room::new(String::from("Living room for all my homies!"));

    let mut smart_house = SmartHouse::new(String::from("Very smart house"));

    kitchen.add_device(String::from("thermometer"), thermometer);

    living_room.add_device(String::from("humidifier"), humidifier);
    living_room.add_device(String::from("purifier"), purifier);

    smart_house.add_room(String::from("living room"), living_room);
    smart_house.add_room(String::from("kitchen"), kitchen);

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

    Ok(())
}
