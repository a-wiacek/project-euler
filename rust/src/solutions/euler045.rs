use crate::utils::numeric::sequences::{hexagonal, pentagonal, triangular};

pub fn euler045() -> String {
    let mut tri = triangular::Triangular::<u64>::new().skip(285).peekable();
    let mut pen = pentagonal::Pentagonal::<u64>::new().skip(165).peekable();
    let mut hex = hexagonal::Hexagonal::<u64>::new().skip(143).peekable();
    loop {
        let t = tri.peek().unwrap();
        let p = pen.peek().unwrap();
        let h = hex.peek().unwrap();
        if t == p && t == h {
            break t.to_string();
        } else if t <= p && t <= h {
            tri.next();
        } else if p <= t && p <= h {
            pen.next();
        } else {
            hex.next();
        }
    }
}
