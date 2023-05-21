print("Hello from rhai script!");

let folder = "C:\\Users\\dailyuse\\dev-src\\organize\\tests";

let apps_ext = ["exe", "msi", "apk"];

let entries = init_organize(folder);
let ext_entries = filter::by_extension(entries, apps_ext);
let last_modified = filter::by_last_modified(ext_entries, "..7d")

action::move(last_modified, "~/backup/INBOX/@Apps/")

let app_inbox_folder = "~/backup/INBOX/@Apps/";
filter::by_extension(entries, apps_ext) move_to app_inbox_folder;

filter(folder->by_extension(apps_ext)) => action(move->app_inbox_folder);