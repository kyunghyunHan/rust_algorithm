use std::io::{BufReader,BufRead,BufWriter,Write,stdin,stdout};

pub fn main(){
    let mut reader= BufReader::new(stdin().lock());
    let mut writer= BufWriter::new(stdout().lock());
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let n :u64= input.trim().parse().unwrap();
    input.clear();
    reader.read_line(&mut input).unwrap();
    let mut nums= input.trim().split_ascii_whitespace().map(|x|x.parse().unwrap()).collect::<Vec<u64>>();
    let mut tmp = vec![0; n as usize];
    let mut result: u64 = 0;
    merge_sort(&mut nums, &mut tmp, 1, n as usize, &mut result);
    writeln!(writer,"{}",result).unwrap();
    writer.flush().unwrap();
}
fn merge_sort(arr: &mut Vec<u64>, tmp: &mut Vec<u64>, s: usize, e: usize, result: &mut u64) {
    if e <= s {
        return;
    }
    let m = s + (e - s) / 2;
    merge_sort(arr, tmp, s, m, result);
    merge_sort(arr, tmp, m + 1, e, result);

    let mut k = s;
    let mut index1 = s;
    let mut index2 = m + 1;

    while index1 < m && index2 < e {
        if arr[index1 ] > arr[index2 ] {
            tmp[k  ] = arr[index2 ];
            *result += (index2 - k) as u64;
            k += 1;
            index2 += 1;
        } else {
            tmp[k] = arr[index1];
            k += 1;
            index1 += 1;
        }
    }

    while index1 <= m {
        tmp[k] = arr[index1];
        k += 1;
        index1 += 1;
    }
    
    while index2 < e {
        tmp[k ] = arr[index2 ];
        k += 1;
        index2 += 1;
    }

    for i in s..e {
        arr[i ] = tmp[i ];
    }
}