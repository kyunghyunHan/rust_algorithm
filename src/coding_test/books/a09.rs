use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};
/*
 Winter in ALGO Kingdom (문제 A09)

 문제 요약:
 ALGO 왕국의 H×W 격자 맵이 있다. 처음에는 어떤 건물도 눈이 쌓이지 않았다.
 위치 (i, j)에 있는 건물 Aij가 눈을 맞은 후, i보다 같거나 큰 모든 행과 j보다 같거나 큰 모든 열에 있는 건물 (r, c)는
 Cr, Dr로 오른쪽 아래로 이동하는 사각형 영역에 적설량이 1cm씩 증가한다.
 최종적인 각 지점의 적설량을 출력하는 프로그램을 작성하시오.

 입력
 5 5 2
 1 1 3 3
 2 2 4 4

 출력
 1 1 1 0 0
 1 2 2 1 0
 1 2 2 1 0
 0 1 1 1 0
 0 0 0 0 0
*/
pub fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut input = String::new();
    reader.read_line(&mut input).unwrap();

    let (h, w, n) = {
        let mut iter = input.split_whitespace();
        (
            iter.next().unwrap().parse::<i32>().unwrap(),
            iter.next().unwrap().parse::<i32>().unwrap(),
            iter.next().unwrap().parse::<i32>().unwrap(),
        )
    };
    let mut x = vec![vec![0; 1509]; 1509];
    let mut z = vec![vec![0; 1509]; 1509];
    let mut av = [0; 100009];
    let mut bv = [0; 100009];
    let mut cv = [0; 100009];
    let mut dv = [0; 100009];

    for i in 1..=n {
        input.clear();
        reader.read_line(&mut input).unwrap();
        let (a, b, c, d) = {
            let mut iter = input.split_whitespace();
            (
                iter.next().unwrap().parse::<i32>().unwrap(),
                iter.next().unwrap().parse::<i32>().unwrap(),
                iter.next().unwrap().parse::<i32>().unwrap(),
                iter.next().unwrap().parse::<i32>().unwrap(),
            )
        };
        av[i as usize] = a;
        bv[i as usize] = b;
        cv[i as usize] = c;
        dv[i as usize] = d;
    }

    for t in 1..=n {
        x[av[t as usize] as usize][bv[t as usize] as usize] += 1;
        x[av[t as usize] as usize][dv[t as usize] as usize + 1] -= 1;
        x[cv[t as usize] as usize + 1][bv[t as usize] as usize] -= 1;
        x[cv[t as usize] as usize + 1][dv[t as usize] as usize + 1] += 1;
    }

    for i in 0..=h {
        for j in 0..=w {
            z[i as usize][j as usize] = 0;
        }
    }
    for i in 1..=h {
        for j in 1..=w {
            z[i as usize][j as usize] = z[i as usize][j as usize - 1] + x[i as usize][j as usize]
        }
    }
    for i in 1..=w {
        for j in 1..=h {
            z[i as usize][j as usize] = z[i as usize - 1][j as usize] + z[i as usize][j as usize]
        }
    }

    for i in 1..=h {
        for j in 1..=w {
            if j >= 2 {
                write!(writer, " ").unwrap();
            }
            write!(writer, "{}", z[i as usize][j as usize]).unwrap();
        }
        writeln!(writer).unwrap();
    }
    writer.flush().unwrap();
}
