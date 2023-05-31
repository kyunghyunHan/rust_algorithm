
/*선택되지 않는 노드중 최소거리를 가진 노드를 찾기
1부터 V까지의 모든 노드 순회
 */

fn find_shortest_node(select:&[bool],dist:&[i32],v:usize)->usize{
let mut min_dist= std::i32::MAX;
let mut min_idx= 0;

for i in 1..=v{
    //노드가 선택된 경우 다음 순회로넘어가기
    if select[i]{continue;
    }

    if dist[i]<min_dist{
        min_dist= dist[i];
        min_idx= i;
    }
}
min_idx
}


fn update_dist(new_node:usize,select:&[bool],dist:&[i32],map:&[Vec<i32>]){

    let v= select.len();
    for i in 1..=v{
        if select[i]{
            continue;;
        }

        
    }
}

fn dijkstra(start: usize, select: &mut [bool], dist: &mut [i32], map: &[Vec<i32>]){
}

pub fn main(){


    
}