//~ ERROR cycle detected when computing layout of
//~| NOTE ...which requires computing layout of
//~| NOTE ...which again requires computing layout of
//~| NOTE cycle used when computing layout of

// build-fail

trait Mirror { type It: ?Sized; }
impl<T: ?Sized> Mirror for T { type It = Self; }
struct S(Option<<S as Mirror>::It>);

fn main() {
    let _s = S(None);
}
