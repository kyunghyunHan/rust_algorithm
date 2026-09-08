use super::utils::sum_2d;
use crate::c::utils::{exchange, print_var_array, sum_1d};
use std::alloc::dealloc;
use std::alloc::{alloc, handle_alloc_error, realloc, Layout};
use std::io::Write;
use std::io::{self, stdin};
use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
    ptr,
};
const MATRIX_ROWS: usize = 3;
const MATRIX_COLS: usize = 4;
const ANIMAL_COUNT: usize = 5;
const ANIMAL_NAME_SIZE: usize = 10;
unsafe fn swap02(ap: *mut std::ffi::c_void, bp: *mut std::ffi::c_void, size: usize) {
    let mut temp = [0u8; 100];

    assert!(size <= temp.len());

    unsafe {
        // temp <- ap
        std::ptr::copy(ap as *const u8, temp.as_mut_ptr(), size);

        // ap <- bp
        std::ptr::copy(bp as *const u8, ap as *mut u8, size);

        // bp <- temp
        std::ptr::copy(temp.as_ptr(), bp as *mut u8, size);
    }
}
fn sort(ary: *mut i32, n: usize) {
    unsafe {
        let mut s = n - 1;
        for i in 0..s {
            for j in i + 1..n {
                if *ary.add(i) > *ary.add(j) {
                    let mut temp: i32 = 0;

                    std::ptr::copy(ary.add(i), &mut temp, 1);
                    std::ptr::copy(ary.add(j), ary.add(i), 1);
                    std::ptr::copy(&temp, ary.add(j), 1);
                }
            }
        }
    }
}
pub fn example() {
    malloc();
}
fn malloc() {
    unsafe {
        let layout = Layout::array::<*mut u8>(5).unwrap();
        let arr = alloc(layout) as *mut *mut u8;

        let s_layout = Layout::array::<u8>(80 * 5).unwrap();
        let mut s = alloc(s_layout) as *mut u8;

        for i in 0..5 {
            *arr.add(i) = s;
            let mut input = String::new();
            let mut reader = BufReader::new(stdin().lock());
            reader.read_line(&mut input).unwrap();
            // arr[i] = s;
            *arr.add(i) = s;

            // '\n' 제거
            let input = input.trim_end_matches(&['\n', '\r'][..]);

            let bytes = input.as_bytes();

            // 문자열 복사
            ptr::copy_nonoverlapping(bytes.as_ptr(), s, bytes.len());

            // C 문자열처럼 마지막에 '\0'
            *s.add(bytes.len()) = 0;

            // s += strlen(s) + 1;
            s = s.add(bytes.len() + 1);
        }
        for i in 0..5 {
            let p = *arr.add(i);

            let mut len = 0;

            while *p.add(len) != 0 {
                len += 1;
            }

            let slice = std::slice::from_raw_parts(p, len);
            let text = std::str::from_utf8(slice).unwrap();

            println!("{}", text);
        }
    }
}
fn alloc_test() {
    unsafe {
        const COUNT: usize = 5;
        const SIZE: usize = 80;

        let mut arr: [*mut u8; COUNT] = [ptr::null_mut(); COUNT];
        let mut offset: [usize; COUNT] = [0; COUNT];

        // C: malloc(5 * 80)
        let old_layout = Layout::array::<u8>(COUNT * SIZE).unwrap();

        arr[0] = alloc(old_layout);

        if arr[0].is_null() {
            handle_alloc_error(old_layout);
        }

        for i in 0..COUNT {
            if i > 0 {
                // 이전 문자열 길이 + '\0'
                let prev_len = c_strlen(arr[i - 1]);

                arr[i] = arr[i - 1].add(prev_len + 1);
            }

            /*문자열 입력후 복사 */
            print!("입력: ");
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim_end();
            let bytes = input.as_bytes();
            // arr[i] 위치에 문자열 복사
            ptr::copy_nonoverlapping(bytes.as_ptr(), arr[i], bytes.len());
            // C 문자열처럼 마지막에 '\0'
            *arr[i].add(bytes.len()) = 0;
            // arr[i] - arr[0]
            offset[i] = arr[i].offset_from(arr[0]) as usize;
        }

        // 마지막 문자열의 끝까지 실제 사용한 크기
        let last_len = c_strlen(arr[4]);

        let used = offset[4] + last_len + 1;

        // C:
        // char *p = realloc(arr[0], used);
        let p = realloc(arr[0], old_layout, used);

        if p.is_null() {
            dealloc(arr[0], old_layout);
            handle_alloc_error(Layout::array::<u8>(used).unwrap());
        }

        // C: arr[0] = p;
        arr[0] = p;

        // realloc으로 주소가 바뀌었을 수 있으므로
        // 각 문자열의 시작 주소 다시 설정
        for i in 1..COUNT {
            arr[i] = arr[0].add(offset[i]);
        }

        // 출력
        for i in 0..COUNT {
            let len = c_strlen(arr[i]);

            let slice = std::slice::from_raw_parts(arr[i], len);

            println!("{}", std::str::from_utf8(slice).unwrap());
        }

        // realloc 후 크기가 used가 되었으므로
        // dealloc도 새로운 Layout 사용
        let new_layout = Layout::array::<u8>(used).unwrap();
        dealloc(arr[0], new_layout);
    }
}
// C의 strlen 같은 함수
unsafe fn c_strlen(p: *const u8) -> usize {
    let mut len = 0;

    while unsafe { *p.add(len) } != 0 {
        len += 1;
    }

    len
}
fn file() {
    let mut matrix = [[0; MATRIX_COLS]; MATRIX_ROWS];
    let matrix_ptr: *mut [[i32; MATRIX_COLS]; MATRIX_ROWS] = &mut matrix;

    input_2d_array(matrix_ptr, MATRIX_ROWS, MATRIX_COLS);
    println!("{matrix:?}");

    let mut animals = [[0_u8; ANIMAL_NAME_SIZE]; ANIMAL_COUNT];

    let file = File::open("animal.txt").unwrap();
    let mut reader = BufReader::new(file);

    input_strings(animals.as_mut_ptr(), animals.len(), &mut reader);
    print_strings(&animals);

    let a = "rabbit";
    let p = &"rabbir".as_ptr();
}
fn print_strings<const ROWS: usize, const COLS: usize>(strings: &[[u8; COLS]; ROWS]) {
    for row in strings {
        for &ch in row {
            if ch == 0 {
                break;
            }

            print!("{}", ch as char);
        }
        println!();
    }
}

