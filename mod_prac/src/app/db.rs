//crate = 根目錄（src）
use crate::app::models::Dog;

//super = 當前所在資料夾
// use super::models::Dog;

//表示models底下的全部都要使用
// use crate::app::models::*;

//可以取縮寫
// use crate::app::models::Dog as D;
// pub fn save_dog (dog: &D) {
//     println!("{} is saved in db.", dog.name );
// }

pub fn save_dog (dog: &Dog) {
    println!("{} is saved in db.", dog.name );
}

//以上等於以下
// pub fn save_dog (dog: &crate::models::Dog) {
//     println!("{} is saved in db.", dog.name );
// }
