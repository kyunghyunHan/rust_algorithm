
/*
이진탐색
- 정렬된 배열 형태로 주어진 원소들을 절반씩 줄여가면서 원하는 값을 찾는 방법

이진 탐색의 분할정복 과정 
분할:배열의 가운데 원소를 기준으로 왼쪽,오른쪽 부분 배열로 분할,탐색키와 가운데 원소가 같으면 반환 및 종료
정복:탐색키가 가운데 원소보다 작으면 왼쪽 부분배열을 대상으로 이진탐색을 순한혼출,크면 오른쪽 부분배열을 대상으로 이진 탐색을 순한 호출
결합:탐색 결과가 직접 반환되므로 결활 불핗요
 

성능:t(n)= logn
*/

/*순한형태 */
fn binary_search(arr: &[i32], target: i32, start: usize, end: usize) -> Option<usize> {
    if start > end {
        return None;//탐색실패
    }
    
    let mid = (start + end) / 2;//floor(소수점 버림)
    if arr[mid] == target {//탐색성공->인덱스 mid반환
        Some(mid)
    } else if arr[mid] < target {//왼쪽 부분배열
        binary_search(arr, target, mid + 1, end)
    } else {
        binary_search(arr, target, start, mid - 1)//오른쪽부분배열
    }
}

/*반복형태 */
fn binary_search2(a: &[i32], x: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = a.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;
        if a[mid] == x {//탐색성공
            return Some(mid);
        } else{//왼쪽
             if a[mid] < x {
                left = mid + 1;
            } else {//오른쪽
                right = mid - 1;
            }
        }
    }

    None//탐색실패
}
pub fn main(){

    let  test = [ 17, 28, 43, 67, 88, 92, 100];
    
    println!("{:?}",binary_search(&test,43,0 ,6));
    println!("{:?}",binary_search2(&test,28 ));
}