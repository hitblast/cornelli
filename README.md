# 🔮 write to your future self <3

*cornelli is your small terminal app for the times when you really desire to give your future self pats for the work you do; or for expressing any thoughts to them.*

### Universal install

```bash
cargo install nelli
```

### Using Homebrew

```bash
brew install hitblast/tap/cornelli
```

### Pre-baked releases

Get binaries for your platform [here](https://github.com/hitblast/cornelli/releases).

<br>

## 🪄 Usage

Start by writing a letter to yourself:

```bash
# unlocks after 20 days, 1 hour, 2 minutes and 1 second
nelli keep "Miss the board games with friends?" -t 20d1h2m1s
```

You'll be asked to enter a "password". This is what you'll use periodically to enter and access cornelli, so set something that you can remember.

As time passes, let this command surprise you once in a while if you feel like it:

```bash
nelli mailbox
```

Hopefully you'll enjoy what comes next! :3

If you do wish to, however, make all your lost messages lost "forever", try:

```bash
nelli burn
```

<br>

## ☘️ Compiling

```bash
cargo build --release --quiet --locked
```

# 🪄 Why

This is a Christmas project - a friend and I decided to spin up a friendly hackathon for building a time capsule of memories. And besides, what better way to set the mood for this day than creating something fun with the tools you have?

## License

Licensed under the [MIT License](./LICENSE).
