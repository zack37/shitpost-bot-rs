SERVICE='shitpost-bot-rs'

echo "version is $VERSION"

if [ -z $VERSION ]; then
  echo "Must provide a version in the form of an environment variable: \$VERSION"
  exit 1
fi

docker build -t $SERVICE:$VERSION -t $SERVICE:latest .

docker service create --name $SERVICE --secret source=discord_token,target=discord_token -e DISCORD_TOKEN_FILE="/run/secrets/discord_token" -e RUST_LOG=info $SERVICE:$VERSION
