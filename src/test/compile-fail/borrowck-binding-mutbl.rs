fn impure(_v: ~[int]) {
}

fn main() {
    let x = {mut f: ~[3]};

    alt x {
      {f: v} => {
        impure(v); //~ ERROR illegal borrow unless pure: unique value in aliasable, mutable location
        //~^ NOTE impure due to access to impure function
      }
    }
}