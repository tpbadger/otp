# OTP

This is a rudimentary implementation of a [One-time pad](https://en.wikipedia.org/wiki/One-time_pad) CLI tool built in rust. 

The tool has 3 operations. 
1. GenerateKey which is used to generate a key of a specified length. It will save the key as key.txt in the location where the tool is run.
2. Encode which is used to encode a message with an existing key. It will save the encoded message as encoded.txt in the location where the tool is run.
3. Decode which is used to decode a message with an existing key. It will save the decoded message as decoded.txt in the location where the tool is run.

## Instructions for use
1. Download the cli tool for your OS
2. Invoke the CLI using `otp <command> <option>`
    1. For example if I want to send messages that will be 11 characters long e.g `hello world` I would generate a 11 character key using `otp GenerateKey -l 11`
    2. If I then wanted to encode my message saved as message.txt with my key I just generated saved as key.txt I would use `otp Encode -k key.txt -m message.txt`
    3. If I then wanted to decode a message I received saved as encoded.txt using the key saved as key.txt I would use `otp Decode -k key.txt -m encoded.txt`
