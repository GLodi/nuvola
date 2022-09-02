# Nuvola

An open-source, distributed cloud storage solution.

This is just a way for me to try Rust by implementing Storj's [whitepaper](https://www.storj.io/storjv3.pdf).

Nuvola is a distributed network of nodes that store fragments - pieces of encrypted files. 
Each node uses a distributed hash table (similar to
[Kademlia](https://en.wikipedia.org/wiki/Kademlia)) to locate fragments of a file
that it had previously uploaded to the network. 
Files are encrypted and split into fragments before being uploaded.
By using erasure codes, nodes avoid long-tail responses during download
operations and provide redundancy. This guarantees better durability in case of
non-responsive nodes and is a more efficient option than replication as it 
requires less bandwidth.

## Components

A Nuvola's node is composed of the following logical components:

### Storage

Fragments are stored directly on the file system, along with some metadata that allows each fragment to be uniquely identified.
Ideally this would be plug-and-play: a user should be able to specify what kind of storage solution to use.

### Metadata

A local db saves information on fragments that have been uploaded to it as well as fragments that have been uploaded by the user.

### Encryption

Regular asymmetric RSA solution to encrypt uploaded files and decrypt downloaded fragments. Again, ideally it would be up to the user's choice.

### Discovery

Nuvola relies on a Kademlia-like network to uniquely identify nodes and perform O(log(n)) lookups.

## Project Structure

This repo follows a [workspace-based](https://stackoverflow.com/a/50402684/3902715)
approach for its structure in order to better separate the actual library
from the executable.
