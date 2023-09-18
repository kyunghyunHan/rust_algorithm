/*퀵정렬
- 기준값을 선정해 해당 값보다 작은 데이터와 큰 데이터로 분류하는 것을 반복해 정렬하는 알고리즘
- pivot을 중심으로 계속 데이터를 2개의 집합으로 나누면서 정렬하는 것이 퀵정렬의 핵ㅅ미
- 파티션 교환정렬
- 비교 기반 정렬 알고리즘
- 문제를 더 작은 하위 문제로 나누고 독립적으로 해결하고 원래 문제를 해결하기 위해 솔루션을 결합하는 분한정복 방법에 따라 작동함


=========순서============
1.배열에서 피벗 선택 ,피벗을 선택하기 위한 일반적인 전략에는 첫 번째 요소, 마지막 요소, 중간 요소 또는 임의의 요소를 선택하는 것이 포함,일부 고급 기술에는 작은 요소 샘플의 중앙값을 선택하는 작업이 포함
2.어레이를 두 개의 더 작은 하위 어레이로 분할,피벗보다 작거나 같은 요소 b. 피벗보다 큰 요소
3.두 하위 배열에 빠른 정렬 알고리즘을 재귀적으로 적용.,재귀의 기본 사례는 하나의 요소가 있는 배열이거나 정렬된 것으로 간주되는 빈 배열
4.정렬된 하위 배열을 결합하여 최종 정렬된 배열을 얻습니다. 첫 번째 하위 배열의 요소는 항상 두 번째 하위 배열의 요소보다 작거나 같기 때문에 이 단계에서는 추가 작업이 필요하지 않습니다. 조합은 재귀가 풀릴 때 자연스럽게 발생합니다
========================
=========순서============
1.데이터를 분할하는 pivot을 설정
- privot을 기준으로 다음 과정을 2개의 집합을 분리
- start가 가리키는 데이터가 pivot이 가리키는 데이터보다 작으면 start를 오른쪽으로 1칸 이동한다,
- end가 가리키는 데이터가 pivot이 가리키는 데이터보다 크면 end를 왼쪽으로 1칸 이동한다
- start가 가리키는 데이터가 pivot이 가리키는 데이터보다 크고 end가 가리키는 데이터가 pivot이 가리키는 데이터보다 작으면 start,end가 가리키는 데이터를 swap하고 start는 오른쪽 end는 왼쪽으로 1칸씩 이동
- start와end가 만날떄까지 반복
- 만 지점에서 가리키는 데이터와 pivot이 가리키는데이터를 비교하여 pivot이 가리키는 데이터가 크면 만난 지점의 오른쪽에 작으면 만난 지점의 왼쪽에 pivot이 가리키는 데이터를 삽입한다
- 분리 집합에서 각각 다시 pivot을 선정
- 분리 집합이 1개 이하가 될떄까지 반복
========================
-  o(n log n)
- 최악의 경우 O(N^2)(피벗 선택이 지속적으로 불균형 파티션을 초래할 때 발생)
- 최악의 시간 복잡도에도 불구하고 퀵 정렬은 더 작은 상수 요소와 더 나은 캐시 성능으로 인해 병합 정렬 및 힙 정렬과 같은 다른 정렬 알고리즘보다 실제로 더 빠른 경우가 많습니다.
 */




fn partition(a: &mut [usize], n: usize) -> usize {
    let mut left = 1;
    let mut right = n - 1;
    let pivot = a[0]; // 피벗을 a[0]로 설정

    while left <= right {
        //피벗보다 큰 값의 위치를 찾음
        while left < n && a[left] <= pivot {
            left += 1;
        }
        println!("{:?}",a);
        //피벗보다 작은 값의 위치를 찾음
        while right > 0 && a[right] > pivot {
            right -= 1;
        }
        if left < right {
            a.swap(left, right);
        }
    }
    a.swap(0, right);
    right
}

fn quick_sort(a: &mut [usize], n: usize) {
    if n > 1 {
        let pivot = partition(a, n);//두 부분배열로 분할
        quick_sort(&mut a[0..pivot], pivot);//왼쪽 부분배열에 대한 순한 호출
        quick_sort(&mut a[pivot + 1..n], n - pivot - 1);//오른쪽 부분배열에 대한 순한 호출
    }
}

pub fn main() {
    let mut a = [30, 35, 25, 55, 10, 50, 15, 45, 20 ,40];
    let n = 10;
    quick_sort(&mut a, n);
    println!("{:?}", a);
}

//최선 T(1)= O(1)


//각 정렬의 인쇄
pub fn example_main(){
    println!("Sort numbers ascending");
    let mut numbers:[isize;10] = [4, 65, 2, -31, 0, 99, 2, 83, 782, 1];
    println!("Before: {:?}", numbers);
    let numbers_len= numbers.len();
    _quick_sort(&mut numbers,0,numbers_len as isize -1);
    println!("After:  {:?}\n", numbers);

    println!("Sort strings alphabetically");
    let mut strings = ["beach", "hotel", "airplane", "car", "house", "art"];
    println!("Before: {:?}", strings);
    let  str_len= strings.len();
    _quick_sort(&mut strings,0,str_len as isize -1);
    println!("After:  {:?}\n", strings);
}



fn _quick_sort<T:PartialOrd>(arr:&mut [T],low:isize,high:isize){
    //low가 high보다 높거나 같으면 반환
      if low <high {
        //피벗 요소를 중심으로 배열을 분할허고 인덱스를 가져옵니다
        let p = partition2(arr, low,high);
        //피벗 요소의 왼쪽과 오른쪽에 있는 하위 배열을 재귀적으로 호출
        _quick_sort(arr, low, p-1);
        _quick_sort(arr,p+1, high);
    }
}
//변경 가능한 슬라이스,낮은 인덱스 및 높은 인덱스 사용
fn partition2<T:PartialOrd>(arr:&mut [T],low:isize,high:isize)->isize{
    //인덱스 에 있는 요소를 피벗 요소로 선택
    let pivot = high as usize;
    let mut store_index= low -1;
    let mut last_index = high;
//피벗보다 작은 요소가 앞에 오고 피벗보다 큰 요소가 뒤에 오도록 배열을 분할
    loop {
        store_index +=1;
        while arr[store_index as usize] <arr [pivot]{
            store_index +=1;
        }
        last_index -=1;
        while last_index >=0 && arr[last_index as usize]>arr[pivot]{
            last_index -=1;
        }
        if store_index >=last_index {
            break;
        }else {
            arr.swap(store_index as usize ,last_index as usize);
        }
    }

    arr.swap(store_index as usize, pivot as usize);
//피벗 요소의 최종 인덱스
    store_index
}