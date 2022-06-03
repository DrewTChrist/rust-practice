pub fn fact(n: u32) -> u32 {
    if n > 0 {
        n * fact(n - 1)
    } else {
        1
    }
}

pub fn inter(s1: String, s2: String, mut str_buf: String, mut idx: usize) -> String {
    if idx < s1.len() && idx < s2.len() {
        str_buf.push_str(s1.get(idx..idx + 1).unwrap());
        str_buf.push_str(s2.get(idx..idx + 1).unwrap());
        idx += 1;
        inter(s1, s2, str_buf, idx)
    } else {
        if idx < s1.len() {
            str_buf.push_str(s1.get(idx..).unwrap());
        } else if idx < s2.len() {
            str_buf.push_str(s2.get(idx..).unwrap());
        }
        str_buf
    }
}

pub fn check_arr_c<
    T: Copy + std::cmp::PartialOrd + std::fmt::Display,
    const R: usize,
    const C: usize,
>(
    arr: [[T; C]; R],
    col: &[T; R],
    mut r: usize,
    mut c: usize,
) -> bool {
    if c < arr[0].len() {
        if r < arr.len() {
            if arr[r][c] == col[r] {
                r += 1;
                check_arr_c(arr, col, r, c)
            } else {
                c += 1;
                r = 0;
                check_arr_c(arr, col, r, c)
            }
        } else {
            true
        }
    } else {
        false
    }
}

pub fn check_arr_r<T: Copy + std::cmp::PartialOrd, const R: usize, const C: usize>(
    arr: [[T; C]; R],
    row: &[T; C],
    mut r: usize,
    mut c: usize,
) -> bool {
    if r < arr.len() {
        if c < arr[r].len() {
            if arr[r][c] == row[c] {
                c += 1;
                check_arr_r(arr, row, r, c)
            } else {
                r += 1;
                c = 0;
                check_arr_r(arr, row, r, c)
            }
        } else {
            true
        }
    } else {
        false
    }
}

pub fn check_arr<T: Copy + std::cmp::PartialOrd, const R: usize, const C: usize>(
    arr: [[T; C]; R],
    value: T,
    mut r: usize,
    mut c: usize,
) -> bool {
    if r < arr.len() {
        if c < arr[r].len() {
            if arr[r][c] == value {
                return true;
            }
            c += 1;
            return check_arr(arr, value, r, c);
        }
        c = 0;
        r += 1;
        check_arr(arr, value, r, c)
    } else {
        false
    }
}

pub fn rev_arr<T: Copy, const R: usize, const C: usize>(
    mut arr: [[T; C]; R],
    mut r: usize,
    mut c: usize,
) -> [[T; C]; R] {
    if r < arr.len() {
        if c < arr.len() {
            let tmp = arr[r][c];
            arr[r][c] = arr[c][r];
            arr[c][r] = tmp;
            c += 1;
            return rev_arr(arr, r, c);
        }
        r += 1;
        c = r;
        rev_arr(arr, r, c)
    } else {
        arr
    }
}

pub fn rev_str(mut s: String, mut beg: usize, mut end: usize) -> String {
    if beg < end {
        let begr = String::from(s.get(beg..beg + 1).unwrap());
        let endr = String::from(s.get(end..end + 1).unwrap());
        s.replace_range(beg..beg + 1, &endr);
        s.replace_range(end..end + 1, &begr);
        beg += 1;
        end -= 1;
        rev_str(s, beg, end)
    } else {
        s
    }
}

fn main() {
    let v: Vec<u32> = [1, 2, 3].into_iter().map(|x| x + 1).rev().collect();
    println!("{:#?}", v);
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_fact() {
        assert_eq!(6, fact(3));
        assert_eq!(5040, fact(7));
    }

    #[test]
    fn test_inter() {
        let s1 = String::from("hello");
        let s2 = String::from("there sir");
        assert_eq!("htehlelroe sir", &inter(s1, s2, String::default(), 0));
    }

    #[test]
    fn test_check_arr_c() {
        const ROWS: usize = 5;
        const COLUMNS: usize = 10;

        let mut arr: [[u8; COLUMNS]; ROWS] = [[0; COLUMNS]; ROWS];
        let mut arr2: [u8; ROWS] = [0; ROWS];

        for i in 0..ROWS {
            arr[i][4] = i as u8;
        }

        for i in 0..ROWS {
            arr2[i] = i as u8;
        }

        assert_eq!(true, check_arr_c(arr, &arr2, 0, 0));
    }

    #[test]
    fn test_check_arr_r() {
        const ROWS: usize = 5;
        const COLUMNS: usize = 10;
        let mut arr: [[u8; COLUMNS]; ROWS] = [[0; COLUMNS]; ROWS];
        let mut arr2: [u8; COLUMNS] = [0; COLUMNS];
        for i in 0..COLUMNS {
            arr[4][i] = i as u8;
        }
        for i in 0..COLUMNS {
            arr2[i] = i as u8;
        }
        assert_eq!(true, check_arr_r(arr, &arr2, 0, 0));
    }

    #[test]
    fn test_check_arr() {
        const ROWS: usize = 5;
        const COLUMNS: usize = 10;

        let mut arr: [[u8; COLUMNS]; ROWS] = [[0; COLUMNS]; ROWS];
        arr[4][4] = 42;
        assert_eq!(true, check_arr(arr, 42, 0, 0));
    }

    #[test]
    fn test_rev_arr() {
        const ROWS: usize = 10;
        const COLUMNS: usize = 10;
        let mut arr_act: [[u8; COLUMNS]; ROWS] = [[0; COLUMNS]; ROWS];
        let mut arr_rev: [[u8; COLUMNS]; ROWS] = [[0; COLUMNS]; ROWS];
        for i in 0..ROWS {
            for j in 0..COLUMNS {
                arr_rev[i][j] = (i + j) as u8;
            }
        }
        for i in 0..ROWS {
            for j in 0..COLUMNS {
                arr_act[j][i] = (i + j) as u8;
            }
        }
        assert_eq!(arr_act, rev_arr(arr_rev, 0, 0));
    }

    #[test]
    fn test_rev_str() {
        let s = String::from("hello");
        let l = s.len();
        assert_eq!("olleh", &rev_str(s, 0, l - 1));
    }
}
