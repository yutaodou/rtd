# rtd

A command line task management tool to practice rust

## Features
**Add task**

```
rtd add "This is a todo" ~inbox #tag1 !low @20200202
```

**List tasks**
```
# list all tasks not done yet
rtd list 

# list completed tasks
rtd list --done

# list all

rtd list --all
```

**Complete task**

```
rtd done <task-id>
```