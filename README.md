# Mimble
Mimble is a language with Python-like syntax that does not uses classes.

## Usage
Compiler either `MimbleLibrary` or `MimbleApplication`.

### MimbleLibrary
`MimbleLibrary` compiles to a  library file for use in other projects.

### MimbleApplication
`MimbleApplication` compiles to a console application.

The executable can either execute any text file that contains Mimble code.
```bash
mimble /path/to/file
```

Alternatively, executing the application with no parameters will open a REPL to execute single lines of Mimble.


## Features
Mimble has most of the features you would expect from other languages with a little extra.

### Values
Mimble has 6 main value types it can handle, these are:
- Strings
    - `"this is a string"`
- Numbers
    - `1234`
    - `12.34`
- Booleans
    - `true`
    - `false`
- Null
    - `null`
- Lists
- Functions

Lists and functions are further on.

### Comments
Comments are written using a single `#`.

```mimble
# this is a comment
```

### Blocks
A block is a chunk of code. Inside Mimble, blocks are written using `do` and `end` (this changes when it comes to functions for ease of use).
```mimble
do
    # this is a block
end
``

### Variables
Varible declaration and initialization is preformed in the same assignmnet expression:
```mimble
a = 123
```

Variables can store any of the values referenced above. The contents of the variable can also change and does not have to be of the same type.

### Lists
List values are defined by `[` and `]`.

```mimble
list = []
```

A list can also be defined with values. 

Lists do not have to contain only one type of value like in other languages.
```mimble
list = ["hello", "world", 123]
```

### Control Flow

#### If Statements
Mimble can handle `if`, `elif` and `else` blocks as follows:

```mimble
if expression do
    print("expression is true")
end
elif other do
    print("other is true")
end
else
    print("expression and other are false")
end
```

#### Loops
While loops:
```mimble
while expression do
    print("expression is true")
end
```

For loops:
```mimble
for var in list do
    print(var)
end
```
> Note that in the `for` loop: `list` can also just be the declaration of a list (`[1, 2, 3]` for example).

### Functions
Defining a function is done with the `function` keyword.

```mimble
function fun() does
    return "this is a function"
end
```

Mimble can also handle function definitions inside other functions.
```mimble
function outer() does
    function inner() does
        print("inner")
    end
    print("outer")
    inner()
end
```

Running the above code:
```bash
outer
inner
```

Mimble also handles all functions as first-class functions.
```mimble
function fun() does
    return "function"
end

a = fun
```

The variable `a` does not contain `"function"` but actually the function `fun()` itself, meaning you can execute it like:
```mimble
a() # prints "function"
```

### System Functions
A list of all functions avaible to the user:
- `append(list, toAppend)`
- `length(list)`
- `pop(list, index)`
- `print(value)`

**More functions will be added later**
