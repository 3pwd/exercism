MAKEFLAGS += --no-print-directory

DEV_BIN_DIR := .bin
CYAN := \033[36m
RED := \033[31m
RESET := \033[0m
PATH := $(DEV_BIN_DIR):$(PATH)
USAGE := Usage: make [task] [language=bash|rust] [exercise=<exercise_name>]
INVALID_LANGUAGE := Invalid language. Use 'bash' or 'rust'.
MISSING_LANGUAGE := Language must be specified when providing an exercise.
INVALID_EXERCISE := Exercise not found. Have you downloaded it?

define err
	$(error $(shell printf "%b\n%s\n" "$(RED)Error: $(1)$(RESET)" "$(USAGE)" >&2))
endef

define check_exercise
	$(if $(wildcard $(1)/$(2)),,$(call err, $(INVALID_EXERCISE)))
endef

define check_language
	$(if $(filter-out bash rust,$(language)),$(call err, $(INVALID_LANGUAGE)))
endef

define download
	exercism download --exercise=$(1) --track=$(2)
endef

define test-bash
	cd bash/$(1) && bats -r . --jobs 4
endef

define tests-bash
	find bash -mindepth 1 -maxdepth 1 -type d -exec bash -c 'cd "{}" && bats -r . --jobs 4' \;
endef

define test-rust
	cargo nextest run -p $(1) --cargo-quiet
endef

define tests-rust
	cargo nextest run --all --cargo-quiet
endef


define submit-rust
	@exercism submit $(shell find rust/$(1)/src -name '*.rs') rust/$(1)/Cargo.toml
endef

define submit-bash
	@exercism submit $(shell find bash/$(1) -name '*.sh' ! -name 'bats-extra.sh')
endef

.PHONY: help download-bash download-rust test submit

help: ## Display this help message (default task)
	@printf "%b\n" "Usage: make [$(CYAN)task$(RESET)]"
	@printf "%s\n" "Available tasks:"
	@awk 'BEGIN {FS = ":.*?## "}; \
		/^[a-zA-Z_-]+:.*?## / { \
			task = $$1; \
			desc = $$2; \
			split(desc, arr, " \\| "); \
			printf "  $(CYAN)%-20s$(RESET)%s\n", task, arr[1]; \
			if (length(arr) > 1) { \
				printf "%22s%s\n", "", arr[2]; \
			} \
		}' $(MAKEFILE_LIST)

download: ## language=bash|rust exercise=<exercise_name> | Download an exercise
	$(call check_language)
	$(call check_exercise)
	$(call download,$(exercise),$(language))

test: ## [language=bash|rust] [exercise=<exercise_name>] | Run tests
	@$(if $(language),\
		$(call check_language)\
		$(if $(exercise),\
			$(call check_exercise,$(language),$(exercise))\
			$(call test-$(language),$(exercise)),\
		$(call tests-$(language))),\
	$(call tests-bash) && $(call tests-rust))


submit: ## language=bash|rust exercise=<exercise_name> | Submit an exercise
	$(call check_language)
	$(call check_exercise,$(language),$(exercise))
	$(call submit-$(language),$(exercise))
