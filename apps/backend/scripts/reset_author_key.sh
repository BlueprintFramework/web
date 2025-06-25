source ./scripts/includes/env.sh

echo "Resetting Key of Author..."

read -p "Name/Id: " name

if [[ $name =~ ^[0-9]+$ ]]; then
	id=$name

	data=$(curl -s -X POST $(APP_URL)/api/__internal/sql \
		-H "Content-Type: application/json" \
		-H "Authorization: $(INTERNAL_KEY)" \
		-d "UPDATE authors SET key = md5(random()::text) WHERE id = $id RETURNING name, key;")

	output=$(echo $data | jq -r '.[0].name')
	output="Name: $output"
else
	name=$(echo $name | sed "s/'/''/g")

	data=$(curl -s -X POST $(APP_URL)/api/__internal/sql \
		-H "Content-Type: application/json" \
		-H "Authorization: $(INTERNAL_KEY)" \
		-d "UPDATE authors SET key = md5(random()::text) WHERE name ilike '$name' RETURNING id, key;")

	output=$(echo $data | jq -r '.[0].id')
	output="ID: $output"
fi

if [[ $(echo $data | jq -r '.[0]') == "null" ]]; then
	echo "Author not found"
	exit 1
fi

key=$(echo $data | jq -r '.[0].key')

echo "Author key reset!"
echo "[$output] Key: $key"