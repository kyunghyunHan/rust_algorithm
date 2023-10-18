/*유한체
타원 곡선 암호를 파악하기 위해 필요
타원 곡선 암호:비트코인의 핵심인 전자서명과 서명 검증 알고리즘을 이해하는데 필수


유한체는 덧셈 곱셉을 가진 집합이며 그 집합의 원수 수가 유한하다.


*/







#[derive(Debug, PartialEq, Clone)]
struct FieldElement {
    num: i64,
    prime: i64,
}

impl FieldElement {
    //num과 prime를 인수로 받은 후 num값이 경곗값을 포함하여 0과 prime-1사이 값인지 조사,그렇지 않은 경우 유효하지 않은 FileElement를 얻게 되므로 Error를 발생
    fn new(num: i64, prime: i64) -> Result<FieldElement, String> {
        if num >= prime || num < 0 {
            return Err(format!("Num {} not in field range 0 to {}", num, prime - 1));
        }
        //조사된 인수값 반환
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

