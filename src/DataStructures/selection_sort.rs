const MAX_SIZE: usize = 5;

/*
선택정렬
- 제자리 정렬 알고리즘 중 하나
- 해당 순서에 원소를 넣을 위치는 이미 정해쟈 있고 어떤 원소를 넣을지 선택하는 알고리즘



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