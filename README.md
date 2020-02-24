# ifc-rs

This is work-in-progress. IFC is not yet implemented.

## Piazza-Like Toy App

Like Piazza, a user is either a student or an instructor. Anyone can make a post. A post is either public to everyone or private to instructors. A student can make a public post but remain anonymous to classmates but not to instructors.

A client and the Piazza server communicate using gRPC. The client (student or instructor) creates a post by calling a `post_msg` RPC and sees the board by calling a `see_board` RPC.

### Build & Run

The Piazza server is configured to run on `localhost:10001`.


``$ cargo run --bin piazza``


``$ cargo run --bin client``
