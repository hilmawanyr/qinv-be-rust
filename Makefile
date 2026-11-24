# Penggunaan:
#   make mod:auth                            -> src/modules/auth/mod.rs
#   make mod:auth handler                    -> src/modules/auth/handler/mod.rs
#   make file:user mod:auth                  -> src/modules/auth/user.rs
#   make file:token mod:auth:handler         -> src/modules/auth/handler/token.rs
#   make file:auth mod:auth adapter:handler  -> src/modules/auth/adapter/handler/auth.rs
#
# Target ber-":" tidak bisa ditulis sebagai target Make biasa, dan kata-kata
# setelahnya dibaca Make sebagai target tambahan. Karena itu semua goal
# ditangkap oleh .DEFAULT: goal pertama (mod:* / file:*) menjalankan mod.sh
# dengan seluruh goal sebagai argumen, goal berikutnya diabaikan.

SCRIPT := bash ./mod.sh
FIRST  := $(firstword $(MAKECMDGOALS))

.DEFAULT_GOAL := _usage

.PHONY: _usage
_usage:
	@echo "make mod:<name>                                create src/modules/<name>/mod.rs"
	@echo "make mod:<name> <sub>                          create src/modules/<name>/<sub>/mod.rs"
	@echo "make file:<file> mod:<name>                    create src/modules/<name>/<file>.rs"
	@echo "make file:<file> mod:<name>:<sub>              create src/modules/<name>/<sub>/<file>.rs"
	@echo "make file:<file> mod:<name> adapter:<folder>   create src/modules/<name>/adapter/<folder>/<file>.rs"

.DEFAULT:
	@$(if $(filter mod:% file:%,$(FIRST)),$(if $(filter $@,$(FIRST)),$(SCRIPT) $(MAKECMDGOALS),:),echo "error: unknown target '$@'" >&2; exit 1)
