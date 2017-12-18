const steps = 349
let buffer = [0];
let current = 0;

for (let i = 1; i < 2018; i++) {
    for (let j = 0; j < steps; j++) {
        current = (current + 1) % buffer.length;
    }
    buffer.splice(current + 1, 0, i);
    current += 1;
}
console.log(buffer[current + 1])