fn pointer_array() {
    // as_ptr = 배열/슬라이스의 첫 번째 원소를 가리키는 raw pointer를 얻는 것
    let a1: [i32; 4] = [0; 4];
    let a2: [*const i32; 4] = [std::ptr::null(); 4];
    let a3: [[i32; 4]; 3] = [[0; 4]; 3];
    let a4: [[[i32; 4]; 3]; 2] = [[[0; 4]; 3]; 2];
    let a5: [*const [i32; 4]; 3] = [std::ptr::null(); 3];

    let _p1: *const i32 = a1.as_ptr();
    let _p2: *const *const i32 = a2.as_ptr();
    let _p3: *const [i32; 4] = a3.as_ptr();
    let _p4: *const [[i32; 4]; 3] = a4.as_ptr();
    let _p5: *const *const [i32; 4] = a5.as_ptr();

    let array3d = [[[0; 4]; 3]; 2];
    let row_pointers: [*const [i32; 4]; 3] = [ptr::null(); 3];
    let callbacks: [Option<fn(*mut i32) -> *mut i32>; 2] = [None; 2];
    let element_pointers: [[*const i32; 4]; 3] = [[ptr::null(); 4]; 3];
    let array_callbacks: [Option<fn() -> *const [i32; 4]>; 5] = [None; 5];

    let _array3d_ptr = array3d.as_ptr();
    let _row_pointers_ptr = row_pointers.as_ptr();
    let _callbacks_ptr = callbacks.as_ptr();
    let _element_pointers_ptr = element_pointers.as_ptr();
    let _array_callbacks_ptr = array_callbacks.as_ptr();
    let _null_pointer: *mut i32 = ptr::null_mut();
    let _identity_callbacks = [identity_pointer; 2];
    let _same_callbacks_ptr = callbacks.as_ptr();
}

fn identity_pointer(pointer: *mut i32) -> *mut i32 {
    pointer
}

fn input_strings(animals: *mut [u8; ANIMAL_NAME_SIZE], count: usize, reader: &mut BufReader<File>) {
    for i in 0..count {
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        let input = input.trim().as_bytes();

        unsafe {
            let row = &mut *animals.add(i);
            for (j, &ch) in input.iter().take(ANIMAL_NAME_SIZE - 1).enumerate() {
                row[j] = ch;
            }
        }
    }
}

fn input_2d_array(array: *mut [[i32; MATRIX_COLS]; MATRIX_ROWS], rows: usize, columns: usize) {
    for i in 0..rows {
        for j in 0..columns {
            let mut input = String::new();

            std::io::stdin().read_line(&mut input).unwrap();

            let value: i32 = input.trim().parse().unwrap();

            unsafe {
                (*array)[i][j] = value;
            }
        }
    }
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

    unsafe {
        println!("{:p} {:p}", ap, ap.add(1));
    }

    unsafe {
        println!(
            "{} {} {}",
            size_of_val(&ap),       // C: sizeof(ap)
            size_of_val(&*ap),      // C: sizeof(*ap)
            size_of_val(&(*ap)[0]), // C: sizeof(**ap)
        );
    }
    /* ---------------------------------------------------------
       a + 1을 따라가 보기
       ---------------------------------------------------------

       a
       ↓
       &a[0]

       a + 1
       ↓
       &a[1]

       *(a + 1)
       ↓
       a[1]
       ↓ 배열이 표현식에서 int *로 변환
       &a[1][0]

       *(a + 1) + 2
       ↓
       &a[1][2]

       *(*(a + 1) + 2)
       ↓
       a[1][2]
    */
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
