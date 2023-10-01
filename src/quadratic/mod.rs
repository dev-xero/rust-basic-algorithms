#[derive(Debug, PartialEq)]
pub struct Quadratic {
    pub equation: String,  // ax^2 + bx + c
    pub variables: (i32, i32, i32)
}

impl Quadratic {
    fn parse_term(term: &str) -> Result<i32, &'static str> {
        if term.contains("-") {
            Ok(term[..2].parse::<i32>().map_err(|_| "Failed to parse term")?)
        } else if term.contains("+") {
            Ok(term[1..2].parse::<i32>().map_err(|_| "Failed to parse term")?)
        } else {
            Ok(term[..1].parse::<i32>().map_err(|_| "Failed to parse term")?)
        }
    }

    pub fn from_equation(equation: &str) -> Result<Quadratic, &'static str> {
        let components: Vec<&str> = equation.split_whitespace().collect();

        if components.len() < 3 {
            return Err("Not enough arguments")
        }

        let a_term: &str = components.get(0).ok_or_else(|| "Constant term 'a' is missing.")?;
        let b_term = components.get(1).ok_or_else(|| "Constant term 'b' is missing.")?;
        let c_term = components.get(2).ok_or_else(|| "Constant term 'c' is missing.")?;

        let a: i32 =  if a_term
            .chars().nth(0).is_some_and(|t| t.is_alphabetic()) {
            1
        } else {
            Self::parse_term(&a_term)?
        };
        let b: i32 = Self::parse_term(&b_term)?;
        let c: i32 = Self::parse_term(&c_term)?;

        Ok(
            Quadratic { 
                equation: equation.clone().to_string(), 
                variables: (a, b, c)
            }
        )
    } 
}