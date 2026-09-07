use std::ptr::{self, null_mut};

type ElementType = i32;
//Node
struct Node {
    data: ElementType,
    next_node: *mut Node,
}
//새로운 노드
fn create_node(new_data: ElementType) -> *mut Node {
    let new_node = Box::new(Node {
        data: new_data,
        next_node: ptr::null_mut(),
    });
    Box::into_raw(new_node)
}
//node 제거
fn destory_node(node: *mut Node) {
    unsafe {
        if !node.is_null() {
            drop(Box::from_raw(node));
        }
    }
}
/*
head가 가리키는 노드가 NULL이면
새 노드를 head로 설정한다.

기존 노드가 있다면
tail이 head를 가리키게 한 뒤,

tail의 next_node가 NULL이 아닐 때까지
tail을 다음 노드로 이동한다.

마지막 노드를 찾으면
tail의 next_node가 new_node를 가리키게 한다.
*/
fn append_node(head: *mut *mut Node, new_node: *mut Node) {
    unsafe {
        if *head == ptr::null_mut() {
            *head = new_node;
        } else {
            let mut tail = *head;
            while (*tail).next_node != ptr::null_mut() {
                tail = (*tail).next_node;
            }
            (*tail).next_node = new_node;
        }
    }
}

/*
몇 번째 노드를 가져올지 나타내는 index를 받아
해당 위치의 노드 주소를 반환
*/
fn get_node_at(head: *mut Node, mut location: i32) -> *mut Node {
    //첫번째 노드부터 탐색
    let mut current = head;
    unsafe {
        /*
        current가 NULL이 아니고,
        아직 이동해야 할 횟수(location)가 남아 있다면

        location을 1 감소시키고
        current를 다음 노드로 이동한다.

        location이 0이 되면 원하는 위치에 도착한 것이므로
        반복을 종료하고 현재 노드의 주소를 반환한다.
        */
        while current != null_mut() && location > 0 {
            location -= 1;
            current = (*current).next_node;
        }
        return current;
    }
}

/*
remove 노드를 연결 리스트에서 제거한다.

1. remove가 head라면
   → head를 remove의 다음 노드로 변경한다.

2. remove가 head가 아니라면
   → head부터 시작해서
   → next_node가 remove를 가리키는 노드를 찾는다.
   → 찾으면 remove를 건너뛰고
      remove의 다음 노드를 직접 연결한다.
*/
fn remove_node(head: *mut *mut Node, remove: *mut Node) {
    unsafe {
        // 삭제할 노드가 현재 head라면
        if *head == remove {
            // head를 삭제할 노드의 다음 노드로 변경
            *head = (*remove).next_node;
        } else {
            // head부터 탐색 시작
            let mut current = *head;

            // current의 다음 노드가 remove가 될 때까지 이동
            while current != null_mut() && (*current).next_node != remove {
                current = (*current).next_node;
            }

            // remove 바로 이전 노드를 찾았다면
            if current != null_mut() {
                // remove를 건너뛰고 다음 노드와 연결
                (*current).next_node = (*remove).next_node;
            }
        }
    }
}
/*
current뒤에 new_node를 끼워넣는 함수
*/
fn insert_after(current: *mut Node, new_node: *mut Node) {
    unsafe {
        (*new_node).next_node = (*current).next_node;
        (*current).next_node = new_node;
    }
}
/*새로운 노드를 새로운 head로 만드는 */
fn insert_new_head(head: *mut *mut Node, new_head: *mut Node) {
    unsafe {
        if (*head) == null_mut() {
            (*head) = new_head;
        } else {
            (*new_head).next_node = (*head);
            (*head) = new_head;
        }
    }
}
/*노드 카운터 */
fn get_node_count(head: *mut Node) -> i32 {
    unsafe {
        let mut cnt = 0;
        let mut current = head;

        while current != null_mut() {
            current = (*current).next_node;
            cnt += 1;
        }
        cnt
    }
}
pub fn example() {
    unsafe {
        let mut list: *mut Node = null_mut();
        let mut current: *mut Node;
        let mut new_node: *mut Node;

        // 0 ~ 4 노드 추가
        for i in 0..5 {
            //새로운 노드를 만들어서 계속 추가
            new_node = create_node(i);
            append_node(&mut list, new_node);
        }

        // head에 -1 추가
        new_node = create_node(-1);
        insert_new_head(&mut list, new_node);

        // head에 -2 추가
        new_node = create_node(-2);
        insert_new_head(&mut list, new_node);

        // 리스트 출력
        //cnt =
        let mut cnt = get_node_count(list);
        //리스트룰 출력
        for i in 0..cnt {
            current = get_node_at(list, i);
            //current 가 null이니면
            if !current.is_null() {
                println!("List[{}] : {}", i, (*current).data);
            }
        }

        // 2번 노드 뒤에 3000 삽입
        println!("\nInserting 3000 After [2]...\n");

        current = get_node_at(list, 2);
        new_node = create_node(3000);

        if !current.is_null() {
            insert_after(current, new_node);
        }

        // 다시 출력
        cnt = get_node_count(list);

        for i in 0..cnt {
            current = get_node_at(list, i);

            if !current.is_null() {
                println!("List[{}] : {}", i, (*current).data);
            }
        }

        // 모든 노드 제거
        println!("\nDestroying List..");

        for _ in 0..cnt {
            current = get_node_at(list, 0);

            if !current.is_null() {
                remove_node(&mut list, current);
                destory_node(current);
            }
        }

        println!("count = {}", get_node_count(list));
    }
}
