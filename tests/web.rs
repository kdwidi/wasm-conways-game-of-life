extern crate wasm_game_of_life;
// use wasm_bindgen_test::wasm_bindgen_test;
// use wasm_game_of_life::universe::Universe;

wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

// #[cfg(test)]
// pub fn input_spaceship() -> Universe {
//     let mut universe = Universe::new(6, 6);
//     universe.set_cell_alive(&vec![
//         universe.get_index(1, 2),
//         universe.get_index(2, 3),
//         universe.get_index(3, 1),
//         universe.get_index(3, 2),
//         universe.get_index(3, 3),
//     ]);
//     universe.set_cell_alive(&vec![
//         universe.get_index(2, 1),
//         universe.get_index(2, 3),
//         universe.get_index(3, 2),
//         universe.get_index(3, 3),
//         universe.get_index(4, 2),
//     ]);
//     universe
// }

// #[cfg(test)]
// pub fn expected_spaceship() -> Universe {
//     let mut universe = Universe::new(6, 6);
//     universe.set_cell_alive(&vec![
//         universe.get_index(2, 1),
//         universe.get_index(2, 3),
//         universe.get_index(3, 2),
//         universe.get_index(3, 3),
//         universe.get_index(4, 2),
//     ]);
//     universe
// }

// #[wasm_bindgen_test]
// pub fn test_tick() {
//     let mut input = input_spaceship();
//     let expected = expected_spaceship();
//     input.tick();
//     assert_eq!(input.get_cells(), expected.get_cells());
// }
