use min_max_heap::MinMaxHeap;
use ordered_float::OrderedFloat;

struct Coords(f64, f64);

impl Coords {
    // Positive solution t to (x + t)(y + t) = 1
    fn mk_square_shift(&self) -> f64 {
        let x = self.0;
        let y = self.1;
        let s = x + y;
        ((s * s - 4.0 * x * y + 4.0).sqrt() - s) / 2.0
    }

    fn mk_area(self, area_index: Index) -> Area {
        let square_shift = OrderedFloat(self.mk_square_shift());
        Area {
            lb_corner: self,
            area_index,
            square_shift,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
struct Index(u32, u32);

struct Area {
    // Left bottom corner of area.
    lb_corner: Coords,
    // Index of to-be-produced square.
    area_index: Index,
    // Precomputed value, if (x, y) == lb_corner, then
    // (x + square_shift, y + square_shift)
    // is right upper corner of to-be-produced square.
    square_shift: OrderedFloat<f64>,
}

impl PartialEq for Area {
    fn eq(&self, other: &Area) -> bool {
        self.square_shift.eq(&other.square_shift)
    }
}

impl Eq for Area {}

impl PartialOrd for Area {
    fn partial_cmp(&self, other: &Area) -> Option<std::cmp::Ordering> {
        self.square_shift.partial_cmp(&other.square_shift)
    }
}

impl Ord for Area {
    fn cmp(&self, other: &Area) -> std::cmp::Ordering {
        self.square_shift.cmp(&other.square_shift)
    }
}

impl Area {
    // Build a square in area and split it to two smaller areas:
    // above and to the right
    fn split(self) -> (Area, Area) {
        let Coords(x, y) = self.lb_corner;
        let Index(ix, iy) = self.area_index;
        let t = self.square_shift.0;
        (
            Coords(x, y + t).mk_area(Index(ix + 1, iy)),
            Coords(x + t, y).mk_area(Index(ix, iy + 1)),
        )
    }
}

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Square {
    num: u32,
    square_index: Index,
}

pub fn euler247() -> String {
    let bound = 1000000;
    let mut area_set = MinMaxHeap::<Area>::new();
    area_set.push(Coords(0.0, 1.0).mk_area(Index(0, 0)));
    let mut square_set = MinMaxHeap::<Square>::new();
    for num in 1..=bound {
        let max = area_set.pop_max().unwrap();
        let square_index = max.area_index.clone();
        let (area_1, area_2) = max.split();
        area_set.push(area_1);
        area_set.push(area_2);
        square_set.push(Square { num, square_index });
    }
    square_set
        .into_iter()
        .filter(|sq| sq.square_index == Index(3, 3))
        .max()
        .unwrap()
        .num
        .to_string()
}
