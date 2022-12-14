use std::hash::{Hash, Hasher};
use std::ops::{Index, IndexMut};

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum Direction {
    NorthToSouth,
    WestToEast,
    SouthToNorth,
    EastToWest,
}

pub static ALL: [Direction; 4] = [Direction::NorthToSouth, Direction::WestToEast, Direction::SouthToNorth, Direction::EastToWest];

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Index2D(pub usize, pub usize);

impl Hash for Index2D {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_usize(self.0);
        state.write_usize(self.1);
    }
}

impl Index2D {
    pub fn max_distance(self, other: Self) -> usize {
        ((self.0 as i64 - other.0 as i64).abs() as usize) + ((self.1 as i64 - other.1 as i64).abs() as usize)
    }

    pub fn step(self, dir: Direction) -> Self {
        let Self(x, y) = self;
        match dir {
            Direction::NorthToSouth => Self(x, y + 1),
            Direction::WestToEast => Self(x + 1, y),
            Direction::SouthToNorth => Self(x, y - 1),
            Direction::EastToWest => Self(x - 1, y)
        }
    }
}

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

    pub fn truncate_height(&mut self, new_height: usize) {
        assert!(new_height <= self.height);

        self.storage.truncate(new_height * self.width);
        self.height = new_height;
    }

    pub fn indices(&self) -> impl Iterator<Item=Index2D> {
        struct Iter {
            count: usize,
            width: usize,
            height: usize
        }

        impl Iterator for Iter {
            type Item = Index2D;

            fn next(&mut self) -> Option<Self::Item> {
                if self.count >= self.width * self.height {
                    None
                } else {
                    let count = self.count;
                    self.count += 1;
                    Some(Index2D(count % self.width, count / self.width))
                }
            }
        }

        Iter {
            count: 0,
            width: self.width,
            height: self.height
        }
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
            Direction::SouthToNorth => (0, height - 1, 0, -1, 1, height),
            Direction::EastToWest => (width - 1, 0, -1, 0, width, 1)
        };

        while x >= 0 && x < width && y >= 0 && y < height {
            let mut state = state.clone();
            while x >= 0 && x < width && y >= 0 && y < height {
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
        &self.storage[x + y * self.width]
    }
}

impl<T> IndexMut<Index2D> for Linear2DArray<T> {
    fn index_mut(&mut self, index: Index2D) -> &mut Self::Output {
        let Index2D(x, y) = index;
        &mut self.storage[x + y * self.width]
    }
}
