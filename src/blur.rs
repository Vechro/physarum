// fn blur(radius: u8, src: &TrailMap) -> TrailMap {
//     let mut dest: TrailMap = Default::default();

//     // Blur horizontally
//     for row in 0..HEIGHT {
//         for column in 0..WIDTH {
//             let left_index = (row - radius as u16).max(0);
//             let right_index = (row + radius as u16).min(WIDTH);

//             let mut sum: u16 = 0;
//             let mut count: u8 = 0;

//             for neighbor in left_index..right_index {
//                 let cell = src.get(neighbor as usize, column as usize);
//                 sum += cell as u16;
//                 count += 1;
//             }

//             dest.set(row as usize, column as usize, (sum / count as u16) as u8)
//         }
//     }

//     // Blur vertically
//     for row in 0..HEIGHT {
//         for column in 0..WIDTH {
//             let top_index = (column - radius as u16).max(0);
//             let bottom_index = (column + radius as u16).min(HEIGHT);

//             let mut sum: u16 = 0;
//             let mut count: u8 = 0;

//             for neighbor in top_index..bottom_index {
//                 let cell = src.get(row as usize, neighbor as usize);
//                 sum += cell as u16;
//                 count += 1;
//             }

//             dest.set(row as usize, column as usize, (sum / count as u16) as u8)
//         }
//     }

//     dest
// }
