class Span {
  spanContext() {
    return { traceId: "mytraceid", spanId: "myspanid" }
  }
}


function fibonacci(num) {
  var a = 1, b = 0, temp;

  while (num >= 0) {
    temp = a;
    a = a + b;
    b = temp;
    num--;
  }

  __dylibso_observe_instrument_span_record(new Span())
  return b;
}

const buffer = new Uint8Array(1);
Javy.IO.readSync(0, buffer);
const result = fibonacci(buffer[0]);
buffer[0] = result;
Javy.IO.writeSync(1, buffer);

