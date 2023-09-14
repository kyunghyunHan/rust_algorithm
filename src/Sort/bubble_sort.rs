/*버블정렬
- 두 인접한 데이터의 크기를 비교해 정렬하는 방법 시간복잡도 0(n^2)으로 다른 정렬 알고리즘 보다 속도가 느린편
- 모든 인접한 두 값을 비교하여 왼쪽의 값이 더 큰 겨웅에는 자리를 바꾸는 과정을 반복해서 정렬하는 방식
- 입력 데이터에 상태에 따라 성능이 달라짐
o(n2)

 */
/*
과정
1.비교연산이 필요한 루프 범위를 설정
2.인접한 데이터 값 비교
3.swap조건에 부합하면 swap연산을 수행
4.루프 범위가 끝날때까지 2~3을 수행
5.정렬 영역을 설정 다름 루프를 실행할떄는 이 영역을 제외
6.비교 대상이 없을 떄 까지 1~5 반복
 */
fn bubble_sort(a: &mut [usize]) {
    let n = a.len() as usize;
    for i in 0..n-1 {
        //ㅑ
        for j in 0..n-1-i {//왼쪽에서 오른쪽으로 진행
            //i를 뺴주는 -> 내부 for 문이 한사이클 마다 돌떄마다 가장 큰수 -i번째 큰수가 정렬됬기 떄문
            if a[j] > a[(j+1 )] {
                a.swap(j, j+1);
            }
        }
    }
}
//개선된 알고리즘
fn bubble_sort2(a: &mut [usize]) {
    let n: usize = a.len() as usize;
    for i in 0..n-1 {
        let mut exchange= false;
        for j in 0..n-1-i {//왼쪽에서 오른쪽으로 진행
            //i를 뺴주는 -> 내부 for 문이 한사이클 마다 돌떄마다 가장 큰수 -i번째 큰수가 정렬됬기 떄문
            if a[j] > a[(j+1 )] {
                a.swap(j, j+1);
                exchange= true;
            }
            if !exchange{break}
        }
    }
}
//PartialOrd:일반 유형에 대한 비교 기능을 특성

fn bubble_sort3<T: PartialOrd>(arr: &mut [T]) {
    let len = arr.len();
    let mut swapped;

    loop {
        //반복 요소가 교체되었는지 여부를 추적
        swapped = false;
   
   //더이상 스왑이 필요하지 않을떄 까지 루프를 사용하여 배열을 반복
         for i in 0..len - 1 {
            // Swap elements if they are out of order.
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }

        if !swapped {
            break;
        }
    }
}



pub fn main(){
    let mut list = [21, 10, 12, 20, 25, 13, 15, 22];
    bubble_sort2(&mut list);
        let mut nums = vec![5, 3, 1, 4, 2];

    bubble_sort3(&mut nums);

   println!("{:?}",list)
}