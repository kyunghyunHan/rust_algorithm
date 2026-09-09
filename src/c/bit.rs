struct BitFiled {
    data: u8,
}
impl BitFiled {
    fn a(&self) -> u8 {
        //0 1 2 사용
        self.data & 0b0000_0111
    }
    fn b(&self) -> u8 {
        //3 4 5비티들을 꺼내서 어떤값인지 확인
        //원본 0 1 1 0 1 1 0 1
        //>>3 0 0 0 0 1 1 0 1
        //    0 0 0 0 0 1 1 1
        //101
        (self.data >> 3) & 0b0000_0111
    }
}
/*
Little endian
Big enian

u
*/
union data {
    i: i32,
    bytes: [u8; 4],
}

fn bitwise_not() {
    let a: i8 = -45;
    //32+8+4+1
    //    00101101
    //음수 11010011
    println!("{:8b}", -a);
}

fn bitt() {
    let a = 0x33CC33CC;
    /*
    1로 만들기 : a |  (1 << n)
    0으로 만들기: a & !(1 << n)
    반전하기    : a ^  (1 << n)
    확인하기    : a &  (1 << n)

         */
    //비트를 읽을떄
    //<< 비트를 설정할떄
    //1번을 5번까지 이동시키고 그다음에 OR을 해서
    //둘중 하나라도 1이면 1
    println!("{:032b}", (1 << 0) | a);
    println!("{:X}", (1 << 5) | a);
    println!("{:X}", (1 << 21) | a);
    println!("{:X}", (1 << 22) | a);
    //0이 하나라도 있으면 0
    println!("{:X}", a & !(1 << 0));
    println!("{:X}", a & !(1 << 5));
    println!("{:X}", a & !(1 << 21));
    println!("{:X}", a & !(1 << 22));
    //두비트가 서로 다르면 1 같으면 0
    println!("{:X}", a ^ (1 << 0));
    println!("{:X}", a ^ (1 << 5));
    println!("{:X}", a ^ (1 << 21));
    println!("{:X}", a ^ (1 << 22));
}
pub fn example() {
    bitwise_not();
}
fn union_test() {
    let data = data { i: 0x1234_5678 };

    unsafe {
        println!("{:X}", data.bytes[0]);
    }
}
fn bit_field() {
    let t = BitFiled { data: 0b0000_0111 };
    println!("{}", t.b());
}
fn byte() {
    /*
    10진수 는 최대 4바이트 필요
    2진수 한자리는 1비트
    8진수 한자리는 3비트
    16진수 한자리는 4비트
     */
    /*
    char      = 1 Byte =  8 bit = 16진수  2자리

     short     = 2 Byte = 16 bit = 16진수  4자리

     int       = 4 Byte = 32 bit = 16진수  8자리

     long      = 환경에 따라 다름
               64bit macOS/Linux에서는 보통
               8 Byte = 64 bit = 16진수 16자리

     long long = 보통
               8 Byte = 64 bit = 16진수 16자리

     float     = 보통
               4 Byte = 32 bit = 16진수  8자리

     double    = 보통
               8 Byte = 64 bit = 16진수 16자리

     pointer   = 64bit 환경에서는 보통
               8 Byte = 64 bit = 16진수 16자리
          */
    let a = 0x1;
    let b = 0o1;
    let c = 0b0001;
    println!("{}", a == b && b == c);

    let a1 = 0xA; //16
    let b1 = 0o7; //8
    let c1 = 0b0010; //2
}
