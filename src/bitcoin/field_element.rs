/*유한체
타원 곡선 암호를 파악하기 위해 필요
타원 곡선 암호:비트코인의 핵심인 전자서명과 서명 검증 알고리즘을 이해하는데 필수


유한체는 덧셈 곱셉을 가진 집합이며 그 집합의 원수 수가 유한하다.
1.a와 b가 집합에 속해 있으면 a+b와 a*b도 집합안에 있다.(집합 위에 두 연산 + *이 닫혀있다) ->덧셈과 곱셈 연산의 결과가 그 집합안에 있도록 두연산을 정의해야한다{0,1,2}는 덧셈에 대해 닫혀있 지 않다.
2.집합에 0으로 표기하는 원소가 존재하고 집합 내 다른원소 a와 + 연산결과는 a다.즉 a+0= a(+연산에 대한 항등원 존재)
3.집합에 1로 표기하는 원소가 존재하고 집합 내 다른 원소a와 *연산 결과는 a다 (a*1= a)
4.집합의 원소a와 + 연산결과가 0이 되개 하는 원소 b가 역시 집합에 속해있고 이러한 b를 -a로 표기한다(+연산에 대한 a의 역원 -a존재)->집합 내에 -a가 집합내에 있다
5.0이 아닌 집합의 원소 a에 대해 a*b=1이 되게하는 원소 b가 역시 집합에 속해 있 고 이러한 b를 a^-1로 표기한다(*연산에 대한 a의 역원 a^-1)존재
- 집합의 위수:유한 개수의 숫자를 원소로 하는 집합에서 집합크기를 표현하는 어떤값 P
- 집합의 위수가 p이면 집합의 원소는 0,1,2...p-1로 쓸수 있다.
- F_p= {0,1,2..p-1}
- 중괄호 안에 잇는 것은 집합의 원소
- F_p= 위수P의 유한체 라고 읽는 특정 유한체,p는 집합의 위수로 잡힌 집합 안의 원소 개수
- 위수 11의 유한체 =  Fq (q = 11) ={0,1,2,3,4,5,6,7,8,9,10}
- 유한체는 위수가 소수,유한체는 반드시 소수이거나 소수의 거듭제곱을 위수로 가져야한다.

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
    fn fmt(&self,)  {
        print!( "FieldElement_{}({})", self.prime, self.num)
    }
    //두 개체가 같은지 검사 객체의 num과 prime가 같은경우에만 True값 반환
    fn eq(&self, other: &FieldElement) -> bool {
        if self.num == other.num && self.prime == other.prime {
            true
        } else {
            false
        }
    }
    //두개체가 서로 다른지 검사
    fn ne(&self, other: &FieldElement) -> bool {
        !self.eq(other)
    }
    //유한체 덧셈
    //
    fn add(&self, other: &FieldElement) -> Result<FieldElement, String> {
        //더해지는 수와 더해지는 수의 위수가 동일한지 확인
        
        if self.prime != other.prime {
            return Err("Cannot add two numbers in different Fields".to_string());
        }
        //나머지 연산을 통해 유한체 덧셈 정의
        let num = (self.num + other.num) % self.prime;
        Ok(FieldElement::new(num, self.prime)?)
    }

    fn sub(&self, other: &FieldElement) -> Result<FieldElement, String> {
                //더해지는 수와 더해지는 수의 위수가 동일한지 확인

        if self.prime != other.prime {
            return Err("Cannot subtract two numbers in different Fields".to_string());
        }
       //나머지 연산을 통해 유한체 뺼셈 정의

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
    let a= FieldElement::new(8, 19).unwrap();
    let b= FieldElement::new(17, 19).unwrap();
    println!("{}",  FieldElement::eq(&a,  &b));

   /*나머지 연산 
   시계처럼 생각
   3시  47시간후는  (3+47)%12= 2
   3시  16시간 전에  (3-16)%12= 11
   12분 843분뒤 = (12+843)%60 = 15
   - 아무리 큰 숫자라도 나머지 연산 후 비교적 작은 범위의 숫자로 변환대기 때문에 숫자 개수가 한정되어 있는 유한체에서 매우 적절한 속성이 된다.
   
   */
    //7%3 1
    println!("{}",7%3);
    println!("{}=60",1747 % 241);
    println!("{}",-27%13);

    /*유한체 덧셈과 뺼셈 
    유한체에서 덧셈을 정의할떄 그결과가 여전히 유한체에 속해있어여 한다.
    Fq (q = 19)= {0,1,2...18}
    a+b= F-19
    a+b= (a+b)%19
    7+f8= (7+8)%19= 15
    11+f17= (11+17)%19= 9
    덧셈에 대한 역원
    - -fa= (-a) %p
    - -f9 = (-9) % 19 = 10
    - -f10 = -10 %19
    = -9 + f10= 0

    - 10은 9의 덧셈에 대한 역원


    */
    let a1: FieldElement= FieldElement::new(8, 19).unwrap();
    let b1= FieldElement::new(17, 19).unwrap();
    println!("계산={}",(11-9)% 19);
    //Rust에서는 음수를 모듈로 연산할때 결과가 음수로 유지
    println!("Rust={}",(((11-9) % 19 + 19) % 19));
    println!("계산2={}",(11-9) % 19 );
    //
    println!("Rust={}",(((6-13) % 19 + 19) % 19));
    println!("계산2={}",(6-13) % 19 );

    println!("더하기={:?}",FieldElement::add(&a1, &b1));
    println!("빼기={:?}",FieldElement::sub(&a1, &b1));

    
    /*유한체 곱셈과 거듭제곱 \
    정수에서의 곱셈은 여러번 더하기
    5*3= 5+5+5
    유한체에서도 비슷하게 가능
    5*f3= 5+f5+f5 = 15 % 19
    5*f17= 8+f8+f8...+f8 = (8*17) %19 = 136 %19
    거듭제곱은 숫자를 여러번 곱하는것
    7^3= 7*7 * 7= 343
    7^3= 343 %19 = 1
    9^12=7
    */
    let a2: FieldElement= FieldElement::new(8, 19).unwrap();
    let b2= FieldElement::new(17, 19).unwrap();
    println!("곱셉={}",136 % 19 );
    println!("곱셉={:?}",FieldElement::mul(&a2, &b2));
   


    let a3: FieldElement= FieldElement::new(9, 19).unwrap();

    /*거듭제곱 */
    let pow= FieldElement::pow(&a3, 12).unwrap();
    println!("거듭제곱={}", i64::pow(9,12)% 19 );
    println!("거듭제곱={:?}", pow );
     
    println!("{}",  FieldElement::eq(&pow,  &b));
    // println!("a / b = {:?}", g);
    FieldElement::fmt(&a);

    /*나눗셈
    
    - 일반대수에서는 나눗셈은 곱셈의 역연산
    7*8= 56 은 56/8=  7을 의미
    유한체에서도 유효
    3*f_7= 21%19 = 2로부터 2/f_7 = 3이라는 등식이 성립
    9*f_5= 45%19 = 7로부터 7/f_5 = 9라는 등식이 성립

    - 나눗셈 결과로 보면 일반 수학에서의 나눗셈과 다르다.유한체원소끼리의 나눗셈이기때문에
    - 의문 ? = 3*f_7 = 2를 모르는 상화에서 어떻게 2/f_7를 계산하는가
    - p와 0보다 큰 n에 대해 n^(p-1)은 항상 1이다
    - n^(p-1) %p = 1
    - 페르마의 소정리
    - 나눗셈은 곱셈의 역연산이기 떄문에
    - a/b =a*f_(1/b)  = a*f_b^-1
    - 여기서 b^-1을 안다면 나눗셈 문제가 곱셈 문제로 바뀐다.
    - b^-1을 계산하는데 페르마의 소정리 활용
   - b^p-1= 1이고 p는 소수 이기에 
b^-1= b^-1*f1=b^-1*fb^(p-1)= b^(p-2)
= b^-1= b^(p-2)
- 유한체  Fq (q = 19)에서 0이 아닌 모든 원소 b에 대해 b^18= 1을 의미하므로 b^-1= b ^17을 의미
- 2/7 = 2 *7^(19-2) = 2*7^17 = 3
- 7/5= 7 *5^(19-2) =  7*5^17 = 53405761771875 % 19 = 9

- 나눗셈은 가장 시간이 많이 걸리는 연산
     */
    let a4: FieldElement= FieldElement::new(2, 19).unwrap();
    let b4= FieldElement::new(7, 19).unwrap();
    println!("나눗셈={}",136 % 19 );
    println!("나눗셈={:?}",FieldElement::truediv(&a4, &b4));
}

