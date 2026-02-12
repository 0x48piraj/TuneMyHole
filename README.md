# Tune My Hole

A self-tuning Pi-hole companion that automatically builds lean, region-aware blocklists based on real network behavior.

Tune My Hole analyzes historical DNS traffic from Pi-hole's FTL (Faster Than Light) query database, correlates observed domains with known malicious and tracking sources producing a small, high-confidence local blocklist.

This project favors signal over volume. No list hoarding. No guesswork. Just evidence-based blocking.

## What it does

- Analyzes historical DNS queries stored in Pi-hole's long-term FTL query database
- Cross-references observed domains against curated malicious, tracking and unwanted domain sources
- Generates an optimized Pi-hole local blocklist designed to complement existing adlists

Instead of blindly blocking millions of domains, this approach produces a small, auditable rule set based on what your network actually resolves. The result is fewer breakages, faster lookups and blocking decisions you can reason about.

The analysis runs entirely offline. Query data never leaves your Pi-hole and blocking decisions remain predictable and transparent.

### Installation

Tune My Hole is a **Pi-hole companion agent** that installs once and runs automatically.

#### Requirements

* Linux system running **Pi-hole**
* Read access to FTL query database
* `systemd` _(recommended)_ or `cron`

#### Install the easy way

Download the binary matching your system from the _Releases_:

```bash
sudo curl -L https://github.com/0x48piraj/tune-my-hole/releases/download/v0.1.0/tmhole-aarch64-unknown-linux-gnu -o /usr/local/bin/tmhole

sudo chmod +x /usr/local/bin/tmhole
```

> Note: Replace the target with [`tmhole-armv7-unknown-linux-gnueabihf`](https://github.com/0x48piraj/tune-my-hole/releases/download/v0.1.0/tmhole-armv7-unknown-linux-gnueabihf) for 32-bit Raspberry Pi OS.

### Set it up once

Run once:

```bash
sudo tmhole init
```

This will:

* Create `/etc/pihole/tune-my-hole.d/`
* Install a daily systemd timer (or cron fallback)
* Create managed state and output files

Tune My Hole now runs automatically.

### Adding reference blocklists

Tune My Hole consumes one or more Pi-hole compatible domain blocklists.

Reference lists live in:

```text
/etc/pihole/tune-my-hole.d/
```

Example _(using AdguardDNS blocklist)_:

```bash
sudo curl -sSL https://v.firebog.net/hosts/AdguardDNS.txt -o /etc/pihole/tune-my-hole.d/AdguardDNS.txt
```

You can add multiple lists. Tune My Hole will combine them automatically.

Reference lists are **inputs only** and are never modified.

### Usage

#### Manual run (optional)

You normally don't need this, but you can run it manually:

```bash
sudo tmhole run
```

This will:

* Analyze Pi-hole DNS history
* Generate a lean, managed blocklist
* Reload Pi-hole DNS (if enabled)

#### Status (recommended)

Check what **Tune My Hole** has done:

```bash
sudo tmhole status
```

Example output:

```
Tune My Hole
────────────────────────
Managed domains: 4,832
Last run:        2026-02-09T03:00:00Z
Blocklist path:  /etc/pihole/tune-my-hole.list

[!] No reference lists found.
    Drop blocklists into:
    /etc/pihole/tune-my-hole.d/
```

### Uninstall

To completely remove Tune My Hole:

```bash
sudo tmhole uninstall
```

This removes:

* `systemd` timer / `cron` job
* Managed blocklist
* State and config files

User blocklists and Pi-hole data are untouched.

### How it works

Tune My Hole:

1. Reads Pi-hole's FTL query database (read-only)
2. Observes which domains your network actually queries
3. Intersects those domains with trusted reference lists
4. Emits a **small, high-confidence** local blocklist

No list hoarding.
No blind automation.
Just evidence-based blocking.

## Philosophy

**Small tuned lists > big lists.**

Blindly stacking massive third-party blocklists is a common pattern and it mostly fails at what it claims to do. Huge lists consume memory, slow down lookups, increase rule churn and introduce false positives all while blocking large numbers of domains that your network will never resolve.

At the same time, keeping a tiny static list without context is just as ineffective. Blocking should be informed by actual network behavior, not guesswork or list hoarding.

Tune My Hole takes a disgustingly straightforward approach:

- Observe what your network actually queries
- Validate those domains against known bad signals
- Block only what is both relevant *and* high-confidence

The goal is not maximum block count. The goal is **accuracy, performance and privacy**.
