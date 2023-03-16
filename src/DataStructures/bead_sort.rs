pub fn bead_sort(a:&mut [usize])->&mut [usize]{

    let mut max= a[0];
    for i in 1..a.len(){
        if a[i]>max{
            max= a[i];
        }
    }

    let mut beads = vec![vec![0; max]; a.len()];
    for i in 0..a.len() {
        for j in (0..a[i]).rev() {
            beads[i][j] = 1;
        }
    }

    for j in  0..max{
        let mut sum =0;


        for i in 0..a.len(){
            sum +=beads[i][j];
            beads[i][j]=0;
        }
        for k in ((a.len()-sum)..a.len()).rev(){
            a[k]=j+1;
        }
    }
    a
    
}