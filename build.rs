const COMMANDS: &[&str] = &[
    "load",
    "execute",
    "select",
    "close",
    "begin_transaction",
    "commit_transaction",
    "execute_transaction",
    "rollback_transaction",
    "select_transaction",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();
}
