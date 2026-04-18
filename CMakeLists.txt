cmake_minimum_required(VERSION 3.16)

project(everydb LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

find_package(Qt6 REQUIRED COMPONENTS Core Gui Qml Quick QuickControls2)

# Find Corrosion
find_package(Corrosion REQUIRED)

corrosion_import_crate(MANIFEST_PATH Cargo.toml)

add_executable(everydb_gui
    crates/app/qml/main.qml
)

target_link_libraries(everydb_gui
    PRIVATE
    Qt6::Core
    Qt6::Gui
    Qt6::Qml
    Qt6::Quick
    Qt6::QuickControls2
    everydb
)

# Qt QML module
qt_add_qml_module(everydb_gui
    URI EveryDB
    VERSION 1.0
    QML_FILES
        crates/app/qml/main.qml
)
