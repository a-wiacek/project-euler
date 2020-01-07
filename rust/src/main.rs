#![allow(dead_code)]
mod solutions {
    pub mod euler001;
    pub mod euler002;
    pub mod euler003;
    pub mod euler004;
    pub mod euler005;
    pub mod euler006;
    pub mod euler007;
    pub mod euler008;
    pub mod euler009;
    pub mod euler010;
}
use std::time::Instant;

fn main() {
    let now = Instant::now();
    println!("Output: {}", solutions::euler008::euler008());
    println!("Execution time: {} s", now.elapsed().as_secs_f64());
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    fn read_answer(line: usize) -> String {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.pop();
        path.push("txt");
        path.push("answers.txt");
        std::fs::read_to_string(path).unwrap().lines().nth(line - 1).unwrap().to_string()
    }

    #[test]
    fn test_euler001() {
        assert_eq!(crate::solutions::euler001::euler001(), read_answer(1));
    }

    #[test]
    fn test_euler002() {
        assert_eq!(crate::solutions::euler002::euler002(), read_answer(2));
    }

    #[test]
    fn test_euler003() {
        assert_eq!(crate::solutions::euler003::euler003(), read_answer(3));
    }
    
    #[test]
    fn test_euler004() {
        assert_eq!(crate::solutions::euler004::euler004(), read_answer(4));
    }
    
    #[test]
    fn test_euler005() {
        assert_eq!(crate::solutions::euler005::euler005(), read_answer(5));
    }
    
    #[test]
    fn test_euler006() {
        assert_eq!(crate::solutions::euler006::euler006(), read_answer(6));
    }
    
    #[test]
    fn test_euler007() {
        assert_eq!(crate::solutions::euler007::euler007(), read_answer(7));
    }
    
    #[test]
    fn test_euler008() {
        assert_eq!(crate::solutions::euler008::euler008(), read_answer(8));
    }
    
    #[test]
    fn test_euler009() {
        assert_eq!(crate::solutions::euler009::euler009(), read_answer(9));
    }
    
    #[test]
    fn test_euler010() {
        assert_eq!(crate::solutions::euler010::euler010(), read_answer(10));
    }
}