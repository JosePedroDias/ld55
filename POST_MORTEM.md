# Post Mortem on Merge or Die

## intro

It's been a while since my last public game hackathon.
Checking on my Github account, last entries were
August '22 with my [Oily](https://github.com/JosePedroDias/oily) entry to [Multiplayer Jam](https://itch.io/jam/multiplayer-jam) and my last [Ludum Dare 41](https://ldjam.com/events/ludum-dare/41/) entry was [Raceman](https://github.com/JosePedroDias/ld41-raceman) on April '18!

After an unfruitful brainstorm on the selected theme (summoning), momentarily diverging from [Comfy](https://comfyengine.org/) to other toolsets such as [Fyrox](https://fyrox.rs/) and almost giving up on the whole thing, I went for a casual merging game, [Merge or Die](https://ldjam.com/events/ludum-dare/55/merge-or-die).

## preparation

[LD55](https://ldjam.com/events/ludum-dare/55) came to my attention days before the compo began. It wasn't to happen on a convenient weekend for me but I peeked at the last 10 voted themes anyway, trying to come up with my take on each of them.

I've been learning [Rust](https://www.rust-lang.org/) on the side, trying to use it for everyday tasks and ad hoc projects, so I gave it a go for the hackathon, after most of the past entries being in my bread and butter Javascript language, which pays for the bills.
I opted for the Comfy engine, the [löve](https://www.love2d.org/) counterpart for Rust. It's simple 2D, not too demanding in Rust's feature set and support publishing for web/wasm. To get acquainted to it, I coded a [simple minesweeper game](https://github.com/JosePedroDias/minesweeper) the week before.

## day 1

The theme 'summoning' wasn't inspiring to me. The elected theme is oftentimes the one I like the least. I followed the ldjam feed and briefly visited twitch streams to check what other participants were planning to do, trying to pick up some inspiration from others but it was fruitless. 90 minutes in I went to bed, not sure whether I would take part on this edition of the competition.

I woke up with vague ideas on dice rolling battles like [Slice & Dice](https://store.steampowered.com/app/1775490/Slice__Dice/) or a shared 2D RPG world built in cohop fashion but these were too big in scope and I wasn't confident I would be able to pull them off. I did a test drive on [Fyrox](https://fyrox.rs/), to maybe go with a more RAD 3D approach. It was promising but it would be too much of a gamble. Spent a couple hours discussing potential casual games instead, which would be a better fit for the Comfy engine and the preparatory work I had done.

After back and forth of mechanics with friends, decided to go for a merging game with a couple stress-inducing mechanics. Time would be a factor, as would be incorrectly picking unmatched pairs. I started to work on this final concept about 14 hours after the compo had begun.

Since I'd been discussing my ideas with friends and had watched a couple of streams from others, I thought, "why don't I stream my own progress too?" And so, out of the blue, installed OBS, configured my Twitch account and started my own stream. It may have been a bit boring to watch, with slow progress, no music or engagement on my part and a relaxed set of goals.
I believe it happened from 11pm to 4pm, give or take. The very first session wasn't recorded due to misconfiguration on my part.
This is [my twitch channel](https://www.twitch.tv/jose_pedro_dias).
I streamed 2 other subsequent sessions and those can be seen for some days still.
I thought I would hate to be streaming but the fact that I had to explain my actions out loud the whole time helped me think and focused.

## day 2

I woke up after crashing out for some hours (4h 20min, according to my smart watch). In retrospect, I should've forced myself to rest more but I was eager to resume my work.

I have kept a clear abstract implementation of the game ([state.rs](https://github.com/JosePedroDias/ld55/blob/main/src/state.rs)) and the comfy setup for the game and resource loading ([main.rs](https://github.com/JosePedroDias/ld55/blob/main/src/main.rs)). Rust was super helpful. Types caught most of the things. I used [Lapce](https://lapce.dev/) as editor (should have used [Zed](https://zed.dev/), forgot about that one, having a much better experience now with it). As the game progressed things start to fall into place. I was able to move from a single level to multiple ones, testing it out every couple mins.

At some point I moved out of the minesweeper sprites I began with to some scribbles I did on [Inkscape](https://inkscape.org/). I introduced some audio cues as well, using [a simple sfx generator](https://www.leshylabs.com/apps/sfMaker/). All the few assets can be checked [here](https://github.com/JosePedroDias/ld55/tree/main/assets).

I had some fun tweaking the level difficulty. There are 4, with the last one being properly crazy hard. Technically doable, but with the bar timer penalty kicking in every 30 seconds, it's probably the game wipes out the merged numbers that took you all those frenetic clicks to obtain. I anticipate the use of profanity.

Got derailed by a couple small issues with Comfy and [trunk](https://trunkrs.dev/).
I expected the `delta` context value to be time in seconds since last update/render but while that's true for native, in the browser I was getting different values and hacked constant 1/60, which I believe is not working well.
Had one of my browsers zoomed in and it took me a bit to figure out the fonts being scaled while the canvas was not was due to the zoom...
I kept testing with the mouse and wasn't aware comfy with trunk does not support touch events, so the game doesn't work on mobile.
I also missed the fact that trunk by default adds sub-resource integrity parameters which completely broke github and ldjam hosts. Now I can see the docs on the matter, ([1](https://trunkrs.dev/assets/#sub-resource-integrity-sri), [2](https://github.com/trunk-rs/trunk/blob/main/Trunk.toml)) but I manually edited the generated HTML page to remove those as I was publishing the compo. Next time, remember to test touch events and publishing to https domains!

## conclusions

Different people take different things from these experiences. I like to design game mechanics and build games. I also like the added challenge of addressing the theme. The hackathons I enjoyed the most were the ones I had people to exchange ideas and divide tasks with.

This time I had to do it solo and adopted a set of tools I wasn't too experienced with. I should have prepared for animations and spending a couple hours sprinkling it with eyecandy. Ended up aiming for small scope but was able to complete it. I believe it was a decent compromise.
