
// Time is the running just that function, all are same.
fn main() {
    //time: user=0.02s system=0.01s cpu=98% total=0.030
    prop();
    //time: user=0.03s system=0.00s cpu=99% total=0.030
    prop_alt();
    //time: user=0.03s system=0.00s cpu=99% total=0.030
    prop_alt_1();
    //time: user=0.03s system=0.00s cpu=98% total=0.030
    prop_alt_2();
    //time: user=0.01s system=0.02s cpu=98% total=0.029
    prop_alt_3();
    //time: user=0.01s system=0.01s cpu=99% total=0.030
    prop_alt_4();
    //time: user=0.03s system=0.00s cpu=98% total=0.030
    prop_alt_5();
}

const N: usize = 3;
// This did not strike me as idiomatic, because
// 1) 3 mut variables, normally want to limit use of mut
// 2) `while` instead of `for`, Rust loves iterators hence it is almost always more idiomatic to use
//    them.
// 3) Explict type declaration, normally we want to let Rust infer and not over-declare.
fn prop() {
    let m: [[i32; N]; N] = [[0, 1, 2], [3, 4, 5], [6, 7, 8]];
    let mut s: i32 = 0;
    let mut r: usize = 0;

    while r < N {
        let mut c = 0;
        while c <= r {
            println!("m[{}][{}] = {}", r, c, m[r][c]);
            s += m[r][c];
            c += 1;
        }
        r += 1;
    }
    println!("Sum over the SW quadrant of m: {}", s);
}
//Direct, but not idiomatic
fn prop_alt() {
    let m = [[0, 1, 2], [3, 4, 5], [6, 7, 8]];
    let mut s: usize = 0; //declaring type here, sets the inferences throught the function

    //Clippy will warn about using a range here, clippy can tell that `r` is an index for m.
    for r in 0..N {
        for c in 0..=r {
            //This change is more style than idiomatic.
            println!("m[{r}][{c}] = {}", m[r][c]);
            s += m[r][c];
        }
    }
    println!("Sum over the SW quadrant of m: {s}");
}

//Direct translation and idiomatic
fn prop_alt_1() {
    let m = [[0, 1, 2], [3, 4, 5], [6, 7, 8]];
    let mut s: usize = 0;

    //Better for readability, but could be better
    for (r, _) in m.iter().enumerate().take(N) {
        for c in 0..=r {
            println!("m[{r}][{c}] = {}", m[r][c]);
            s += m[r][c];
        }
    }
    println!("Sum over the SW quadrant of m: {s}");
}
//Requires some understanding of what the code is doing (the r+1 step) and slightly more idiomatic (maybe?)
fn prop_alt_2() {
    let m = [[0, 1, 2], [3, 4, 5], [6, 7, 8]];
    let mut s = 0usize; //not sure if its idiomatic or style, but prefer postfix typing of
                        //literal

    //Better because we no loner access via index, when should save bound checks
    for (r, n) in m.iter().enumerate().take(N) {
        for (c, p) in n.iter().enumerate().take(r + 1) {
            println!("m[{r}][{c}] = {}", p);
            s += p;
        }
    }
    println!("Sum over the SW quadrant of m: {s}");
}
//Not sure if this is idiomatic or style, but we have no declared `mut` vars.
fn prop_alt_3() {
    let m = [[0, 1, 2], [3, 4, 5], [6, 7, 8]];
    let s = m.iter().enumerate().take(N).fold(0usize, |acc, (r, n)| {
        acc + n.iter().enumerate().take(r + 1).fold(0usize, |a2, (c, p)| {
            println!("m[{r}][{c}] = {}", p);
            a2 + p
        })
    });

    println!("Sum over the SW quadrant of m: {s}");
}
// With a little more understanding of the code, we can remove the first take.
fn prop_alt_4() {
    let m = [[0, 1, 2], [3, 4, 5], [6, 7, 8]];
    //Since N == m.len(), we can just iterator over the whole array
    let s = m.iter().enumerate().fold(0usize, |acc, (r, n)| {
        acc + n.iter().enumerate().take(r + 1).fold(0usize, |a2, (c, p)| {
            println!("m[{r}][{c}] = {}", p);
            a2 + p
        })
    });

    println!("Sum over the SW quadrant of m: {s}");
}

// This how I would have written it first, alt_4 would have been a second pass
// refactor.
fn prop_alt_5() {
    let m = [[0, 1, 2], [3, 4, 5], [6, 7, 8]];
    let mut s = 0usize;

    for (r, n) in m.iter().enumerate() {
        for (c, p) in n.iter().enumerate().take(r + 1) {
            println!("m[{r}][{c}] = {}", p);
            s += p;
        }
    }
    println!("Sum over the SW quadrant of m: {s}");
}
