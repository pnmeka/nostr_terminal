# nostr_terminal
Publish notes from terminal

1. Introduction to Nostr and its Importance

Nostr is a decentralized, open-source protocol for secure and censorship-resistant communication. It allows users to create and share content, such as text, images, and videos, without relying on centralized platforms. The importance of Nostr lies in its ability to provide a free and open network for information exchange, resistant to censorship and control. This makes it an attractive option for individuals and communities seeking to maintain their online freedom and autonomy.
2. Structure of Nostr: Client and Relay

The Nostr protocol consists of two primary components: clients and relays. Clients are applications that allow users to create, send, and receive Nostr events, which are the basic units of content in the Nostr network. These events are encrypted and signed, ensuring the authenticity and integrity of the content. Relays, on the other hand, are servers that store and forward Nostr events, allowing clients to discover and retrieve content from the network. Relays can be run by anyone, and they can be configured to filter or moderate content according to their own policies. This decentralized architecture enables Nostr to operate without a single point of control or failure, making it a robust and resilient platform for online communication.


Example Usage

To use the following Nostr executable, you can print a message using the following command:

    ./nostr_terminal "post hello nostr"

This command will create a new Nostr event with the text "hello nostr" and send it to the Nostr network. The event will then be relayed by one or more relays, making it available to other users on the network.

To build use the Nostr executable, simply compile the rust code and run the resulting binary with the desired command-line arguments. For example:

    ./nostr_terminal "post hello world"

# Get Started


1. Git clone the repo

        git clone https://github.com/pnmeka/nostr_terminal
2. Navigate to the directory

       cd nostr
3. Change the nsec in src/main at line 124. I have a general nsec over there. Compile to build binary
   
       cargo run
4. navigate to directory and try the binary

       cd target/debug

       ./nostr_terminal "Hello from Terminal"

    
