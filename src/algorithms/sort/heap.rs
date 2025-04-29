/*
힙정렬
- o(n log n)
- 내부정렬
- 힙을 사용
- 힙정렬 알고리즘에서는 최대힙을 사용


이진 힙 자료구조
- 가장 낮은 레벨 빼고는 완전히 차있고
- 가장 낮은 왼쪾부터 채움


*/


fn parent(i: usize) -> usize {
    i / 2
}

fn left(i: usize) -> usize {
    return 2 * i;
}
fn right(i: usize) -> usize {
    2 * i + 1
}
//최대힙 특성을 유지하는데 핵심역활
fn max_heapify(arr: &mut [usize], n: usize, i: usize) {
    let l = left(i);
    let r = right(i);
    let mut largest = i;

    if l < n && arr[l] > arr[i] {
        largest = l;
    }
    if r < n && arr[r] > arr[largest] {
        largest = r;
    }

    if largest != i {
        arr.swap(i, largest);
        max_heapify(arr, n, largest);
    }
}
//정렬되지 않은 입력배열로 부터 최대힙 생성
fn build_max_heap(arr: &mut [usize], n: usize) {
    for i in (0..n / 2).rev() {
        max_heapify(arr, n, i);
    }
}
//배열을 내부 정렬
fn heapsort(arr: &mut [usize]) {
    let mut n = arr.len();
    build_max_heap(arr, n);
    
    for i in (1..n).rev() {
        arr.swap(0, i);
        n -= 1;
        max_heapify(arr, n, 0);
    }
}
pub fn example() {
    let mut a:[usize;10] =[16,14,10,8,7,9,3,2,4,1];
    heapsort(&mut a);
    println!("{:?}",a);
}
