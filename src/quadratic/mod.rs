use std::process::exit;

#[derive(Debug, PartialEq)]
pub struct Quadratic {
    pub equation: String,  // ax^2 + bx + c
    pub variables: (i32, i32, i32)
}

impl Quadratic {
    fn parse_term(term: &str) -> i32 {
        return if term[..].contains("-") {
            term[..2].parse::<i32>().unwrap_or_else(|err| {
                eprintln!("{}", format!("Failed to parse the constant term : {}", err));
                exit(1);
            })
        } else if term[..].contains("+") {
            term[1..2].parse::<i32>().unwrap_or_else(|err| {
                eprintln!("{}", format!("Failed to parse the constant term: {}", err));
                exit(1);
            })
        } else {
            term[..1].parse::<i32>().unwrap_or_else(|err| {
                eprintln!("{}", format!("Failed to parse the constant term: {}", err));
                exit(1);
            })
        };
    }

    pub fn from_equation(equation: &str) -> Quadratic {
        let components: Vec<&str> = equation.split(" ").collect();

        if components.len() < 3 {
            eprintln!("Not enough terms for quadratic equation");
            exit(1);
        }

        let a_string = components.get(0)
            .unwrap_or_else(|| {
                eprintln!("Constant term 'a' is missing.");
                exit(1);
            });

        if a_string.chars().nth(0).is_some_and(|t| t.is_alphabetic()) {
            eprintln!("Invalid 'a' term");
            exit(1);
        }
        
        let b_string = components.get(1)
            .unwrap_or_else(|| {
                eprintln!("Constant term 'b' is missing.");
                exit(1);
            });
        
        let c_string = components.get(2)
            .unwrap_or_else(|| {
                eprintln!("Constant term 'c' is missing.");
                exit(1);
            });
        
        let a: i32 = Self::parse_term(&a_string);
        let b: i32 = Self::parse_term(&b_string);
        let c: i32 = Self::parse_term(&c_string);

        Quadratic { 
            equation: equation.clone().to_string(), 
            variables: (a, b, c)
        }
    } 
}