use std::ops::{Index, IndexMut};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    NorthToSouth,
    WestToEast,
    SouthToNorth,
    EastToWest,
}

impl Direction {
    pub const ALL: [Direction; 4] = [Direction::NorthToSouth, Direction::WestToEast, Direction::SouthToNorth, Direction::EastToWest];
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Index2D(pub usize, pub usize);

impl From<(usize, usize)> for Index2D {
    fn from(tpl: (usize, usize)) -> Self {
        Index2D(tpl.0, tpl.1)
    }
}

#[derive(Debug, Clone)]
pub struct Linear2DArray<T> {
    storage: Vec<T>,
    pub width: usize,
    pub height: usize,
}

impl<T> Linear2DArray<T> {
    pub fn new(storage: Vec<T>, width: usize) -> Self {
        let height = storage.len() / width;
        assert_eq!(storage.len() % width, 0);
        Self { storage, width, height }
    }

    pub fn dimensions(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.storage.iter()
    }

    pub fn sweep<State, OnElement>(&mut self, state: State, dir: Direction, on_element: OnElement)
        where
            State: Clone,
            OnElement: Fn(&mut State, Index2D, &mut T) -> bool
    {

        let height = self.height as i32;
        let width = self.width as i32;
        let (mut x, mut y, delta_x, delta_y, delta_x2, delta_y2) = match dir {
            Direction::NorthToSouth => (0, 0, 0, 1, 1, -height),
            Direction::WestToEast => (0, 0, 1, 0, -width, 1),
            Direction::SouthToNorth => (0, height  - 1, 0, - 1, 1, height),
            Direction::EastToWest => (width  - 1, 0, -1, 0, width, 1)
        };

        while x >= 0 && x < width && y >= 0 &&  y < height {
            let mut state = state.clone();
            while x >= 0 && x < width && y >= 0 &&  y < height {
                let idx = Index2D(x as usize, y as usize);
                let value = &mut self[idx];
                on_element(&mut state, idx, value);
                x += delta_x;
                y += delta_y;
            }

            x += delta_x2;
            y += delta_y2;
        }
    }
}

impl<T> Index<Index2D> for Linear2DArray<T> {
    type Output = T;

    fn index(&self, index: Index2D) -> &Self::Output {
        let Index2D(x, y) = index;
        assert!(x < self.width);
        assert!(y < self.height);
        &self.storage[x + y * self.width]
    }
}

impl<T> IndexMut<Index2D> for Linear2DArray<T> {
    fn index_mut(&mut self, index: Index2D) -> &mut Self::Output {
        let Index2D(x, y) = index;
        assert!(x < self.width);
        assert!(y < self.height);
        &mut self.storage[x + y * self.width]
    }
}
