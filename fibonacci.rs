fn main() {
let n = 10;
let mut fibonacci = vec![0, 1];

```
for i in 2..n {
    let next = fibonacci[i - 1] + fibonacci[i - 2];
    fibonacci.push(next);
}

println!("Fibonacci Sequence: {:?}", fibonacci);

```

}
