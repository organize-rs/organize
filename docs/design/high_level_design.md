# System Design

1. **File Scanning and Filtering Module**: This module is responsible for
   traversing the file system and identifying files that match the
   user-specified filters. It can be designed to use efficient data structures
   and algorithms to reduce resource consumption and increase speed.

2. **Rule Engine**: This module interprets the rules from the config file. It
   manages rule creation, modification, deletion, and validation. It also maps
   the matched files from the scanning module to the corresponding actions.

3. **Action Handler**: This module is responsible for executing actions on
   matched files. It interfaces with the rule engine to get the list of files
   and corresponding actions. This module must handle potential errors and
   exceptions gracefully.

4. **Config File Manager**: This component handles reading from and writing to
   the config file. It should provide an interface for other modules (like the
   Rule Engine) to interact with the config file data.

5. **Version Control System**: This module is responsible for tracking changes
   to the file system. It should provide the ability to revert to a previous
   state. Depending on the level of complexity required, it can be a simple
   logging system or a complex version control system like Git.

6. **User Interface (UI)**: The UI interfaces with all the other modules. It can
   be a command-line interface, a graphical interface, or both. It must be
   user-friendly and intuitive.

7. **Logging and Error Handling System**: This system-wide component handles all
   errors and exceptions. It provides logs of actions performed and errors
   encountered.

Here's a high-level diagram that illustrates how these components interact:

```
User Interface
    |
    | - Interacts with all modules to get user input and provide feedback.
    |
    V
Config File Manager <--> Rule Engine <--> Action Handler
    |                   |      |       ^
    |                   |      |       |
    |                   V      V       |
Version Control <---- File Scanning and Filtering
    |
    | - Keeps track of changes and provides the ability to revert actions.
    V
Logging and Error Handling
```

### UML Diagram

A UML Class Diagram can represent the main structures and relationships in the
design:

```
                   +-----------------------+
                   |       main.rs         |
                   +-----------------------+
                   |       main()          |
                   +-----------------------+
                             |
                             | creates
                             |
                   +-----------------------+
                   |    FileOrganizer      |
                   +-----------------------+
                   | - filter_sets: Vec<FilterSet>
                   | - action_executor: ActionExecutor
                   +-----------------------+
                   | + organize_files()    |
                   +-----------------------+
                             |
                             | uses
                             |
          +------------------v------------------+
          |             FileFilter             |
          +------------------------------------+
          | - filter_criteria: Vec<FilterCriteria> |
          +------------------------------------+
          | + add_criteria()                  |
          | + remove_criteria()               |
          | + match_file(file_info) : bool    |
          +------------------------------------+
                     |         |
                     |         |
      +--------------+         +---------------+
      |                                      |
      |                                      |
+------------------------+          +------------------------+
|   FilterCriteria       |          |      FilterSet         |
+------------------------+          +------------------------+
| - attribute: String    |          | - name: String         |
| - operator: Operator   |          | - filters: Vec<FileFilter> |
| - value: String        |          +------------------------+
+------------------------+
| + new(attribute, operator, value)  |
+------------------------------------+
             |
             |
   +------------------------+
   |      Operator          |
   +------------------------+
   | EQ                     |
   | NEQ                    |
   | CONTAINS               |
   | NOT_CONTAINS           |
   | GT                     |
   | LT                     |
   +------------------------+
             |
             |
    +------------------------+
    |      ActionExecutor    |
    +------------------------+
    | - actions: Vec<Action> |
    +------------------------+
    | + add_action()         |
    | + remove_action()      |
    | + execute_action(file_info) |
    +------------------------+
             |
             |
     +--------------------+
     |       Action       |
     +--------------------+
     | - action_type: ActionType |
     | - destination: String     |
     +--------------------+
     | + new(action_type, destination) |
     +---------------------------------+
                         |
                         |
              +----------------------+
              |     ActionType       |
              +----------------------+
              | MOVE                 |
              | COPY                 |
              | DELETE               |
              | RENAME               |
              +----------------------+
```
