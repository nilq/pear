## pear
a strong and very confused programming language

### beautiful syntax

```
foo: number = 100 + 100
```

---

fib

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
