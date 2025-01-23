#
# Only edit the source copy in /source/common/Makefile.common!!!
#
.PHONY: default
default:
	@which just > /dev/null || cargo install just
	@echo
	@echo "Use 'just <recipe>' to run commands"
	@echo
	@just

*: default
