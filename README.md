# rtd

Manage your todo in command line with `rtd` (Rust To Do)

## Features
**Add a todo**

```
// Add a new to-do to inbox as low priority
rtd add "This is a todo" :inbox +low 
```

**List todos**
```
# List all todo not done yet
rtd list 

# List completed todo
rtd list --done
rtd list -d

# List all
rtd list --all
rtd list -a

# List all todo from a specific list
rtd list inbox
```

**Today's priority**
```
// Mark a todo as your today's priority
rtd today <todo-id>

// Unmark a todo from your today's priority
rtd today --unset <todo-id>
rtd today -u <todo-id>
```

**Edit todo**

```
rtd <todo-id> "new-title" :<new-list> +<priority>
```

**Mark todo as completed**

```
// Mark todo as done
rtd done <todo-id>

// Mark todo as not done
rtd done --unset <todo-id>
rtd done -u <todo-id>
```