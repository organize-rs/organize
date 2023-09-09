# Domain specific language

## Rhai Documentation

- <https://rhai.rs/book/engine/dsl.html>

## Goals

- make it possible to create a general model, as in I have a directory where all
  of the files/folders go, that match these filters
  - so basically the other way around, not `filter->action` but

    `destination <- action(action_command) <- filter(filter_cmd) <- location`

    as in:

    `app_directory <- action[move] <- filter[(ext(apps) | !name(keep_)) & size(20mb..)] <- ~/Downloads`

- make it simple and straight forward to invoke filters and defining the actions
  that are applied to them

- make it possible to chain filters, so you can run multiple filters on the same
  defined location twice or more
  - e.g. when you want to filter for `zip` files that have been created in the
    last seven days
  - negate filters, so you can easily filter for `zip` files that have **not**
    been created in the last seven days

- make it possible to pass optional arguments easily, especially the often used
  ones like `recursive` (bool) and `max-depth` (u64)

- make it possible to create aliases for a group of certain directories
  - so you can keep it DRY and access them whereever you need
  - error handling can be tricky here:
    - e.g. filtering for files and moving them to an alias that contains more
      than one location
      - [RESEARCH] what should happen in that case?
      - we shouldn't just move them to the first given location
      - we shouldn't just copy them to each location and then remove the
        original
