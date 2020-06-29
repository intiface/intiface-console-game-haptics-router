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
controls rerouting front, which made it so I could go from zero to this in a few
hours:

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

| Console | Protocol |
|---|---|
| PS3 | BT2.1 EDR (HID Profile) |
| PS4 | BT2.1 EDR + Pairing (HID Profile) |
| Switch | BT3 HS (HID Profile) |
| XBox (Old) | WiFi Direct (?) |
| XBox (New) | BT4 (HID Profile) |

Let's take a quick stroll through what these mean:

- BT 2.1 EDR
  - Bluetooth "Classic" with Enhanced Data Rate, capable of up to 3Mb/s
    transfer. The usual rate, known as "Basic Rate", is 1Mb/s.
- BT3 HS
  - Bluetooth 3 with High Speed, capable of up to 24Mb/s
- BT4
  - Bluetooth LE, probably the most widely known modern version of Bluetooth.
    Max data rate of 1Mb/s.
- WiFi Direct
  - Basically good ol' WiFi with some special pairing and no routing, so it's
    just direct contact between controller and console.

For BT4 (which only newer xbox controllers support), we can use tap hardware
such as a [Adafruit Sniffer](https://www.adafruit.com/product/2269), [Ubertooth
One](https://greatscottgadgets.com/ubertoothone/), or my personal favorite,
[Sniffle](https://github.com/nccgroup/Sniffle), to track the data going between
the controller and console. This makes life super easy, because it's a passive
way of doing things. Other than making sure we are tracking the controller and
console connection from the time it's established, it's pretty easy to monitor.

Also, it's worth noting here that most bluetooth sex toys are BT4/LE. They don't
require much bandwidth, and also the people that are making them are usually
interested in phone compatibility. Non-BT4 devices may have problems with compat
on iOS (file a bug and correct me if I'm wrong here).

BT2/3, on the other hand, don't have much in the way of cheap tap hardware. Due
to bandwidth capabilities and lack of need to tap the protocol very often, most
hardware that can sniff BT2.1 EDR or BT3 HS starts in the $1000s. That's not
going to be very scalable for people who just want their consoles to control
their buttplugs.

Instead of going the monitoring route there, we run a hardware MITM via an L2CAP
proxy to get the information, then use BT. This requires:

- A Linux box with Bluez. A RPi will do for this.
- 2 bluetooth radios.

On one radio, we'll run the L2CAP proxy, which is what a majority of the code in
this project is. We do whatever mating dance is required to hook up the
controller and the console to our tap, then sit there and watch the data as it
goes back and forth over the proxy.

Then, Buttplug handles talking to any toys that are either in the vicinity (for
BT4 toys, which is what we use the second radio for) or hooked up to the tap
device (for serial/USB toys). 

With all of that in place, we can do things like:

- Translate rumble commands to toy commands
- Trigger toys on button or direction presses

Fun!

## FAQ

### What's the end product for this look like?

Ideally, I'd like to have a recommended setup, which will probably just be a
special RPi image distributed on this repo, plus places to buy the required
hardware.

The RPi image would host a web app that allows users to do toy scanning, set up
reaction bindings (rumble -> toy, button press -> toy, and at some point,
command injection, i.e. kegel sensor -> button press).

Obviously, we're a ways off from that right now.

### When will there be PS4/XBox Support?

PS4 is tricky. The same L2CAP proxy idea works, but the controller requires
constant key exchanges with the console, which means the initial connection and
tapping scenario is more complicated. If you really want to get into the weeds
on this, [the GIMX wiki](https://gimx.fr/wiki/index.php?title=Main_Page) is
chock full of good information.

I'd rather not have to go the GIMX route of putting a hardware tap on the USB
line to the console to monitor key exchange, but I may not have much of a
choice. If that's the case, [I'll probably just point people at GIMX's cable for
that](https://blog.gimx.fr/product/gimx-adapter/) unless you want to [build your
own](https://gimx.fr/wiki/index.php?title=DIY_USB_adapter_for_dummies, since
it's just a Teensy, a UART adapter, and a few wires.

For XBox, holdup there is that I need to get one of their Bluetooth capable
gamepads. For WiFi direct, I've got loads of WiFi direct controllers, but just
haven't had a chance to see what the proxy situation is there yet.

### Will this ever work with stroking hardware like the Fleshlight Launch or The Handy?

This has been a request on the original GHR for quite a while, and it's
something I'd like to implement for both. Mainly just a matter of time, and
figuring out exactly what that interaction would look like.

### Why does this require 2 radios?

Bluetooth radios saturate quickly, so managing the L2CAP Proxy is about all one
radio is going to handle. Also, due to Bluez weirdness, the bluetooth service
has to be reset on controller connection, which can confuse Buttplug. It's just
easiest to split the jobs between two radios.

### How are you translating Switch Haptics to Toys?

Right now, in the dumbest way possible. If we see a live haptics packet (HID
report byte 3 is greater than 0), we just set the toy to full speed vibration.
That's it. It's so dumb.

Actually translating Switch Haptics to toys is going to be a chore, because
Switch haptics uses "HD Rumble". This means instead of just saying "set motor at
speed" (1 dimension), it basically sends audio waveform information, so that it
will play vibrations at a certain frequency and amplitude (2 dimensions). We
need to figure out a nice way of translating the Freq/Amp back to displacement
motor speeds, accounting for spinup/down time and all that. It's not impossible,
but it's more time than this weekend project had.

### Controller input seems to lag

Yes, yes it does. No, I don't know why. Definitely something that needs to be
fixed.

### What else would you like to fix?

- Switch autoconnect should work after we know the system MAC on first connect.
- Would really like to figure out why the connection keeps flaking out after 2-3
  min.
- Initial connection reliability.

I may also try the original joycontrol to see how that works out and if it's any
more stable. There's a [Python Buttplug
Client](https://github.com/buttplugio/buttplug-py), so that part isn't a problem.

### Why is this GPL3?

Because joycontrolrs was. I'd expect the whole repo to get a rewrite and be
under our normal BSD 3-Clause license at some point.