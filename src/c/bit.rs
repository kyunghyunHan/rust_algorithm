struct BitFiled {
    data: u8,
}
impl BitFiled {
    fn a(&self) -> u8 {
        //0 1 2 사용
        self.data & 0b0000_0111
    }
    fn b(&self) -> u8 {
        //3 4 5사용
        (self.data >> 3) & 0b0000_0111
    }
}

fn bit_field() {
    let t = BitFiled { data: 0b0000_0111 };
    println!("{}", t.b());
}
pub fn example() {
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
