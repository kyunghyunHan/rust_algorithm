use std::cmp::Ordering;

pub fn example() {
    pointer_practice();
}
fn pointer_practice() {
    /*2차원배열 */
    /*
    a 는 표현식에서 대부분 &a[0] 으로 변형
    &a[0]의 타입은 int (*)[8];



    */
    let a = [[0i32; 8]; 3];

    let ap: *const [i32; 8] = a.as_ptr();

    println!(
        "{:p} {:p} {:p} {:p} {:p}",
        &a,            // C의 &a int (*)[3][8] : 전체 배열 a의 주소
        a.as_ptr(),    // C의 a [0]의 주소
        &a[0],         // C의 &a[0] a[0]의주소
        a[0].as_ptr(), // C의 a[0] a[0][0]의주소
        &a[0][0],      // C의 &a[0][0] a[0][0]의 주소
    );

    unsafe {
        println!(
            "{:p} {:p} {:p} {:p} {:p}",
            (&a as *const [[i32; 8]; 3]).add(1),
            // C: &a + 1
            // 전체 [[i32;8];3] 배열 크기만큼 이동
            // 3 * 8 * sizeof(i32)
            a.as_ptr().add(1),
            // C: a + 1
            // a.as_ptr() 타입: *const [i32;8]
            // [i32;8] 한 줄만큼 이동
            // 결과: &a[1]
            (&a[0] as *const [i32; 8]).add(1),
            // C: &a[0] + 1
            // [i32;8] 한 줄만큼 이동
            // 결과: &a[1]
            a[0].as_ptr().add(1),
            // C: a[0] + 1
            // *const i32
            // i32 1개만큼 이동
            // 결과: &a[0][1]
            (&a[0][0] as *const i32).add(1),
            // C: &a[0][0] + 1
            // i32 1개만큼 이동
            // 결과: &a[0][1]
        );
    }
    println!(
        "{} {} {} {} {}",
        size_of_val(&&a),       // C: sizeof(&a)       → 참조 크기
        size_of_val(&a),        // C: sizeof(a)        → 96
        size_of_val(&&a[0]),    // C: sizeof(&a[0])    → 참조 크기
        size_of_val(&a[0]),     // C: sizeof(a[0])     → 32
        size_of_val(&&a[0][0]), // C: sizeof(&a[0][0]) → 참조 크기
    );

    unsafe{
        println!("{:p} {:p}",ap,ap.add(1));
    }
}
/// 2차원 배열을 참조와 원시 포인터로 순회하는 간단한 예제입니다.
pub fn matrix_pointer_example() {
    let mut matrix = [[0; 8]; 3];

    // 안전한 참조를 사용한 접근
    matrix[1][1] = 10;
    let second_row = &matrix[1];
    println!("reference: matrix[1][1] = {}", second_row[1]);

    // 같은 위치를 원시 포인터 연산으로 접근
    let matrix_ptr = matrix.as_ptr();
    unsafe {
        let second_row_ptr = matrix_ptr.add(1);
        let element_ptr = (*second_row_ptr).as_ptr().add(1);

        println!("raw pointer: matrix[1][1] = {}", *element_ptr);
    }
}

fn compare_i32(a: i32, b: i32) -> i32 {
    ordering_to_i32(a.cmp(&b))
}

/// 두 유효한 `i32` 포인터가 가리키는 값을 비교합니다.
///
/// # Safety
///
/// `a`와 `b`는 읽을 수 있고 올바르게 정렬된 `i32`를 가리켜야 합니다.
unsafe fn compare_i32_ptrs(a: *const i32, b: *const i32) -> i32 {
    unsafe { compare_i32(*a, *b) }
}

fn ordering_to_i32(ordering: Ordering) -> i32 {
    match ordering {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

fn factorial(n: usize) -> u64 {
    assert!(n <= 20, "u64로 표현할 수 있는 범위는 20!까지입니다");

    (1..=n as u64).product()
}

fn copy_string(to: &mut String, from: &str) {
    to.clear();
    to.push_str(from);
}

fn compare_bytes(left: &[u8], right: &[u8]) -> i32 {
    for (&left_byte, &right_byte) in left.iter().zip(right) {
        match left_byte.cmp(&right_byte) {
            Ordering::Equal => continue,
            ordering => return ordering_to_i32(ordering),
        }
    }

    ordering_to_i32(left.len().cmp(&right.len()))
}

/// 널 종료 문자열 `from`을 `to`로 복사하고 `to`를 반환합니다.
///
/// # Safety
///
/// - `from`은 읽을 수 있는 널 종료 문자열을 가리켜야 합니다.
/// - `to`에는 문자열과 널 문자를 담을 공간이 충분해야 합니다.
/// - 복사가 끝날 때까지 두 메모리 영역이 겹치면 안 됩니다.
unsafe fn copy_c_string(to: *mut u8, from: *const u8) -> *mut u8 {
    let mut destination = to;
    let mut source = from;

    unsafe {
        while *source != 0 {
            *destination = *source;
            destination = destination.add(1);
            source = source.add(1);
        }

        *destination = 0;
    }

    to
}

/// 두 널 종료 바이트 문자열을 사전순으로 비교합니다.
///
/// # Safety
///
/// `left`와 `right`는 각각 읽을 수 있는 널 종료 문자열을 가리켜야 합니다.
unsafe fn compare_c_strings(mut left: *const u8, mut right: *const u8) -> i32 {
    unsafe {
        while *left == *right {
            if *left == 0 {
                return 0;
            }

            left = left.add(1);
            right = right.add(1);
        }

        ordering_to_i32((*left).cmp(&*right))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compares_integers() {
        assert_eq!(compare_i32(1, 2), -1);
        assert_eq!(compare_i32(2, 2), 0);
        assert_eq!(compare_i32(3, 2), 1);

        let left = 10;
        let right = 20;
        assert_eq!(unsafe { compare_i32_ptrs(&left, &right) }, -1);
    }

    #[test]
    fn calculates_factorial() {
        assert_eq!(factorial(0), 1);
        assert_eq!(factorial(5), 120);
        assert_eq!(factorial(20), 2_432_902_008_176_640_000);
    }

    #[test]
    fn copies_strings() {
        let mut safe_destination = String::from("old value");
        copy_string(&mut safe_destination, "abc");
        assert_eq!(safe_destination, "abc");

        let source = b"hello\0";
        let mut destination = [0_u8; 6];
        unsafe {
            copy_c_string(destination.as_mut_ptr(), source.as_ptr());
        }
        assert_eq!(&destination, source);
    }

    #[test]
    fn compares_strings() {
        assert_eq!(compare_bytes(b"abc", b"abc"), 0);
        assert_eq!(compare_bytes(b"abc", b"abd"), -1);
        assert_eq!(compare_bytes(b"abcd", b"abc"), 1);

        assert_eq!(
            unsafe { compare_c_strings(c"abc".as_ptr().cast(), c"abc".as_ptr().cast()) },
            0
        );
        assert_eq!(
            unsafe { compare_c_strings(c"abc".as_ptr().cast(), c"abd".as_ptr().cast()) },
            -1
        );
    }
}
