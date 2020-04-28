# rtd

Manage your todo in command line with `rtd` (Rust To Do)

## Features
### Add a todo

```
// Add a new to-do to inbox as low priority, which dues at 24th Apr 2020
rtd add "This is a todo" :inbox +low  @2020-04-24
```

**Supported Priority**
- high / h
- low / l
- medium / m

**Supported due date format**
- 2020-04-01
- 20200401
- today / tomorrow
- Monday / Tuesday / Wed...
- Mon / Tues / Thur


### List todos
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

### Today's priority
```
// Mark a todo as your today's priority
rtd today <todo-id>

// Unmark a todo from your today's priority
rtd today --unset <todo-id>
rtd today -u <todo-id>
```

### Edit todo

```
rtd edit <todo-id> "new-title" :<new-list> +<priority> @<due-date>
```

### Delete todo

```
rtd delete <todo-id> <todo-id> <todo-id> 
```

### Mark todo as completed

```
// Mark todo as done
rtd done <todo-id>

// Mark todo as not done
rtd done --unset <todo-id>
rtd done -u <todo-id>
```

## Licence
Copyright 2020 

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License. You may obtain a copy of the License at http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.