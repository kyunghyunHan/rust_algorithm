
/*삽입정렬 
o(n2)
- 입력이 거의 정렬된 경우 다른 어떤 정렬보다 빠름

*/

fn insertion_sort(arr: &mut [i32]) {
    for i in 1..arr.len() {
        let val = arr[i];
        let mut j = i;
        while j > 0 && arr[j - 1] > val {
            arr[j] = arr[j - 1];
            j -= 1;
        }
        arr[j] = val;
    }
}


pub fn main(){}