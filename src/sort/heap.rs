fn parent(i: usize) -> usize {
    i / 2
}

fn left(i: usize) -> usize {
    return 2 * i;
}
fn right(i: usize) -> usize {
    2 * i + 1
}
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

fn build_max_heap(arr: &mut [usize], n: usize) {
    for i in (0..n / 2).rev() {
        max_heapify(arr, n, i);
    }
}
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
