use cxx_qt_build::CxxQtBuilder;

fn main() {
    CxxQtBuilder::new()
        .file("src/app_controller.rs")
        .file("src/connection_model.rs")
        .file("src/query_controller.rs")
        .file("src/redis_model.rs")
        .file("src/result_model.rs")
        .file("src/schema_model.rs")
        .build();
}
