#!/bin/bash

for counter in $(seq 1 255); do
	curl "http://localhost:8000/api/users/danielhe4rt";
	curl "http://localhost:8000/api/users/danielhe4rts$counter";
	echo "$counter";
done
