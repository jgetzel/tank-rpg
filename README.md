# Untitled Tank RPG
Top-down competitive multiplayer game made in Rust using Bevy

## Current usage
There are two executables, client.exe and server.exe. 
The client will ask for a server address to connect to upon starting the game.
Starting up the server will display an empty world as well as your local server address. 
Currently, the server defaults to hosting on port `1337`.

Very basic combat is now implemented, you can shoot bullets at other connected players to the server.
However, respawning is not implemented, so after a player despawns, they must restart their client 
and reconnect to play again.