build: ../build/solution-check ../build/solution-image
.PHONY: build

../build/solution-check: main.js lambda.js
	npm install
	xshar --output=../build/solution-check \
		--command='nodejs $${XSHAR_ROOT}'"$$(pwd)"'/main.js' *.js node_modules

../build/solution-image: canvas.js lambda.js
	npm install
	xshar --output=../build/solution-image \
		--command='nodejs $${XSHAR_ROOT}'"$$(pwd)"'/canvas.js' *.js node_modules
