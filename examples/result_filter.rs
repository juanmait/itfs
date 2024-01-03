use itfs::ResultFilter;

fn main() {
    let items = [
        Result::Ok(0),
        Result::Err(1),
        Result::Ok(2),
        Result::Ok(3),
        Result::Err(4),
        Result::Ok(5),
    ];

    for item in ResultFilter(items.into_iter()) {
        println!("{:?}", item);
    }
}
