# Commands

### TODOs

- [ ] ASK is not a command and it needs to be refactored
- [ ] ASKING command should be sent to the original node and then the next command should be sent to the originally intended node
    - `GET A` - Original query to the node
    - `-ASK 7865 127.0.0.1:9001` - Server responds with the "ASK" error
    - `ASKING` - Client sends this to the node
    - `GET A` - Client sends GET command to the new node (located at 127.0.0.1:9001)
- [ ] Implement MOVED
    - `MOVED <slot> <ip:port>`
    - Since this is a permanent move message, the client should update it's internal slot-to-node mapping
    - No `ASKING` needed to send the command to the new node
