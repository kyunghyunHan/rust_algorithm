pub struct Array<T> {
    data: Vec<T>, // 배열을 위한 필드들이 여기에 들어갈 예정
}

impl<T: PartialEq + Clone> Array<T> {
    // 새 배열 생성
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    // 특정 용량으로 배열 생성
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            data: Vec::with_capacity(capacity),
        }
    }

    // 배열이 비어있는지 확인
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    // 배열의 크기 반환
    pub fn size(&self) -> usize {
        self.data.len()
    }

    // 배열의 용량 반환
    pub fn capacity(&self) -> usize {
        self.data.capacity()
    }

    // 특정 인덱스의 값 참조 반환
    pub fn at(&self, index: usize) -> Option<&T> {
        self.data.get(index)
    }

    // 배열의 첫 요소 참조 반환
    pub fn front(&self) -> Option<&T> {
        self.data.first()
    }

    // 배열의 마지막 요소 참조 반환
    pub fn back(&self) -> Option<&T> {
        self.data.last()
    }

    // 배열의 처음에 값 추가
    pub fn push_front(&mut self, value: T) {
        self.data.insert(0, value);
    }

    // 배열의 끝에 값 추가
    pub fn push_back(&mut self, value: T) {
        self.data.push(value);
    }

    // 배열의 처음 값 제거 및 반환
    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(self.data.remove(0))
        }
    }

    // 배열의 마지막 값 제거 및 반환
    pub fn pop_back(&mut self) -> Option<T> {
        self.data.pop()
    }

    // 특정 인덱스에 값 삽입
    pub fn insert(&mut self, index: usize, value: T) -> Result<(), &'static str> {
        if index > self.size() {
            return Err("Index out of bounds");
        }
        self.data.insert(index, value);
        Ok(())
    }

    // 특정 인덱스의 값 삭제
    pub fn erase(&mut self, index: usize) -> Option<T> {
        if index < self.size() {
            Some(self.data.remove(index))
        } else {
            None
        }
    }

    // 특정 값과 일치하는 첫 번째 요소 삭제
    pub fn remove(&mut self, value: &T) -> bool {
        if let Some(pos) = self.data.iter().position(|x| x == value) {
            self.data.remove(pos);
            true
        } else {
            false
        }
    }

    // 배열의 모든 요소 삭제
    pub fn clear(&mut self) {
        self.data.clear();
    }

    // 배열 뒤집기
    pub fn reverse(&mut self) {
        self.data.reverse();
    }

    // n번째 요소로부터 끝까지의 값 가져오기
    pub fn value_n_from_end(&self, n: usize) -> Option<&T> {
        if n == 0 || n > self.size() {
            return None;
        }
        self.data.get(self.size() - n)
    }

    // 벡터로 변환
    pub fn to_vec(&self) -> Vec<T> {
        self.data.clone()
    }
}

// 사용 예시 함수
pub fn example() {
    let mut arr = Array::new();

    arr.push_back(1);
    arr.push_back(2);
    arr.push_back(3);
    arr.push_front(0);

    println!("Array: {:?}", arr.to_vec()); // [0, 1, 2, 3]
    println!("Front: {:?}", arr.front()); // Some(0)
    println!("Back: {:?}", arr.back()); // Some(3)

   
    arr.insert(2, 99).unwrap();
    println!("After insert: {:?}", arr.to_vec()); // [0, 1, 99, 2, 3]

    arr.erase(2);
    println!("After erase: {:?}", arr.to_vec()); // [0, 1, 2, 3]

    arr.remove(&2);
    println!("After remove 2: {:?}", arr.to_vec()); // [0, 1, 3]

    arr.reverse();
    println!("Reversed: {:?}", arr.to_vec()); // [3, 1, 0]

    println!("Value 2nd from end: {:?}", arr.value_n_from_end(2)); // Some(1)

    arr.clear();
    println!("After clear: {:?}", arr.to_vec()); // []


}
