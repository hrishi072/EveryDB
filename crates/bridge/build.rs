use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        .file("src/lib.rs")
        .setup_linker()
        .build();
}
