# `rasm++`: The Rust rewrite of rasm

This project aims to rewrite the rasm assembler in Rust but with additional features to make it a proper language. Planned features include:
- [ ] Plugging constants directly instead of being forced to use the constants section and using `const[]`
- [ ] Changes in how local, global, and const registers are indexed ([] instead of ())
- [ ] Changes in instruction names to make reading easier (ex. stack_push instead of STACKPUSH)
- [ ] Explicit bytecode version option for programmers

Right now we're looking for contributors to help finish the rewrite and implement these features! Join the discord server if you're interested!: https://discord.gg/KQvBbJk8h6

How to run (as of January 2023):
* Step 1. `git clone https://github.com/Resurgence-VM-Development/rasm`
* Step 2 (needed now until Resurgence enters its first alpha). `git clone https://github.com/Resurgence-VM-Development/Resurgence`
	* Step 2.5. If the most recent commit doesn't build, reset Resurgence to the last commit that does (in the commit history, it should have a green checkmark, a red X means it didn't build) using `git reset --hard <commit-that-builds>`. 
* Step 3. Edit this line in `Cargo.toml`: `resurgence = { path = "path/to/Resurgence" }`
And that's it! You can start coding afterwards or build Resurgence
