.PHONY: explicit_phony

explicit_phony:

day%: explicit_phony
	cd $@; cat ../input/$@.txt | cargo run

