/*
insetion sort
- 적절한 위치에 삽입하는 정렬
- 두번쨰 자료부터 시작하여 그앞의 자료들과 비교하여 삽입할 위치를 찾아 삽입
*/

fn sort(arr: &mut Vec<i32>, n: usize) {
    for i in 1..n {//n
        let key = arr[i]; //현재 삽입될 정수 n-1
        //a[i]를 정렬된 배열 A[1:i-1]에 삽입한다
        let mut j = (i - 1) as i32;
        //현재 비교 대상이 배열의 시작을 넘어가지 않게 하고 정렬된 부분에서 
        //A[j] >key를 찾고 j가 0에 도달할떄 while문 종료
        println!("{}",j);
        println!("비교{}",arr[j as usize]);
        println!("key{}",key);
        //비교대상이 음수가 첫번쨰 배열보다 낮으면안대야 하며 비교대상이 키보다 크면 
        //내림차순으로 변경할려면 >key => <key 로변경
        while j >= 0 && arr[j as usize] > key {  
            println!("이거{}",arr[j as usize]);
            println!("{}",j + 1);

            arr[(j + 1) as usize] = arr[j as usize];
            j -= 1;//왼쪽으로 이동하면서 비교
        }
        println!("j는?{}",j);
        //while문이 종료되면 j는 key가 들어갈 위치보다 한칸 왼쪽에 위치
        arr[(j + 1) as usize] = key;
    }
}
pub fn example() {
    let mut arr = vec![5,2,4,6,1,3];
    let n = arr.len();
    sort(&mut arr, n);
    println!("{:?}", arr);
}
