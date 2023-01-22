# sendit

## What is it?

Introducing Sendit, an end-to-end encrypted, fault-tolerant, and blockchain-based file transfer system. It is designed to be a decentralized alternative to centralized file transfer services such as WeTransfer.

Easily send files to anyone, anywhere, without the need for a centralized server.

## How does it work?

Someone wants a file you, the sender, have. They send you a key (an address), and you upload the file using the [bendit app](https://github.com/hainsdominic/bendit) by specifying the key to the blockchain. The receiver then receives file on their machine.

## Technical deep dive

There are two main components to the system: the bendit app and the sendit app.

### sendit - the blockchain node

The sendit app is a blockchain node that serves 2 purposes:

- It has a peer IP resolver, meaning it can resolve the IP address of a peer given their public key.

- It has the blockchain, which essentially stores the file hashes and the public keys of the sender and receiver.

The blockchain block structure is as follows:

```json
{
  "timestamp": 1674371129,
  "sender_pub_key": "b'-----BEGIN PUBLIC KEY-----\\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAxpj1jAU/RoTfNoOnfaTD\\nmcWqjbnxY+7NmTQJJVIPrdQ29mFb9WJOW8YdtCRLoxF70ABdz1davXew+x2JkShH\\nEmN5XY/nRCFRolQpgwiftpbe5R0vJ9xJEXeip0DZmX+HPlWAkmm6VD3euc8GV9TH\\nEk0noZ3g6t5Bq+yWo9WDfjAVINUNv8nYx61k/U0pLhyzJ7AY0FywpvWS8X6x44OH\\njEWlQe8YRNi5fgY8dKyW7aGSi8mOowsG/UWshJTmq5BpvTY5c1SZCIZwIC1n6T6r\\nHQ5L9Ad3BZjlq3dApQwpzZGfhMMIs/+noIhDDJGK6FvKQIbtf2bd2POg72V7yALP\\ncQIDAQAB\\n-----END PUBLIC KEY-----\\n'",
  "receiver_pub_key": "b'-----BEGIN PUBLIC KEY-----\\nMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKCAQEAotFJw10yfZxiDZ7UXieE\\nGndSOjqde5NB7EXkV2BbEexCf/nvW88UXUXx15r3hwweZs9fH00TUA9RkiTMm5TC\\nM1srW7RvN1qlBokdOgSLN3R/LTBloIO6STwxmuuLqF5bkMy9GuAWKq7q1wYkNPat\\nT6N3DyBcSHam+XfWJh+mBNWzlj4oqevoD3mjbg8jefRl4l+p6FDeiezrLYdgBJUy\\nH7SkETe7QOlFnfIel2AsMhRGo5Pf9mQ7Ca87mEmIOxYbEMDvyTZLLeEuubggTLiy\\nq5ww7m/9Cn/GlnNHAsCu8oP+vI5Jv8hNZpp8N9ChRHbo57JS4R77aOdEd2vx/Nw/\\nNQIDAQAB\\n-----END PUBLIC KEY-----\\n'",
  "file_hash": "fa5944ed2b80732bf6866497e0b60e988bb4a0531129ac764636435c0efe3da8",
  "sender_signature": "b'\\x86\\xeavp7<AjMQv\\xe8\\xb81nyu\\xe9\\xb4o{\\xeb\\x1a\\xa9\\xb6\\xd7\\xe5\\x07\\xfcc\\x1e\\xc8.>\\xe6&\\xbe\\xa1\\x12\\xffe]\\xd9Y+b\\x10\\xe2\\xe2J\\x8c\\x16\\xdc\\xa2]\\x99]$\\xe4P\"\\x869\\xbb\\xd7o\\x04\\xbf\\x8f\\xbcX\\xa65\\x9d\\xe9\\x1b\\x1d\\xf8u\\xd3\\xbe\\x9c\\xbd\\xaa{\\x12\\xc5Bt\\xd1\\x9d6\\xfae\\x01N\\x1d\\x9a(bU\\x86\\\\`^X\\x88\\xc0L\\xf4\\xc3\\x8f\\xe4\\xaf\\xed\\x00\\xa9\\r\\x1b\\xbd\\x04\\x80T,\\tE\\x94 i\\x90Z\\xed\\x9a\\x83#y\\x9c\\xa6 24dj\\xd2\\xda\\xc6\\xc3\\x13D;\\xbe\\x05M\\xe9\\x86K\\xa2\\tU\\xff\\xc0,x\\x15\\xc6\\xd4)\\x93|\\x1aw\\x93Ho\\x03]5M\\x06\\x91\\x82\\xfb\\x88\\xb3\\xf5x\\xa1{#)o\\xed\\x16j\\xb4D\\xecY\\x88]p\\xab\\xf7\\x8fWh\\xa4xk:\\xd0 \\xb1Z\\xdbb\\xb2G\\x99\\xcc\\x16\\r\\xa4\\x144 \\x7f\\x98\\x8a\\xae\\xdb\\xb3\\x8d\\xe3\\x8c\\xbb\\r\\xbf\\xbe\\xad\\xf2\\xffV1\\xcc\\x11\\xf7\\x95)h\\xf3\\xe2\\x91\\xf4\\xa3N'",
  "index": 1,
  "prev_block_hash": "12a2f3b25d8f84df5243f2501f61e13d366e7c16c1c0861b2991c81819586825",
  "block_hash": "c5cdb868801b2a6a122fb2099a3e39adaffddd67b39fc3b58460391d4d2c5fd6"
}
```

It is also responsible for managing the state of the blockchain, it adds new blocks and queues pending file transfer confirmations to the blockchain and it also resolves conflicts between the blockchain of different nodes.

### bendit - the client

The client is responsible for the user interface and the user interaction with the blockchain. It is also responsible for the file transfer and the file encryption.

Here, Alice wants to send a file to Bob. The app already generated their RSA private and public keys.

![schema](https://github.com/hainsdominic/bendit/blob/main/resources/schema.jpg?raw=true)

The first thing the peers do is make sure they're IP is known by the node and that it can be resolved by another peer later. This is done by the `add_peer` RPC.

Using the public key of Bob, Alice resolves Bob's IP address using the `get_peer` RPC.

Alice then encrypts the file using Bob's public key.

Using Bob's IP address, Alice sends the encrypted file straight to Bob using the `send_file` RPC.

Alice concurrently signs the encrypted file using her private key and sends the signature to the node by creating a new block, sending the file hash, the receiver public key, the sender public key and the signature to the node using the `add_block` RPC.

Bob thens mine the block and adds it to the blockchain.

The mining operation verify that the signature is valid and that the file hash is correct. If it is, the block is added to the blockchain.

### technical limits of this project

This project is a proof of concept implemented for a hackathon and is not meant to be used in production. It has a lot of limitations:

- The nodes do not sync with each other yet. This means that if a node goes offline, the blockchain will be offline.

- The file size is capped because of the buffer size of the RPC. This means that the file size is limited. Experimentally, the file size is limited to 1MB.
