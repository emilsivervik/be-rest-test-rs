#SmartHome Inc.

### Code-test for undisclosed company

## Information about the test
- I was able to create endpoints for creating lightbulbs and schedules
- I've created an interval cycle that lives besides the REST server to check if it should turn on lights, I didn't have enough time to implement turning off the lights.
- Unfortunately I did not implement any unit tests due to short amount of time and I've never implemented any integration tests for MongoDB and almost all functionality is required to have an active connection so if I we're to add unittests I'd probably check if I could extract functionality to testable components/functions.

## Running

### Database
I've used MongoDB as a DB so either an local instance is required or I've provided a docker compose file which could be run with `docker compose up`

### Application
After the database is up and running `cargo run` can  be run to start the application.

## Structure
- `service` service module that containts logic to connecting to the database
- `routes` contains routing logic for the API
- `models`
	- `storage` module which contains a generic trait to be implemented for generic MongoDB operations.
	- `scheduler` module for handling the scheduling cycles (when to turn off and on lightbulbs)
	- `lightbulb` modules for handling storing and transforming data related to lightbulbs
	- `schedule` modules for handling storing and transforming data related to schedules

## Example API requests

### Add lightbulb
```
curl --request POST \
  --url http://127.0.0.1:8000/lightbulb \
  --header 'Content-Type: application/json' \
  --data '{
	"active": "off"
}'
```
### List lightbulbs
```
curl --request GET \
  --url http://127.0.0.1:8000/lightbulb \
  --header 'Content-Type: application/json'
```

### Add schema
We can either specify `lightbulbs` directly or update a schema later if needed, in this example I'll specify the lightbulb returned from the above "Add lightbulb"
```
curl --request POST \
  --url http://127.0.0.1:8000/schedule \
  --header 'Content-Type: application/json' \
  --data '{
	"repeating": "daily",
	"onTime": {
		"hour": 14,
		"minute": 0
	},
	"offTime": {
		"hour": 16,
		"minute": 0
	},
	"lightbulbs": [
		{
			"_id": "643cf43840dc32b6098a6f50",
			"active": "off"
		}
	]
}'
```

### List schedules
```
curl --request GET \
  --url http://127.0.0.1:8000/schedule \
  --header 'Content-Type: application/json'
```

### Update schedule
```
curl --request PUT \
  --url http://127.0.0.1:8000/schedule/643ce4c18c6f5dc3a5c06e64 \
  --header 'Content-Type: application/json' \
  --data '{
	"_id": "643ce4c18c6f5dc3a5c06e64",
	"lightbulbs": [
		{
			"_id": "6438626236678b4db80c8623",
			"active": "off"
		}
	],
	"repeating": "daily",
	"weekDays": null,
	"active": "on",
	"onTime": {
		"hour": 8,
		"minute": 45
	},
	"offTime": {
		"hour": 8,
		"minute": 46
	}
}'
```
