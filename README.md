# multithreaded-webserver
Multithreaded web server made with Rust. Uses a thread pool to handle multiple requests while avoiding DOS attacks.

To view the proof of concept, clone repository, then run `cargo run` while in the directory of the project.

The current project demonstrates its cleanup with after 2 requests, but you can change this by going to line `11` in main and passing an argument to `.take()` to signify the number of requests you want the server to take before the server cleans up and shuts donw. 

*Please check **production** branch for polished version of project*

## Images

<img width="1728" alt="image" src="https://user-images.githubusercontent.com/71617542/189497555-a5f7a567-d071-44d6-9f65-4c7b6aead3fd.png">



