# Intiface Console Game Haptics Router

[![Patreon donate button](https://img.shields.io/badge/patreon-donate-yellow.svg)](https://www.patreon.com/qdot)
[![Github donate button](https://img.shields.io/badge/github-donate-ff69b4.svg)](https://www.github.com/sponsors/qdot)
[![Discourse Forum](https://img.shields.io/badge/discourse-forum-blue.svg)](https://metafetish.club)
[![Discord](https://img.shields.io/discord/353303527587708932.svg?logo=discord)](https://discord.buttplug.io)
[![Twitter](https://img.shields.io/twitter/follow/buttplugio.svg?style=social&logo=twitter)](https://twitter.com/buttplugio)

Have you ever been like "Gosh I wish I could use the [Intiface Game Haptics
Router](https://intiface.com/ghr) with a game console"? 

Or maybe you just saw someone tweeting a sex toy working with Animal Crossing:
New Horizons and were wondering what the fuck was going on?

Well, here's your answer.

## Before we get started

What you see in this repo right now is the barest of Proofs of Concept. I forked
someone's quick joycon relay project, added a few lines for Buttplug, and that's
it. Right now, the system is horribly unreliable, connecting maybe 1 in every
3-4 tries, and control lag is super noticable.

There are definitely plans to improve on this, but I also hate posting gifs
without PoCs, so here we are.

This project is Switch only right now just because that's what's on my desk and
easiest to work with. PS4/Xbox is coming at some point. See the FAQ for
specifics.

I'd like to thank the following projects for everything they've done on the
controls rerouting front, which made it so I could go from zero to this in under
8 hours:

- [GIMX](https://blog.gimx.fr/)
  - A really neat open source project for rerouting PC controls to PS4/XBox
- [joycontrol](https://github.com/mart1nro/joycontrol/)
  - Python joycon emulation
- [joycontrolrs](https://github.com/juanpotato/joycontrolrs/)
  - Rust port of joycontrol, what this PoC is based on
- [joycon-rs](https://github.com/KaiseiYokoyama/joycon-rs)
  - Referenced this a few times, will be using it in buttplug for joycon control
    support
- [dekuNukem's Reverse Engineering Work](https://github.com/dekuNukem/Nintendo_Switch_Reverse_Engineering)
  - The source for HID packet breakdowns.

## What is the Game Haptics Router?

The Game Haptics Router (GHR) is a set of utilities built on top of the
[Buttplug Intimate Hardware Control Library](https://buttplug.io). It works as a
tap in between games and controllers, rerouting rumble and control information
in order to also control sex toys.

The [original GHR](https://intiface.com/ghr) is a Windows application that hooks
calls to either XInput or Unity's VR functions, passing the commands to the
controllers as well as translating them to any vibrating sex toys supported by
the Buttplug library and currently connected to the system.

For examples of how the GHR works, check out the [Will It Buttplug episodes of
Buttpluggin' With
qDot](https://www.youtube.com/playlist?list=PLDZBOOe-bdwMPbogC_A4VfdFDXi75dXV-),
which shows how different games work with the original GHR!

## What is the Console Game Haptics Router?

One of the main goals of the GHR is to work with stock games and hardware as
much as possible. This allows us to distribute the software to the largest
audience, in the hopes it will "just work" for everyone (which it never does).
As the GHR is a side project of the [lead Buttplug
developer](http://twitter.com/qdot), requiring  complicated mods to games would
take up too much support time, leaving less time for library development.

However, many people would like to be able to have GHR functionality with game
consoles instead of PCs. We can't replicate the GHR application on consoles, as
that would require jailbreaking/modding (as well as us having to understand
homebrew toolchains, which is a job in itself), which goes against the core
ideal of the GHR. Instead, we've created (or, more appropriately, copypasta'd) a
way to tap the connection between the controller and the console. From there, we
can do the same rerouting we do in the GHR app.

## How does the Console GHR work?

__Beware: Technical Info Ahead__

Here's a super quick and dumb chart of how different controllers talk to their
consoles:

|---|---|
| Console | Protocol |
|---|---|
| PS3 | BT2.1 EDR |
| PS4 | BT2.1 EDR + Pairing |
| Switch | BT3 HS |
| XBox (Old) | WiFi Direct |
| XBox (New) | BT4 |

## FAQ

### Will this ever work with stroking hardware like the Fleshlight Launch or The Handy?