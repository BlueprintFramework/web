if [ ! -f .env ]; then
	echo "Missing .env file"
	exit 1
fi

INTERNAL_KEY() {
	local env=$(cat .env)
	local key=$(echo $env | grep -oP 'INTERNAL_KEY="\K[^"]+')

	printf $key
}

APP_URL() {
	local env=$(cat .env)
	local url=$(echo $env | grep -oP 'APP_URL="\K[^"]+')

	printf $url
}

if ! command -v jq &> /dev/null; then
	echo "jq is required to parse JSON data"
	exit 1
fi