#[derive(Debug, PartialEq)]
pub struct Quadratic {
    pub equation: String,           // ax^2 + bx + c
    pub variables: (i32, i32, i32)
}

impl Quadratic {
    fn parse_term(term: &str) -> Result<i32, &'static str> {
        let term = term.trim();

        if term.is_empty() {
            Ok(0)
        } else if term.contains("x^2") {
            if term == "x^2" { return Ok(1) }
            let coeff = &term[..term.len() - 3].trim();
            Ok(coeff.parse::<i32>().map_err(|_| "Failed to parse 'a' term")?)
        } else if term.contains("x") {
            let coeff = &term[..term.len() - 1].trim();
            Ok(coeff.parse::<i32>().map_err(|_| "Failed to parse 'b' term")?)
        } else {
            term.parse::<i32>().map_err(|_| "Failed to parse 'c' term")
        }
    }

    pub fn from_equation(equation: &str) -> Result<Quadratic, &'static str> {
        let components: Vec<&str> = equation.split_whitespace().collect();

        if components.len() < 3 {
            return Err("Not enough arguments")
        }

        let a_term: &str = components.get(0).ok_or_else(|| "Constant term 'a' is missing.")?;
        let b_term: &str = components.get(1).ok_or_else(|| "Constant term 'b' is missing.")?;
        let c_term: &str = components.get(2).ok_or_else(|| "Constant term 'c' is missing.")?;

        let a: i32 = Self::parse_term(&a_term)?;
        let b: i32 = Self::parse_term(&b_term)?;
        let c: i32 = Self::parse_term(&c_term)?;

        Ok(
            Quadratic { 
                equation: equation.to_string(), 
                variables: (a, b, c)
            }
        )
    } 

    fn discriminant(&self) -> i32 {
        let (a, b, c) = self.variables;
        b.pow(2) - 4 * (a * c)
    }
    
    pub fn root(&self) -> (f64, f64) {
        let a = self.variables.0 as f64;
        let b = self.variables.1 as f64;
        let d = Self::discriminant(&self) as f64;

        if d >= 0.0 {
            let r1 = (-b + d.sqrt()) / (2.0 * a);
            let r2 = (-b - d.sqrt()) / (2.0 * a);

            (r1, r2)
        } else {
            let rp = -b / (2.0 * a);           // real part
            let ip = (-d).sqrt() / (2.0 * a);  // imaginary part

            (rp, ip)
        }
    }
}