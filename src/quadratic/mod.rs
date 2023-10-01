use std::process::exit;

#[derive(Debug, PartialEq)]
pub struct Quadratic {
    pub equation: String,  // ax^2 + bx + c
    pub variables: (i32, i32, i32)
}

impl Quadratic {
    pub fn from_equation(equation: &str) -> Quadratic {
        let components: Vec<&str> = equation.split(" ").collect();
        if components.len() != 5 {
            eprintln!("Not enough terms for quadratic equation");
            exit(1);
        }

        let a: i32 = components.get(0)
            .unwrap_or_else(|| {
                eprintln!("Constant term 'a' is missing.");
                exit(1);
            })
            .parse()
            .unwrap_or_else(|err| {
                eprintln!("{}", format!("Failed to parse the constant term 'a': {}", err));
                exit(1);
            });

        let b: i32 = components.get(2)
            .unwrap_or_else(|| {
                eprintln!("Constant term 'b' is missing.");
                exit(1);
            })
            .parse()
            .unwrap_or_else(|err| {
                eprintln!("{}", format!("Failed to parse the constant term 'b': {}", err));
                exit(1);
            });

        let c: i32 = components.get(4)
            .unwrap_or_else(|| {
                eprintln!("Constant term 'c' is missing.");
                exit(1);
            })
            .parse()
            .unwrap_or_else(|err| {
                eprintln!("{}", format!("Failed to parse the constant term 'c': {}", err));
                exit(1);
            });

        Quadratic { 
            equation: equation.clone().to_string(), 
            variables: (a, b, c)
        }
    } 
}