curl -s --location --request POST 'localhost:3000/customers' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
        "guid": "11",
        "first_name": "first name",
        "last_name": "last name",
        "email":"test@email.com",
        "address":"4 Loomis Drive"
}'

echo 

curl -s --location --request GET 'localhost:3000/customers' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' | jq '. | length'

echo 

curl --location --request PUT 'localhost:3000/customers/1' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \
--data-raw '{
        "guid":"1",
        "first_name": "updated name",
         "last_name": "Bottrell",
  "email": "dbottrell0@furl.net",
  "address": "020 Evergreen Parkway"

}'

echo 

curl -s --location --request GET 'localhost:3000/customers' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' | jq '.[0]'

echo 

curl --location --request DELETE 'localhost:3000/customers/2' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' \

echo 

curl -s --location --request GET 'localhost:3000/customers' \
--header 'Content-Type: application/json' \
--header 'Content-Type: text/plain' | jq '. | length'

echo 
