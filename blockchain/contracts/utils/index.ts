export function int2ip(ipInt: number | bigint) {
  let ipIntBigInt = BigInt(ipInt);
  return (
    (ipIntBigInt >> 24n) +
    '.' +
    ((ipIntBigInt >> 16n) & 255n) +
    '.' +
    ((ipIntBigInt >> 8n) & 255n) +
    '.' +
    (ipIntBigInt & 255n)
  );
}

export function ip2int(ip: string) {
  return (
    ip.split('.').reduce(function (ipInt, octet) {
      return (ipInt << 8) + parseInt(octet, 10);
    }, 0) >>> 0
  );
}
