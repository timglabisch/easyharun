build_test_container:
	cd easyharun_test_container && docker build -t easyharun_test_container --no-cache .

run_test_container: build_test_container
	docker run --rm --name run_test_container -e "SERVER_NAME=foo" easyharun_test_container