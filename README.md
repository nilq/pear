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
fizz: fun (a: number) -> number {
  match a % 3 {
    0 => print "fizzz"
    _ => a
  }
}

buzz: fun (a: number) -> number {
  match a % 5 {
    0 => print "buzz"
    _ => a
  }
}

fizzbuzz := print >> fizz >> buzz
```
