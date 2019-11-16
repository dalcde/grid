fn main() {
    let file = std::fs::File::create("test.pdf").expect("Cannot create file");
    grid::grid(
        file,
        grid::Config {
            width: 11.7,
            height: 8.27,
            margin: 0.2,
            color: (100, 100, 100),
            num_x: 56,
            num_y: 39,
            d: 0.2,
            num_pages: 2,
        },
    )
    .expect("Cannot write to file");
}
