test:
	cargo test

commit: test
	git add -i
	git commit

push: commit
	git push
