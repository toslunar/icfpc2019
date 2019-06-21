type APos = (usize, usize);
type RPos = (isize, isize);


fn deps(v: RPos) -> Vec<RPos> {
    let (x, y) = v;
    let sx = x.signum();
    let sy = y.signum();
    let mut txs = vec![];
    for i in 0..x.abs() {
        txs.push((2 * i + 1) * y.abs());
    }
    let mut tys = vec![];
    for i in 0..y.abs() {
        tys.push((2 * i + 1) * x.abs());
    }
    txs.reverse();
    tys.reverse();

    let mut cx = 0;
    let mut cy = 0;
    let mut tx = txs.pop();
    let mut ty = tys.pop();
    let mut rets = vec![];
    while !(tx.is_none() && ty.is_none()) {
        if tx == ty {
            cx += sx;
            cy += sy;
            tx = txs.pop();
            ty = tys.pop();
        } else if ty.is_none() {
            cx += sx;
            tx = txs.pop();
        } else if tx.is_none() {
            cy += sy;
            ty = tys.pop();
        } else if tx < ty {
            cx += sx;
            tx = txs.pop();
        } else {
            cy += sy;
            ty = tys.pop();
        }
        rets.push((cx, cy));
    }
    return rets;
}


fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a
    } else {
        gcd(b, a%b)
    }
}



fn main() {
    println!("{:?}", deps((3, -5)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(deps((-2, 0)), [(-1, 0), (-2, 0)]);
        assert_eq!(deps((2, 4)), [(0, 1), (1, 1), (1, 2), (1, 3), (2, 3), (2, 4)]);
        // assert_eq!(2 + 2, 4);
        //read_map()
    }
}
