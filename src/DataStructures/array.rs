

/* 
배열
- 같은 타입의 데이터를 여러개 나열한 선형 자료구조
- 연속적인 메모리 공간에 순차적으로 데이터를 저장
- 배열은 선언할떄 크기를 정하면 그 크기로 저장
- 선언된 값을 다시 배열을 선언하지 않으면 변경할수 없다.
- 배열은 index를 통해서 배열에 있는 요소에 접근 할수 있다.
- index는 0부터 시작 마지막 인덱스는 배열의 요수의 개수 -1이다
- 시간 복잡도 O(1)
- 데이터를 배열에 삽입을 하려면 기존에 있는 데이터를 한 칸 shift 한 후 데이터를 넣어야 하기에 시간복잡도는 O(n)이 걸린다.

- 구현이 쉽다
- 메모리관리가 편하다 연속적이므로
- 검색이빠르다

- 단점
- 배열의 크기를 변경할수 없다(배열내의 데이터 이동 및  재구성이 어렵다)
- 메모리 낭비가 발생하게 된다.
*/
struct Array {
    arr: Vec<i32>, // Vector to store the elements
}

impl Array {
    // Constructor to create a new Array
    fn new(size: usize) -> Self {
        let arr = vec![0; size]; // Initialize the vector with zeros
        Array { arr }
    }

    // Get the element at a specific index
    fn at(&self, idx: usize) -> Option<i32> {
        if idx < self.arr.len() {
            Some(self.arr[idx])
        } else {
            None
        }
    }

    // Add a value at a specific index
    fn add(&mut self, idx: usize, value: i32) {
        if idx <= self.arr.len() {
            self.arr.insert(idx, value);
        } else {
            println!("-1");
        }
    }

    // Remove the element at a specific index
    fn remove(&mut self, idx: usize) {
        if idx < self.arr.len() {
            self.arr.remove(idx);
        }
    }

    // Set the value of an element at a specific index
    fn set(&mut self, idx: usize, value: i32) {
        if idx < self.arr.len() {
            self.arr[idx] = value;
        }
    }

    // Print all elements in the array
    fn print(&self) {
        for &element in &self.arr {
            print!("{} ", element);
        }
        println!();
    }
}

fn main() {
    let mut ar = Array::new(5);
    ar.add(1, 10); // 0 10 0 0 0
    ar.add(1, 20); // 0 20 10 0 0
    ar.print();    // 0 20 10 0 0

    ar.remove(1);  // 0 
}
// fn main() {
//     let mut arr: [i32; 3] = [0, 0, 0];
//     arr[1] = 1;
//     arr[2] = 2;
//     for element in arr.iter() {
//         println!("{}", element);
//     }
// }