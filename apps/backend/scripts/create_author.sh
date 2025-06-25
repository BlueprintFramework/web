source ./scripts/includes/env.sh

echo "Creating Author..."

read -p "Name: " name

name=$(echo $name | sed "s/'/''/g")
data=$(curl -s -X POST $(APP_URL)/api/__internal/sql \
	-H "Content-Type: application/json" \
	-H "Authorization: $(INTERNAL_KEY)" \
	-d "INSERT INTO authors (name) VALUES ('$name') RETURNING id, key;")

id=$(echo $data | jq -r '.[0].id')
key=$(echo $data | jq -r '.[0].key')

echo "Author created!"
echo "[ID: $id] Key: $key"