# WIP
version = "1.0.0",
aliases = [{}],
rules = [{
    name = "Empty file",
    enabled = true,
    locations = [{
        recursive = {
            path = "crates\organize-rs_core\tests\fixtures\filters\empty_file",
            max_depth = 1,
            target = "Files"
        }
    }],
    filter_groups = [
        {
            filters = [
                "empty" = {}
            ],
            results = "include",
            match = "all",
        }
    ],
    actions = [
        {
            mode = "preview",
            action = {
                "trash"
            }
        },
        {
            mode = "preview",
            action = {
                "echo"
                msg = "test"
            }
        }
    ],
    tags: [
        {

        }
    ]

}],