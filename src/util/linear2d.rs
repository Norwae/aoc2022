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

    pub fn sweep_by
    <State, IndexMutator1, IndexMutator2, LineInit, OnElement>
    (&mut self, state: &mut State, mut index: Index2D, index_increment: IndexMutator1,
     line_increment: IndexMutator2, line_init: LineInit, on_element: OnElement,
    )
        where IndexMutator1: Fn(&mut Index2D),
              IndexMutator2: Fn(&mut Index2D),
              LineInit: Fn(&mut State) -> bool,
              OnElement: Fn(&mut State, Index2D, &mut T) -> bool
    {
        while index.0 < self.width && index.1 < self.height {
            if !line_init(state) {
                break;
            }
            while index.0 < self.width && index.1 < self.height {
                if !on_element(state, index.clone(), &mut self[index]) {
                    break;
                }
                index_increment(&mut index)
            }

            line_increment(&mut index)
        }
    }

    pub fn sweep<State, LineInit, OnElement>(&mut self, state: &mut State, dir: Direction, line_init: LineInit, on_element: OnElement)
        where
            LineInit: Fn(&mut State) -> bool,
            OnElement: Fn(&mut State, Index2D, &mut T) -> bool {
        let height = self.height;
        let width = self.width;
        match dir {
            Direction::NorthToSouth => self.sweep_by(
                state,
                (0, 0).into(),
                |idx| idx.1 += 1,
                |idx| *idx = Index2D(idx.0 + 1, 0),
                line_init,
                on_element,
            ),
            Direction::WestToEast => self.sweep_by(
                state,
                (0, 0).into(),
                |idx| idx.0 += 1,
                |idx| *idx = Index2D(0, idx.1 + 1),
                line_init,
                on_element,
            ),
            Direction::SouthToNorth => self.sweep_by(
                state,
                (0, self.height - 1).into(),
                |idx| idx.1 = idx.1.wrapping_sub(1),
                |idx| *idx = Index2D(idx.0 + 1, height - 1),
                line_init,
                on_element,
            ),
            Direction::EastToWest => self.sweep_by(
                state,
                (self.width - 1, 0).into(),
                |idx| idx.0 = idx.0.wrapping_sub(1),
                |idx| *idx = Index2D(width - 1, idx.1 + 1),
                line_init,
                on_element,
            )
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