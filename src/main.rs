use bracket_lib::terminal::{main_loop, BTermBuilder};
mod state;


fn main() {
    let result = BTermBullder::simple80x50()
        .with_title("Hello Bracket World")
        .build();
}
