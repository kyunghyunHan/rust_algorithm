const MAX_SIZE: usize = 5;

/*
선택정렬
- 주어진 데이터 중에서 가장 작은 값부터 차례대로 선택해서 나열하는 방식
- 제자리 정렬 알고리즘 중 하나
- 해당 순서에 원소를 넣을 위치는 이미 정해쟈 있고 어떤 원소를 넣을지 선택하는 알고리즘
- 대상 데이터에서 최대나 최소 데이터를 데이터가 나열된 순으로 찾아가며 선택하는 방법
- 구현방법이복잡 시간복잡도 O(n^2)


- 구현과정
- 남은 정렬에서 최솟값 또는 최대값을 찾는다
- 남은 정렬 부분에서 가장 앞에 있는 데이터와 선택된 데이터를 swap
- 가장 앞에 잇는 데이터의 위치를 변경해 남은 정렬 부분의 범위를 축소
- 전체 데이터 크기만큼 index가 커질떄까지 즉 남은 정렬 부분이 없을 떄까지 반복
 */
fn selection_sort(list: &mut [i32]) {

    //마지막 숫자는 자동으로 정렬되기때문에 숫자-1만큼 반복
    for i in 0..list.len() - 1 {
        let mut least = i;


        //최솟값 탐색
        for j in i + 1..list.len() {
            if list[j] < list[least] {
                least = j;
            }
        }

        //최솟값이 자기 자신이면 자료이동을 하지 않는다.
        if i != least {
            list.swap(i, least);
        }
    }
}

pub fn main() {
    let mut list = [9, 6, 7, 3, 5];

    // 선택 정렬 수행
    selection_sort(&mut list);

    // 정렬 결과 출력
    for i in 0..list.len() {
        println!("{}", list[i]);
    }
}

/*
시간복잡도

 */