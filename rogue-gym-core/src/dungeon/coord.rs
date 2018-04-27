use rect_iter::{FromTuple2, IntoTuple2};
use tuple_map::TupleMap2;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, PartialOrd, Ord, Eq, Add, Sub, Mul, Div,
         Neg, AddAssign, SubAssign, MulAssign, DivAssign, From, Into, Serialize, Deserialize)]
pub struct X(pub i32);

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, PartialOrd, Ord, Eq, Add, Sub, Mul, Div,
         Neg, AddAssign, SubAssign, MulAssign, DivAssign, From, Into, Serialize, Deserialize)]
pub struct Y(pub i32);

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, PartialOrd, Ord, Eq, Add, Sub, AddAssign,
         SubAssign, Serialize, Deserialize)]
pub struct Coord {
    pub x: X,
    pub y: Y,
}

impl Coord {
    pub fn new<T: Into<X>, U: Into<Y>>(x: T, y: U) -> Self {
        Coord {
            x: x.into(),
            y: y.into(),
        }
    }
    pub fn euc_dist(self, other: Coord) -> f64 {
        let (x, y) = ((self.x - other.x).0, (self.y - other.y).0);
        let f: f64 = (x, y).map(|i| i * i).sum().into();
        f.sqrt()
    }
    #[inline]
    pub fn scale<T: Into<i32>>(mut self, x: T, y: T) -> Self {
        self.x *= x.into();
        self.y *= y.into();
        self
    }
    #[inline]
    pub fn slide_x<T: Into<X>>(mut self, x: T) -> Self {
        self.x += x.into();
        self
    }
    #[inline]
    pub fn slide_y<T: Into<Y>>(mut self, y: T) -> Self {
        self.y += y.into();
        self
    }
    pub fn endless_iter(self, dir: Direction) -> DirectionIterEndless {
        DirectionIterEndless { cur: self, dir }
    }
}

impl FromTuple2<i32> for Coord {
    fn from_tuple2(t: (i32, i32)) -> Coord {
        Coord::new(t.0, t.1)
    }
}

impl IntoTuple2<i32> for Coord {
    fn into_tuple2(self) -> (i32, i32) {
        (self.x.0, self.y.0)
    }
}

impl Into<(i32, i32)> for Coord {
    fn into(self) -> (i32, i32) {
        (self.x.0, self.y.0)
    }
}

impl From<(i32, i32)> for Coord {
    fn from(t: (i32, i32)) -> Coord {
        Coord::new(t.0, t.1)
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, PartialOrd, Ord, Eq, Serialize, Deserialize,
         EnumIterator)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    LeftUp,
    RightUp,
    LeftDown,
    RightDown,
    Stay,
}

impl Direction {
    pub fn to_cd(&self) -> Coord {
        use self::Direction::*;
        match *self {
            Up => Coord::new(0, -1),
            Down => Coord::new(0, 1),
            Left => Coord::new(-1, 0),
            Right => Coord::new(1, 0),
            LeftUp => Coord::new(-1, -1),
            RightUp => Coord::new(1, -1),
            LeftDown => Coord::new(-1, 1),
            RightDown => Coord::new(1, 1),
            Stay => Coord::new(0, 0),
        }
    }
}

pub struct DirectionIter<F> {
    cur: Coord,
    dir: Direction,
    end_condition: Box<F>,
}

impl<F> Iterator for DirectionIter<F>
where
    F: FnMut(Coord) -> bool,
{
    type Item = Coord;
    fn next(&mut self) -> Option<Coord> {
        let f = &self.end_condition;
        if f(self.cur) {
            return None;
        }
        let cur = self.cur;
        self.cur += self.dir.to_cd();
        Some(cur)
    }
}

pub struct DirectionIterEndless {
    cur: Coord,
    dir: Direction,
}

impl Iterator for DirectionIterEndless {
    type Item = Coord;
    fn next(&mut self) -> Option<Coord> {
        let cur = self.cur;
        self.cur += self.dir.to_cd();
        Some(cur)
    }
}
