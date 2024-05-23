# Copiler

Copiler is a very dumb programming language that gets compiled into Luau

## Example

Compile the example script

```shell
cargo run .\examples\hello_world.cop
```

Check out ./dist/out.luau to see the compiled result

## Features

### Classes

```js
class MyClass {
    let speed = 10
    let model = "xeltda ford"

    let print_info = () {
        print(self.speed, self.model)
    }
}

let my_class = MyClass.new()
my_class.print_info(my_class)

```

### Functions

```js
let my_function = (name: string) {
    print("Hello", name)
}

my_function("Cool!")
```

### Anonymous Functions

```js
let my_function = (callback) {
    callback()
}

my_function(() {
    print("Hello!")
})
```

### Variables

```js
let x = 10
let also_x = x

x = 50
```

### If Statements

```js
if true {
    print("Hello, World!")
}
```

### 'and' and 'or'

```js
let part = get_part() && true || false
```

which turns into:

```lua
local part: boolean = get_part() and true or false
```

### Returning

```js
let my_function = () {
    return 69.9
}

let x = my_function()
print(x)
```

### Luau

You can directly write luau

```js
class MyClass {
    let speed = 10
    let model = "xeltda ford"

    let print_info = () {
        \\
        print(self.speed, self.model)
        \\
    }
}

\\
-- luau code here
let my_class = MyClass.new()
my_class:print_info()
\\

```
