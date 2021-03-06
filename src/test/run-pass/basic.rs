// -*- rust -*-

use std;
import comm;
import comm::send;
import comm::chan;
import comm::recv;
import task;

fn a(c: chan<int>) {
    if true {
        debug!{"task a"};
        debug!{"task a"};
        debug!{"task a"};
        debug!{"task a"};
        debug!{"task a"};
    }
    send(c, 10);
}

fn k(x: int) -> int { return 15; }

fn g(x: int, y: ~str) -> int {
    log(debug, x);
    log(debug, y);
    let z: int = k(1);
    return z;
}

fn main() {
    let mut n: int = 2 + 3 * 7;
    let s: ~str = ~"hello there";
    let p = comm::port();
    let ch = comm::chan(p);
    task::spawn(|| a(ch) );
    task::spawn(|| b(ch) );
    let mut x: int = 10;
    x = g(n, s);
    log(debug, x);
    n = recv(p);
    n = recv(p);
    debug!{"children finished, root finishing"};
}

fn b(c: chan<int>) {
    if true {
        debug!{"task b"};
        debug!{"task b"};
        debug!{"task b"};
        debug!{"task b"};
        debug!{"task b"};
        debug!{"task b"};
    }
    send(c, 10);
}
