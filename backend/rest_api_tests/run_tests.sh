#!/bin/bash

# Выполняем тесты из get.py
pytest -o log_cli=true get.py

# Проверяем статус выполнения pytest get.py
if [ $? -ne 0 ]; then
  echo "Ошибка при выполнении тестов в get.py"
  exit 1
fi

# Выполняем тесты из post.py
pytest -o log_cli=true post.py

# Проверяем статус выполнения pytest post.py
if [ $? -ne 0 ]; then
  echo "Ошибка при выполнении тестов в post.py"
  exit 1
fi

echo "Все тесты успешно выполнены"

