NAME := computor

debug:
	@cd computor_v1 && cargo build
	@cp computor_v1/target/release/computor_v1 ./$(NAME)

release:
	@cd computor_v1 && cargo build --release
	@cp computor_v1/target/release/computor_v1 ./$(NAME)

$(NAME): release

all: release

clean:
	@cd computor_v1 && cargo clean

fclean: clean
	@rm ./computor

re: fclean all

.PHONY: all clean fclean re release debug
