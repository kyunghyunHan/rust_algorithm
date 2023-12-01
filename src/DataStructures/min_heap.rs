const MAX_SIZE: usize = 100000;

struct MinHeap {
    data: Vec<i32>,
    heap_size: usize,
}

impl MinHeap {
    // 힙 초기화 함수
    fn new() -> Self {
        MinHeap {
            data: vec![0; MAX_SIZE],
            heap_size: 0,
        }
    }

    // 데이터 추가 함수
    fn add(&mut self, data: i32) {
        // 맨 마지막에 데이터를 추가
        self.heap_size += 1;
        self.data[self.heap_size] = data;

        let mut me = self.heap_size;
        let mut parent = me / 2;

        // 부모가 자식보다 값이 작은지 확인하고 위치 교환
        while parent > 0 {
            if self.data[parent] > self.data[me] {
                self.data.swap(parent, me);
                me = parent;
                parent = me / 2;
            } else {
                break;
            }
        }
    }

    // 힙이 비어 있는지 확인하는 함수
    fn is_empty(&self) -> bool {
        self.heap_size == 0
    }

    // 데이터 삭제 함수
    fn remove(&mut self) -> Option<i32> {
        if self.is_empty() {
            return None;
        }

        // 최소값 반환하고 삭제
        let res = Some(self.data[1]);

        self.data[1] = self.data[self.heap_size];
        self.heap_size -= 1;

        let mut me = 1;
        let mut child = 2;

        // 자식의 값과 비교하여 위치 교환
        while child <= self.heap_size {
            if child + 1 <= self.heap_size && self.data[child] > self.data[child + 1] {
                child += 1;
            }

            if self.data[me] > self.data[child] {
                self.data.swap(me, child);
                me = child;
                child = me * 2;
            } else {
                break;
            }
        }

        res
    }
}

pub fn main() {
    let mut h = MinHeap::new();

    h.add(3);
    h.add(5);
    h.add(1);
    h.add(23);
    h.add(9);

    while let Some(value) = h.remove() {
        print!("{} ", value);
    }
}