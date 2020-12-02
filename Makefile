.PHONY : test
test:
	cargo test

.PHONY : commit
commit: test
	git add -i
	git commit

.PHONY : push
push: commit
	git push
