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
    let mut number = String::new();
    println!("Which problem do you want to run?");
    std::io::stdin().read_line(&mut number).expect("Failed to read line");
    let fun = match number.trim().parse() {
        Ok(1) => solutions::euler001::euler001,
        Ok(2) => solutions::euler002::euler002,
        Ok(3) => solutions::euler003::euler003,
        Ok(4) => solutions::euler004::euler004,
        Ok(5) => solutions::euler005::euler005,
        Ok(6) => solutions::euler006::euler006,
        Ok(7) => solutions::euler007::euler007,
        Ok(8) => solutions::euler008::euler008,
        Ok(9) => solutions::euler009::euler009,
        Ok(10) => solutions::euler010::euler010,
        Ok(num) => panic!("Solution for problem {} does not exist yet!", num),
        Err(_) => panic!("Failed to read a number")
    };
    let now = Instant::now();
    println!("Output: {}", fun());
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