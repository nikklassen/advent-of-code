const steps = 349
let current = 0;
let afterZero = null;

for (let i = 1; i < 50000000; i++) {
    current = (current + (steps % i)) % i;
    if (current === 0) {
        afterZero = i;
    }
    current += 1;
}
console.log(afterZero)