
> In addition to this, search for `//TODO` in the source.

# Fixes

- [ ] Bug: Laucher watchdog currently don't know to restart `main_worker` if `main_worker` was killed.

# Optimizing
- [ ] The so called "Linux" OS should actually be called "Ubuntu".
- [ ] In gathering info: Use direct buff file read instead of calling command for faster speed.
- [ ] For Linux have to be care full with cmd to avoid bare bone server.
- [ ] Please note on Window 10, Window 8 and Window XP as corporation may still use those version.
- [ ] If agent have a fatal error, it should try to send distress signal to server.
- [ ] For USB (in `disk` topic) and other peripheral, may need socket listener to catch them.
- [ ] Core OS specific info gathering: <span style="color:rgb(255, 0, 0)">The cache logic need to change, currently it just read all. The refesh need to be specific each time</span>. This is currently skip for mockup solution! It should be zero cache at scan, just cache at payload making.
- [ ] May need an alternative solution for save the config when shut down, as that is the easiest time to get corrupted file.
- [ ] Sender logic: For server command that need immediate comply from Agent, may be better to keep on holding the first HTTPS connection rather than keep re initializing.
- [ ] Scheduler design: Min-Heap (currently) vs TimerWheel (array slot) vs Delta queue, which one is better?
- [ ] Might have to change all the binary name to `gsoft-...`?
- [ ] Make `main_worker` able to known other binary version
- [ ] Sender: The hardcoded URLs should be a build-time secret injection rather than a literals.

# New feature

- [ ] Update upstream for Windows (parallel with Linux)
- [ ] Update upstream for MacOS?
- [ ] Proxy scanner binary
- [ ] Promotion Agent to Proxy scanner logic