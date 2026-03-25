# Arma 3 Mods Parser
*Please note that I use Linux for my server, this means that I don't know how the program will operate on Windows based servers. I also will be using moderate to advanced terminology, generally if you're self-hosting you'll know this, if not I recommend you first search a thesaurus to see if you may already know something by another name. Also, if you're using a hosting provider you'll likely not need this program in the first place.*

## How to Use

The way I have the program set up currently you will need a `mods` directory under the server root, in the mods directory will lie the `mods.html` you can export from the launcher.<sup>[[1]](#things-to-note)</sup> After setting this up running the program will print to the terminal the `-mod=` flag with all your mods parsed and ready to be passed directly to the program. I pipe that output into a file (`modlist.txt`) and then append that file's contents to the launch command. There's no reason to do this unless you forsee bugfixing in your future, which most people self-hosting servers should. I digress, though my explanation is complete at this point.

## Things to Note
1. The vanilla launcher works on my system, I don't know how the [arma3-unix-launcher](https://github.com/muttleyxd/arma3-unix-launcher) exports mods lists. I assume it's designed to be compatible with the vanilla launcher, however I would not know.
2. Though it's not yet a feature I may make adjustments to automatically download mods from the modlist via Steam, given a public API is available to do so, it all depends on how fed up I get with transferring mod files from my desktop to my server.

