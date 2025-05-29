function checkLcg(modulus, multiplier, increment) {
  const visited = new Set();
  let value = 0;
  for (let i = 0; i < modulus; i++) {
    value = (multiplier*value + increment) % modulus;
    if (visited.has(value)) return false;
    visited.add(value);
  }
  return true;
}

const MODULUS = 999;
for (let a = 0; a < MODULUS; a++) {
  for (let c = 0; c < MODULUS; c++) {
    if (checkLcg(MODULUS, a, c)) {
      console.log(`[${MODULUS}, ${a}, ${c}]`);
    }
  }
}
