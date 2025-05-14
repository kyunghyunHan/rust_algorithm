pub struct Array {
    // 배열을 위한 필드들이 여기에 들어갈 예정
}

impl Array {
    // 새 배열 생성
    pub fn new() {}

    // 특정 용량으로 배열 생성
    pub fn with_capacity(capacity: usize) {}

    // 배열이 비어있는지 확인
    pub fn is_empty(&self) {}

    // 배열의 크기 반환
    pub fn size(&self) {}

    // 배열의 용량 반환
    pub fn capacity(&self) {}

    // 특정 인덱스의 값 참조 반환
    pub fn at(&self, index: usize) {}

    // 배열의 첫 요소 참조 반환
    pub fn front(&self) {}

    // 배열의 마지막 요소 참조 반환
    pub fn back(&self) {}

    // 배열의 처음에 값 추가
    pub fn push_front(&mut self) {}

    // 배열의 끝에 값 추가
    pub fn push_back(&mut self) {}

    // 배열의 처음 값 제거 및 반환
    pub fn pop_front(&mut self) {}

    // 배열의 마지막 값 제거 및 반환
    pub fn pop_back(&mut self) {}

    // 특정 인덱스에 값 삽입
    pub fn insert(&mut self, index: usize) {}

    // 특정 인덱스의 값 삭제
    pub fn erase(&mut self, index: usize) {}

    // 특정 값과 일치하는 첫 번째 요소 삭제
    pub fn remove(&mut self) {}

    // 배열의 모든 요소 삭제
    pub fn clear(&mut self) {}

    // 배열 뒤집기
    pub fn reverse(&mut self) {}

    // n번째 요소로부터 끝까지의 값 가져오기
    pub fn value_n_from_end(&self, n: usize) {}

    // 벡터로 변환
    pub fn to_vec(&self) {}
}

// 사용 예시 함수
pub fn example() {
    // 사용 예시 코드가 여기에 들어갈 예정
}
