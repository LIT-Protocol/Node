#!/usr/bin/env python3

import getpass
import codecs
import ecdsa

print('Enter your private key below')
private_key = getpass.getpass()

private_key_bytes = codecs.decode(private_key, 'hex')
# Get ECDSA public key
key = ecdsa.SigningKey.from_string(private_key_bytes, curve=ecdsa.SECP256k1).verifying_key
key_bytes = key.to_string()
key_hex = codecs.encode(key_bytes, 'hex')
key_str = key_hex.decode('utf-8')

print(f'Public key: 0x04{key_str}')