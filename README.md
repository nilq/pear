## pear
a strong and very confused programming language

### a syntax

---

assignments
```
foo: number = 100 + 100

foo *=
```

---

fib
```
fib: match n -> number {
  0 => 0
  1 => 1
  n => (fib n - 1) + (fib n - 2)
}
```

---

functions
```
puts: fun (a: number) -> number {
  print a
  0
}

fizz: fun (a: number) -> number {
  match a % 3 {
    0 => puts "fizzz"
    _ => a
  }
}

buzz: fun (a: number) -> number {
  match a % 5 {
    0 => puts "buzz"
    _ => a
  }
}

fizzbuzz := print >> fizz >> buzz
```
