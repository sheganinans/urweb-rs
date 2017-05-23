bindgen --no-unstable-rust --conservative-inline-namespaces --ignore-functions --ignore-methods --no-layout-tests --generate types,vars /usr/local/include/urweb/urweb.h > src/urweb.rs
