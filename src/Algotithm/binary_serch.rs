
/*이진 탐색의 분할정복 과정 
분할:배열의 가운데 원소를 기준으로 왼쪽,오른쪽 부분 배열로 분할,탐색키와 가운데 원소가 같으면 반환 및 종료
정복:탐색키가 가운데 원소보다 작으면 왼쪽 부분배열을 대상으로 이진탐색을 순한혼출,크면 오른쪽 부분배열을 대상으로 이진 탐색을 순한 호출

결합:탐색 결과가 직접 반환되므로 결활 불핗요


*/

/*순한형태 */
pub fn binary_search(a:vec<i32>,left:i32,right:i32,x:i32)->i32{
    if left>right {
        -1
    }
    let mid= 0;

    if x==a[mid] {
        mid
    }

    else{
        if x<mid{

        }else{

        }
    }

}
/*반복형태 */
pub fn binary_search2(){}