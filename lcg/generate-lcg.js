function generate(modulus, multiplier, increment, initialValue = 0) {
  const sequence = [];
  let value = initialValue;
  for (let i = 0; i < modulus; i++) {
    sequence.push(value);
    value = (multiplier * value + increment) % modulus;
  }
  return sequence;
}

console.log(JSON.stringify(generate(999, 445, 713)));
