# Architecture

## Flux architecture

### User action

### Dispatcher

### Store

### View

## Bird's eye view

```
+----------------------------------+
|         File Organization        |
|           Software               |
+----------------------------------+
|                                  |
| - Responsible for organizing     |
|   files based on user-defined    |
|   filters and actions.           |
|                                  |
| - Utilizes the FileFilter and    |
|   ActionExecutor modules         |
|   for filtering and executing    |
|   actions on files.              |
|                                  |
| - Interacts with the file system |
|   using the FileSystem module.   |
|                                  |
+----------------------------------+
                 |
                 |
   +-----------------------------+
   |        FileFilter           |
   +-----------------------------+
   | - Manages filter criteria   |
   |   and checks file matches.  |
   |                             |
   | - Uses FilterCriteria       |
   |   and Operator modules.     |
   |                             |
   +-----------------------------+
                 |
                 |
   +-----------------------------+
   |      ActionExecutor         |
   +-----------------------------+
   | - Manages file actions      |
   |   and executes them.        |
   |                             |
   | - Uses Action module.       |
   |                             |
   +-----------------------------+
                 |
                 |
   +-----------------------------+
   |        FileSystem           |
   +-----------------------------+
   | - Handles file system       |
   |   interactions (scan,       |
   |   access, etc.).            |
   |                             |
   | - Interfaces with the OS    |
   |   file system functions.    |
   |                             |
   +-----------------------------+
```

In this bird's eye view, the primary components of the File Organization
Software are depicted. The File Organization Software is the core module
responsible for organizing files. It uses two main modules, the FileFilter
module, and the ActionExecutor module, to apply filters and execute actions on
the files, respectively. The software also interacts with the file system using
the FileSystem module to access and manipulate files in the user's storage
system.

Each component's responsibilities are briefly summarized to provide an overview
of how the different parts of the architecture work together to achieve the file
organization functionality.
