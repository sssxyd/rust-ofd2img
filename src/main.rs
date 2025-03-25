mod ofd;
mod document;
mod st_types;

fn main() {
    let start_time = std::time::Instant::now();
    let ret = ofd::OfdDoc::open("data/fapiao.ofd");
    if let Err(e) = ret {
        println!("{:?}", e);
        return;
    }
    let doc = ret.unwrap();
    let attributes = doc.info();
    println!("{:?}", attributes);
    println!("Elapsed time: {:?}", start_time.elapsed());
}
