A minimal reproduction of Steam causing an error when calling [NonRoamableId](https://learn.microsoft.com/en-us/uwp/api/windows.gaming.input.rawgamecontroller.nonroamableid?view=winrt-22621) when a controller is added through [Steam Remote Play Together](https://store.steampowered.com/remoteplay).

## How to test
You will need a Steam game that supports Steam Remote Play Together and a second PC with a Steam account. You can find games here: https://store.steampowered.com/remoteplay

1. Install Rust https://rustup.rs/
2. Download this repo
   ```
   git clone https://github.com/paul-hansen/steam_remote_play_bug_repro.git
   ```
3. From the project's root directory, compile the program
   ```
   cargo build
   ```
4. Copy the executable found in the project folder at `.\target\debug\steam_remote_play_bug_repro.exe`
5. Browse to the local files of a game you have that supports [Steam Remote Play Together ](https://store.steampowered.com/remoteplay)
   and replace the game's executable file with the executable file you generated.
   Make sure the name is the same as the game's original executable.
6. Launch that game through Steam
7. It will now launch a window, make sure you can see the console that spawns behind it.
8. From your steam friend's list, invite a friend for a remote play together session.
9. Have your friend join, in my experience it will take a couple tries (happens with any game).
10. When your friend joins you should see a log in the console starting with `Found controller:`
11. Observe that your friend's controller will have an error for the controller ID. This is what is unexpected and ideally would be fixed.