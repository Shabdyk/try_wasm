mod tictactoe;

use std::cell::RefCell;
use tictactoe::*;
use wasm_bindgen::prelude::*;

thread_local! {
  static TICTACTOE: RefCell<Tictactoe>
    = RefCell::new(Tictactoe::new(10, 10));
}

#[wasm_bindgen(js_name = getState)]
pub fn get_state() -> String {
    TICTACTOE.with(|ttt| ttt.borrow().to_string())
}

#[wasm_bindgen(js_name = playX)]
pub fn play_x(x: usize, y: usize) -> bool {
    TICTACTOE.with(|ttt| {
        ttt.borrow_mut().select('x', (x, y));
        let chkx = ttt.borrow_mut().x_fields.clone();
        ttt.borrow_mut().win(chkx)
    })
}

#[wasm_bindgen(js_name = playO)]
pub fn play_o(x: usize, y: usize) -> bool {
    TICTACTOE.with(|ttt| {
        ttt.borrow_mut().select('o', (x, y));
        let chko = ttt.borrow_mut().o_fields.clone();
        ttt.borrow_mut().win(chko)
    })
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

// #[wasm_bindgen]
// pub fn greet(name: &str) {
//     alert(&format!("Hello, {}!", name));
// }

//
// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
