fn main() {
    let labels = [("foo", "bar".to_owned())];
    metrics::gauge!("some", 5.0, &labels);
}
