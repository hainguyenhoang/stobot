# STOBot
A Discord bot that fetches Star Trek Online news as they come out.
## Build instructions
1. Install Rust: https://www.rust-lang.org/learn/get-started
2. Install other dependencies
   * On Debian/Ubuntu: `sudo apt install gcc libc6-dev`
   * On Fedora: `sudo dnf install gcc`
3. Run `cargo install --path .`
## Run instructions
1. Set the `DISCORD_TOKEN` environment variable to your Discord bot's token
2. Run `stobot`
   * Run with `--help` to see available arguments
3. In your desired channel, type this: `!stobot`
   * The bot should respond to this, and then you'll receive future news in that channel.
   * To stop the bot posting there, type `!unstobot`
