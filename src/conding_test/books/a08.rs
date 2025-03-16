use std::io::{stdin, stdout, BufRead, BufReader, BufWriter, Write};

/*Two Dimensional sum

*/
fn main() {
    let mut reader = BufReader::new(stdin().lock());
    let mut writer = BufWriter::new(stdout().lock());

    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let parts: Vec<i32> = line
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let (h, w) = (parts[0], parts[1]);

    // Initialize X matrix
    let mut x: Vec<Vec<i32>> = vec![vec![0; 1509]; 1509];

    // Read X values
    for i in 1..=h {
        line.clear();
        reader.read_line(&mut line).unwrap();
        let values: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        for j in 1..=w {
            x[i as usize][j as usize] = values[(j - 1) as usize];
        }
    }

   
    line.clear();
    reader.read_line(&mut line).unwrap();
    let q: i32 = line.trim().parse().unwrap();

    let mut a: Vec<i32> = vec![0; 100009];
    let mut b: Vec<i32> = vec![0; 100009];
    let mut c: Vec<i32> = vec![0; 100009];
    let mut d: Vec<i32> = vec![0; 100009];

    for i in 1..=q {
        line.clear();
        reader.read_line(&mut line).unwrap();
        let values: Vec<i32> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        a[i as usize] = values[0];
        b[i as usize] = values[1];
        c[i as usize] = values[2];
        d[i as usize] = values[3];
    }

    let mut z: Vec<Vec<i32>> = vec![vec![0; 1509]; 1509];

    for i in 1..=h {
        for j in 1..=w {
            z[i as usize][j as usize] = 0;
        }
    }

    for i in 1..=h {
        for j in 1..=w {
            z[i as usize][j as usize] = z[(i - 1) as usize][j as usize] + x[i as usize][j as usize];
        }
    }

    for i in 1..=h {
        for j in 1..=w {
            z[i as usize][j as usize] = z[i as usize][(j - 1) as usize] + z[i as usize][j as usize];
        }
    }

    for i in 1..=q {
        let result = z[c[i as usize] as usize][d[i as usize] as usize]
            - z[c[i as usize] as usize][(a[i as usize] - 1) as usize]
            - z[(b[i as usize] - 1) as usize][d[i as usize] as usize]
            + z[(b[i as usize] - 1) as usize][(a[i as usize] - 1) as usize];

        writeln!(writer, "{}", result).unwrap();
    }

    writer.flush().unwrap();
}
