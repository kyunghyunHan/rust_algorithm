#[derive(Debug, PartialEq, Clone)]
struct FieldElement {
    num: i64,
    prime: i64,
}

impl FieldElement {
    fn new(num: i64, prime: i64) -> Result<FieldElement, String> {
        if num >= prime || num < 0 {
            return Err(format!("Num {} not in field range 0 to {}", num, prime - 1));
        }
        Ok(FieldElement { num, prime })
    }

    fn add(&self, other: &FieldElement) -> Result<FieldElement, String> {
        if self.prime != other.prime {
            return Err("Cannot add two numbers in different Fields".to_string());
        }
        let num = (self.num + other.num) % self.prime;
        Ok(FieldElement::new(num, self.prime)?)
    }

    fn sub(&self, other: &FieldElement) -> Result<FieldElement, String> {
        if self.prime != other.prime {
            return Err("Cannot subtract two numbers in different Fields".to_string());
        }
        let num = (self.num - other.num + self.prime) % self.prime;
        Ok(FieldElement::new(num, self.prime)?)
    }

    fn mul(&self, other: &FieldElement) -> Result<FieldElement, String> {
        if self.prime != other.prime {
            return Err("Cannot multiply two numbers in different Fields".to_string());
        }
        let num = (self.num * other.num) % self.prime;
        Ok(FieldElement::new(num, self.prime)?)
    }

    fn pow(&self, exponent: i64) -> Result<FieldElement, String> {
        let n = exponent.rem_euclid(self.prime - 1);
        let num = i64::pow(self.num, n as u32) % self.prime;
        Ok(FieldElement::new(num, self.prime)?)
    }
    fn truediv(&self, other: &FieldElement) -> Result<FieldElement, String> {
        if self.prime != other.prime {
            return Err("Cannot divide two numbers in different Fields".to_string());
        }
        let num = self.num * i64::pow(other.num, (self.prime - 2)as u32) % self.prime;
        Ok(FieldElement::new(num, self.prime)?)
    }
}

pub fn main(){
    let a= FieldElement::new(8, 13).unwrap();
    let b= FieldElement::new(8, 13).unwrap();
    
    // println!("{}",  FieldElement::eq(&a,  &b));
    /*거듭제곱 */
    let pow= FieldElement::pow(&a, -3).unwrap();
    println!("{:?}", pow);
    
    println!("{}",  FieldElement::eq(&pow,  &b));
    let g = a.truediv(&b).unwrap();
    println!("a / b = {:?}", g);

    /*나머지 연산 */
    // println!("{}",7%3);
    // println!("{}",-27%13);
}

