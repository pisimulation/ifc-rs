# ifc-rs

This is work-in-progress. IFC is not yet implemented.

## Piazza-Like Toy App

Like Piazza, a user is either a student or an instructor. Anyone can make a post. A post is either public to everyone or private to instructors. A student can choose to post anonymously to classmates but not to instructors.

A client and the Piazza server communicate using gRPC. The client (student or instructor) creates a post by calling a `post_msg` RPC and sees the board by calling a `see_board` RPC.

### Build & Run
``$ cargo run --bin client``


``$ cargo run --bin piazza``
