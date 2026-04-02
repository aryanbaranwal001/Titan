.PHONY: strip

strip:
	grep -rl --include="*.rs" '##\|\[dev\]' crates/ cli/ \
	| xargs sed -i \
	  -e '/^[[:space:]]*\/\/[[:space:]]*##/d' \
	  -e '/\[dev\]/d' \
	  -e 's/[[:space:]]*\/\/[[:space:]]*##.*$$//'
