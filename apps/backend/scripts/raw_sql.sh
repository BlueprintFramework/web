source ./scripts/includes/env.sh

echo "Running Raw SQL..."
echo "Type 'exit' to stop"

while true; do
	read sql

	if [[ $sql == "exit" ]]; then
		break
	fi

	echo "Running SQL..."

	data=$(curl -s -X POST $(APP_URL)/api/__internal/sql \
		-H "Content-Type: application/json" \
		-H "Authorization: $(INTERNAL_KEY)" \
		-d "$sql")

	echo $data | jq
done