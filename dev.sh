# Quick development script to run the daemon and client for testing
cargo build --release
./target/release/taskr kill
./target/release/taskr-daemon
./target/release/taskr "echo hello" 2

# // TODO: Move this to the install.sh where you need to pass an env variable with configuration
# Make sure you export TASKR_DISCORD_WEBHOOK=YOUR_URL
if [ -z "$TASKR_DISCORD_WEBHOOK" ]
then
    if [ -f "discord.txt" ]
    then
        TASKR_DISCORD_WEBHOOK=$(cat discord.txt)
    else
        echo "TASKR_DISCORD_WEBHOOK is not set and discord.txt does not exist"
        exit 1
    fi
fi

echo $TASKR_DISCORD_WEBHOOK > /tmp/taskr/discord_webhook
