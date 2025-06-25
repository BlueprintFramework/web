source ./scripts/includes/env.sh

echo "Getting pending extensions..."

data=$(curl -s -X POST $(APP_URL)/api/__internal/sql \
	-H "Content-Type: application/json" \
	-H "Authorization: $(INTERNAL_KEY)" \
	-d "SELECT extensions.id, extensions.name, identifier, authors.name as author_name FROM extensions INNER JOIN authors ON extensions.author_id = authors.id WHERE pending = true ORDER BY extensions.id ASC;")

mapfile -t extensions < <(echo $data | jq -c '.[]')

if [[ ${#extensions[@]} -eq 0 ]]; then
	echo "No pending extensions found"
	exit 0
fi

for extension in "${extensions[@]}"; do
	id=$(echo $extension | jq -r '.id')
	name=$(echo $extension | jq -r '.name')
	identifier=$(echo $extension | jq -r '.identifier')
	author=$(echo $extension | jq -r '.author_name')

	echo "$name ($identifier) [ID: $id]"
	echo " by $author"
	echo ""
	echo "Approve? (y/n): "
	read -n 1 answer

	if [[ $answer == "y" || $answer == "Y" ]]; then
		echo "Approving extension..."

		curl -s -X POST $(APP_URL)/api/__internal/sql \
			-H "Content-Type: application/json" \
			-H "Authorization: $(INTERNAL_KEY)" \
			-d "UPDATE extensions SET pending = false WHERE id = $id;" > /dev/null

		echo "Extension approved!"
	else
		echo "Skipping extension..."
	fi
done