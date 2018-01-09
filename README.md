# pear
a strong and very confused programming language

## the pitch

### transpilation

the pear compiler is like a very hungry dog with diarrea, that furiously eats up and
consumes pear source code, after which it plops out messy/optimized lua source code(for later use in love2d).

### beautiful syntax

```
fib: fun (a: number) -> number {
  match a {
    | 0 -> 0
    | 1 -> 1
    | n -> (fib n - 1) + (fib n - 2)
  } 
}
```

equivalent to:

```
fib: match n -> number {
  | 0 -> 0
  | 1 -> 1
  | n -> (fib n - 1) + (fib n - 2)
}
```

```
mut human := [
  x: love.graphics.getWidth! / 2
  y: 100
]

move: fun a: [x, y] with: (x: number, y: number) {
  a.x += x
  a.y += y
}

love.update: fun (dt: number) {
  unless love.keyboard.isDown "space" {
    move human (0, 10 * dt)
  }
}
```
