// There are two types of image
// 4bpp and 8bpp

use core::ops::{Index, IndexMut};

// for now lets ignore 4bpp tiles
struct DoubleTile {
    data: [[u8; 8]; 8];
}

/// You should be able to set a tile like
/// tiles[0] = Tile::Default();
struct TileData<T>{
    foo: T,
}

impl<T> Index<usize> for TileData<T> {
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        unimplemented!();
    }
}

impl<T> IndexMut<usize> for TileData<T> {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Self::Output {
        // I need to make sure that this write is a 16bit write.
        //
        // This will probably be fastest as a DMA request.
        unimplemented!();
    }
}

// let tile = Tile::Default();
// for x in 0..8 {
//     for y in 0..8 {
//         if (x < 4 && y >= 4)  || (x >= 4 && y < 4) {
//             tile[y][x] = 0;
//         }else {
//             tile[y][x] = 1;
//         }
//     }
// }
// 
// let tile_data = ...;
// tile_data[1] = tile;
