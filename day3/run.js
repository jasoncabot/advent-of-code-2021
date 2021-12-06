fs = require('fs');
const data = fs.readFileSync('input1.txt', 'utf8');

const bitWidth = 12;
const counts = new Array(bitWidth).fill(0);
data.split('\n').forEach(value => {
    for (let idx = 0; idx < bitWidth; idx++) {
        counts[idx] += (value[idx] == '0' ? 0 : 1);
    }
});

const gamma = parseInt(counts.map(num => num > 500 ? '1' : '0').join(''), 2);
const epsilon = parseInt(counts.map(num => num < 500 ? '1' : '0').join(''), 2);

console.log(gamma);
console.log(epsilon);
console.log(gamma * epsilon);
