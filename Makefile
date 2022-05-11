run:
	python bot.py
beauty:
	isort .
	black .
	flake8 .  --exit-zero
	autoflake --remove-all-unused-imports --remove-unused-variables --in-place -r .
