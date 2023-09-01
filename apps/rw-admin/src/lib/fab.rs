

pub fn fab(n: i32) -> i32 {
    if n < 0 {
        return 0;
    }
    if n == 1 {
        return 1;
    }
    fab( n -2) + fab(n -1)
}