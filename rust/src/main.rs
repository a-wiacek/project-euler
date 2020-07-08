pub mod utils {
    pub mod collections {
        pub mod find_union;
    }
    pub mod input;
    pub mod itertools;
    pub mod numeric {
        pub mod binsearch;
        pub mod digits;
        pub mod divisors;
        pub mod factorial;
        pub mod sequences {
            pub mod continued_fractions {
                pub mod approximations;
                pub mod e;
                pub mod square_root;
            }
            pub mod fibonacci;
            pub mod heptagonal;
            pub mod hexagonal;
            pub mod octogonal;
            pub mod pentagonal;
            pub mod square;
            pub mod triangular;
        }
    }
    pub mod number_theory {
        pub mod invert_mod;
        pub mod radical;
        pub mod totient;
    }
}
pub mod solutions {
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
    pub mod euler011;
    pub mod euler012;
    pub mod euler013;
    pub mod euler014;
    pub mod euler015;
    pub mod euler016;
    pub mod euler017;
    pub mod euler018;
    pub mod euler019;
    pub mod euler020;
    pub mod euler021;
    pub mod euler022;
    pub mod euler023;
    pub mod euler024;
    pub mod euler025;
    pub mod euler026;
    pub mod euler027;
    pub mod euler028;
    pub mod euler029;
    pub mod euler030;
    pub mod euler031;
    pub mod euler032;
    pub mod euler033;
    pub mod euler034;
    pub mod euler035;
    pub mod euler036;
    pub mod euler037;
    pub mod euler038;
    pub mod euler039;
    pub mod euler040;
    pub mod euler041;
    pub mod euler042;
    pub mod euler043;
    pub mod euler044;
    pub mod euler045;
    pub mod euler046;
    pub mod euler047;
    pub mod euler048;
    pub mod euler049;
    pub mod euler050;
    pub mod euler051;
    pub mod euler052;
    pub mod euler053;
    pub mod euler054;
    pub mod euler055;
    pub mod euler056;
    pub mod euler057;
    pub mod euler058;
    pub mod euler059;
    pub mod euler060;
    pub mod euler061;
    pub mod euler062;
    pub mod euler063;
    pub mod euler064;
    pub mod euler065;
    pub mod euler066;
    pub mod euler067;
    pub mod euler068;
    pub mod euler069;
    pub mod euler070;
    pub mod euler071;
    pub mod euler072;
    pub mod euler073;
    pub mod euler074;
    pub mod euler075;
    pub mod euler076;
    pub mod euler077;
    pub mod euler078;
    pub mod euler079;
    pub mod euler080;
    pub mod euler081;
    pub mod euler082;
    pub mod euler083;
    pub mod euler084;
    pub mod euler085;
    pub mod euler086;
    pub mod euler087;
    pub mod euler088;
    pub mod euler089;
    pub mod euler090;
    pub mod euler091;
    pub mod euler092;
    pub mod euler093;
    pub mod euler094;
    pub mod euler095;
    pub mod euler096;
    pub mod euler097;
    pub mod euler098;
    pub mod euler099;
    pub mod euler100;
    pub mod euler101;
    pub mod euler102;
    pub mod euler103;
    pub mod euler104;
    pub mod euler105;
    pub mod euler106;
    pub mod euler107;
    pub mod euler108;
    pub mod euler109;
    pub mod euler110;
    pub mod euler111;
    pub mod euler112;
    pub mod euler113;
    pub mod euler114;
    pub mod euler115;
    pub mod euler116;
    pub mod euler117;
    pub mod euler118;
    pub mod euler119;
    pub mod euler120;
    pub mod euler121;
    pub mod euler122;
    pub mod euler123;
    pub mod euler124;
    pub mod euler125;
    pub mod euler126;
    pub mod euler127;
    pub mod euler129;
    pub mod euler130;
    pub mod euler131;
    pub mod euler132;
    pub mod euler134;
    pub mod euler135;
    pub mod euler136;
    pub mod euler137;
    pub mod euler138;
    pub mod euler139;
    pub mod euler140;
    pub mod euler142;
    pub mod euler145;
    pub mod euler146;
    pub mod euler149;
    pub mod euler155;
    pub mod euler156;
    pub mod euler157;
    pub mod euler159;
    pub mod euler160;
    pub mod euler161;
    pub mod euler162;
    pub mod euler163;
    pub mod euler164;
    pub mod euler165;
    pub mod euler166;
    pub mod euler168;
    pub mod euler169;
    pub mod euler171;
    pub mod euler172;
    pub mod euler173;
    pub mod euler174;
    pub mod euler178;
    pub mod euler179;
    pub mod euler180;
    pub mod euler181;
    pub mod euler182;
    pub mod euler183;
    pub mod euler188;
    pub mod euler190;
    pub mod euler191;
    pub mod euler197;
    pub mod euler274;
    pub mod euler277;
    pub mod euler278;
    pub mod euler282;
    pub mod euler286;
    pub mod euler291;
    pub mod euler512;
    pub mod euler613;
    pub mod euler622;
    pub mod euler623;
}

project_euler_rust_macros::make_choose_euler_fun!();

fn ask_for_problem_num() -> usize {
    let mut number_str = String::new();
    println!("Which problem do you want to run?");
    std::io::stdin()
        .read_line(&mut number_str)
        .expect("Failed to read a line");
    number_str.trim().parse().expect("Failed to read a number")
}

fn get_problem_num() -> usize {
    match std::env::args().nth(1) {
        Some(number_str) => match number_str.trim().parse() {
            Ok(num) => {
                println!("Running problem {}", num);
                num
            }
            Err(_) => {
                println!("Unrecognized value passed in argument: {}", number_str);
                ask_for_problem_num()
            }
        },
        None => ask_for_problem_num(),
    }
}

fn main() {
    let fun = choose_euler_fun(get_problem_num());
    let now = std::time::Instant::now();
    let result = fun();
    let time = now.elapsed().as_secs_f64();
    println!("Output: {}", result);
    println!("Execution time: {}s", time);
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    fn read_answer(line: usize) -> String {
        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        path.pop();
        path.push("txt");
        path.push("answers.txt");
        std::fs::read_to_string(path)
            .unwrap()
            .lines()
            .nth(line - 1)
            .unwrap()
            .to_string()
    }

    fn test_euler(num: usize) {
        assert_eq!(crate::choose_euler_fun(num)(), read_answer(num))
    }

    project_euler_rust_macros::make_tests!();
}
