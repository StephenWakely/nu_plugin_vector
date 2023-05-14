use nu_plugin::{serve_plugin, MsgPackSerializer};
use nu_plugin_vector::Vector;

fn main() {
    serve_plugin(&mut Vector, MsgPackSerializer {})
}
