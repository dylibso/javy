function fibonacci(num) {
  var a = 1, b = 0, temp;

  while (num >= 0) {
    temp = a;
    a = a + b;
    b = temp;
    num--;
  }

  return b;
}

const buffer = new Uint8Array(1);
Javy.IO.readSync(0, buffer);
const input = (new TextDecoder()).decode(buffer);
const result = fibonacci(parseInt(input, 10));
Javy.IO.writeSync(1, (new TextEncoder()).encode(result.toString() + "\n"));

