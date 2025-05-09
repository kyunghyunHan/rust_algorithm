pub fn example() {
    // 8x8 체스판 크기
    let n = 8;

    // 체스판을 -1로 초기화 (방문하지 않은 상태)
    let mut board = vec![vec![-1; n]; n];

    // 기사의 이동 방향 (8가지 L자 이동)
    let move_x = [2, 1, -1, -2, -2, -1, 1, 2];
    let move_y = [1, 2, 2, 1, -1, -2, -2, -1];

    // 시작 위치를 (0, 0)으로 설정하고 첫 번째 이동을 0으로 표시
    board[0][0] = 0;

    // 백트래킹으로 기사의 여행 시작
    if solve_knights_tour(&mut board, 0, 0, 1, n.try_into().unwrap(), &move_x, &move_y) {
        // 해결책 출력
        print_solution(&board, n.try_into().unwrap());
    } else {
        println!("해결책이 존재하지 않습니다.");
    }
}

// 백트래킹을 사용하여 기사의 여행 문제 해결
fn solve_knights_tour(
    board: &mut Vec<Vec<i32>>,
    curr_x: i32,
    curr_y: i32,
    move_idx: i32,
    n: i32,
    move_x: &[i32; 8],
    move_y: &[i32; 8],
) -> bool {
    // 모든 칸을 방문했다면 성공
    if move_idx == n * n {
        return true;
    }

    // 8가지 가능한 이동 방향을 시도
    for i in 0..8 {
        // 다음 위치 계산
        let next_x = curr_x + move_x[i];
        let next_y = curr_y + move_y[i];

        // 다음 위치가 유효하고 아직 방문하지 않았는지 확인
        if is_safe(next_x, next_y, board, n) {
            // 이동 기록
            board[next_x as usize][next_y as usize] = move_idx;

            // 다음 이동 재귀적으로 시도
            if solve_knights_tour(board, next_x, next_y, move_idx + 1, n, move_x, move_y) {
                return true;
            }

            // 이 경로가 해결책으로 이어지지 않으면 백트래킹
            board[next_x as usize][next_y as usize] = -1;
        }
    }

    // 모든 이동을 시도했지만 해결책을 찾지 못함
    false
}

// 위치가 유효한지 확인하는 함수
fn is_safe(x: i32, y: i32, board: &Vec<Vec<i32>>, n: i32) -> bool {
    // 체스판 내부에 있고 아직 방문하지 않았는지 확인
    x >= 0 && x < n && y >= 0 && y < n && board[x as usize][y as usize] == -1
}

// 체스판 출력 함수
fn print_solution(board: &Vec<Vec<i32>>, n: i32) {
    println!("기사의 여행 해결책:");
    for i in 0..n as usize {
        for j in 0..n as usize {
            // 3자리 공간에 숫자 정렬하여 출력
            print!("{:3} ", board[i][j]);
        }
        println!();
    }
}
