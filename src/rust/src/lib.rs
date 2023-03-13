use extendr_api::prelude::*;

/// Trigger Stack Overflow
/// @export
#[extendr]
fn stack_overflow_list(x: List) -> List {
    x
        .into_iter()
        .map(|(_, x)| {
            let dbs = Doubles::from_robj(&x).unwrap();
            [dbs[1].0, dbs[0].0]
        })
        .collect::<List>()
}


#[extendr]
/// @export
fn stack_overflow_list_dbls(x: List) -> Doubles {
    x
        .into_iter()
        .map(|(_, x)| {
            let dbs = Doubles::from_robj(&x).unwrap();
            let sum = dbs[1].0  + dbs[0].0;
            Rfloat::from(sum)
        })
        .collect::<Doubles>()
}

extendr_module! {
    mod stackoverflow;
    fn stack_overflow_list;
    fn stack_overflow_list_dbls;
}
