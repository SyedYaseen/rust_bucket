// mod guess;
mod collections;
mod enums;
mod modul;
mod ownership;
mod structs;
mod errHandl;
// use guess::guess;
// use ownership::ownership;
// use enums::enums;
// use structs::strcuts;
// use modul::modul::Breakfast;
// use varmut::varmut;
// use collections::collections;
use errHandl::err_handl;
fn main() {
    // ownership();
    // guess();
    // varmut();
    // strcuts();
    // enums();

    //Modul
    // let mut summer_bf = Breakfast::summer_menu();
    // summer_bf.food = String::from("Pudding");
    // summer_bf.update_drink();

    // println!("{:#?}", summer_bf);
    // collections();
    err_handl();
}
