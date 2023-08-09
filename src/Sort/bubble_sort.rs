/*버블정렬
- 모든 인접한 두 값을 비교하여 왼쪽의 값이 더 큰 겨웅에는 자리를 바꾸는 과정을 반복해서 정렬하는 방식
- 입력 데이터에 상태에 따라 성능이 달라짐
o(n2)

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