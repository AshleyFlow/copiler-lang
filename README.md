# Copiler

Copiler is a very dumb programming language that gets compiled into Luau, and it mostly let's you slide with doing crazy things, like:

```js
let testFunction = (100: "cool!", "a": 500.2) {

}
```

or this, which is just... wrong

```js
let my_car = class {}
```

## Example

Compile the example script

```shell
cargo run ./main.cop
```

Check out ./dist/out.luau to see the compiled result

## Features

### Classes

```js
class MyClass {
    let speed = 10
    let model = "xeltda ford"
}

let my_class = MyClass.new()
```

### Functions

```js
let my_function = (name: string) {
    print("Hello", name)
}

my_function("Cool!")
```

### Variables

```js
let x = 10
let also_x = x
```

### If Statements

```js
if true {
    print("Hello, World!")
}
```
